//! Table-driven field codec.
//!
//! Every IR-200 parameter is described by one generated [`P`] const in
//! [`crate::map`] (offset, wire encoding, display offset, range). The typed
//! models never carry offsets or ranges of their own: a field names its `P` and
//! the [`block!`] / [`area!`] macros wire `from_bytes` / `to_bytes` through
//! [`Field`].
//!
//! ## Encodings (IR Loader `constant.js`)
//!
//! | [`Enc`] | Bytes | Raw value |
//! |---|---|---|
//! | `I7`   | 1 | the byte |
//! | `N2`   | 2 | nibbles, high first: `(a<<4)\|b` |
//! | `N4`   | 4 | 4 nibbles, high first |
//! | `N8`   | 8 | 8 nibbles |
//! | `I14`  | 2 | `(a<<7)\|b` |
//! | `Text(n)` | n | ASCII, space padded |
//!
//! Displayed value = raw − `ofs` (so `ofs: 20` stores ±20 dB as 0–40, and
//! `ofs: -1` stores a 1-based number 0-based).

pub use midi_access_core::codec::CodecError;

use crate::address::{from_u28, to_u28};

/// One leaf block of the address map: its absolute wire address and length.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Block {
    /// IR Loader's block id, e.g. `"PATCH_AMP"`.
    pub name: &'static str,
    /// Absolute wire address (four 7-bit bytes, big-endian, in a `u32`).
    /// Memory blocks are given at the temporary memory (`20 00 00 00`).
    pub addr: u32,
    /// Length in bytes.
    pub len: usize,
}

impl Block {
    /// The block's absolute wire address as four bytes.
    pub fn address(&self) -> [u8; 4] {
        self.addr.to_be_bytes()
    }

    /// The block's address relocated from `from` (the area base the map uses)
    /// to `to` (e.g. a stored memory instead of the temporary memory).
    pub fn relocated(&self, from: [u8; 4], to: [u8; 4]) -> [u8; 4] {
        from_u28(to_u28(self.address()) - to_u28(from) + to_u28(to))
    }

    pub(crate) fn check(&self, bytes: &[u8]) -> Result<(), CodecError> {
        if bytes.len() == self.len {
            Ok(())
        } else {
            Err(CodecError::WrongLength {
                expected: self.len,
                actual: bytes.len(),
            })
        }
    }
}

/// Wire encoding of one parameter.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Enc {
    I7,
    N2,
    N4,
    N8,
    I14,
    Text(usize),
}

impl Enc {
    /// Width in bytes.
    pub const fn width(self) -> usize {
        match self {
            Enc::I7 => 1,
            Enc::N2 | Enc::I14 => 2,
            Enc::N4 => 4,
            Enc::N8 => 8,
            Enc::Text(n) => n,
        }
    }
}

/// One parameter of the address map.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct P {
    /// IR Loader's parameter id without the `PRMID_` prefix.
    pub id: &'static str,
    /// The leaf block the parameter lives in.
    pub block: &'static Block,
    /// Byte offset inside the block.
    pub off: usize,
    pub enc: Enc,
    /// Display offset: `display = raw - ofs`.
    pub ofs: i64,
    /// Display range, inclusive.
    pub min: i64,
    pub max: i64,
    /// IR Loader's initial (display) value.
    pub init: i64,
}

impl P {
    /// Absolute wire address of this parameter (at the temporary memory for
    /// memory parameters).
    pub fn address(&self) -> [u8; 4] {
        from_u28(to_u28(self.block.address()) + self.off as u32)
    }

    /// Raw (stored) value from a block's bytes.
    pub fn read_raw(&self, block: &[u8]) -> i64 {
        let b = &block[self.off..self.off + self.enc.width()];
        match self.enc {
            Enc::I7 => b[0] as i64,
            Enc::I14 => ((b[0] as i64) << 7) | b[1] as i64,
            Enc::N2 | Enc::N4 | Enc::N8 => {
                b.iter().fold(0i64, |a, &x| (a << 4) | (x & 0x0F) as i64)
            }
            Enc::Text(_) => 0,
        }
    }

    /// Display value from raw bytes: the first `width` bytes of `data`.
    /// Used for single-parameter DT1s, whose data starts at the parameter.
    pub fn display_from_data(&self, data: &[u8]) -> Option<i64> {
        let w = self.enc.width();
        if data.len() < w || matches!(self.enc, Enc::Text(_)) {
            return None;
        }
        let mut buf = vec![0u8; self.off + w];
        buf[self.off..].copy_from_slice(&data[..w]);
        Some(self.read_raw(&buf) - self.ofs)
    }

    fn write_raw(&self, out: &mut [u8], raw: i64) {
        let w = self.enc.width();
        let b = &mut out[self.off..self.off + w];
        match self.enc {
            Enc::I7 => b[0] = raw as u8,
            Enc::I14 => {
                b[0] = ((raw >> 7) & 0x7F) as u8;
                b[1] = (raw & 0x7F) as u8;
            }
            Enc::N2 | Enc::N4 | Enc::N8 => {
                for (i, x) in b.iter_mut().enumerate() {
                    *x = ((raw >> (4 * (w - 1 - i))) & 0x0F) as u8;
                }
            }
            Enc::Text(_) => {}
        }
    }

    fn range_err(&self, field: &'static str, value: i64) -> CodecError {
        CodecError::OutOfRange {
            field,
            value: value as i32,
            valid: Box::leak(format!("{}..={}", self.min, self.max).into_boxed_str()),
        }
    }

    /// Decode and range-check the display value.
    pub fn decode(&self, block: &[u8], field: &'static str) -> Result<i64, CodecError> {
        let v = self.read_raw(block) - self.ofs;
        if (self.min..=self.max).contains(&v) {
            Ok(v)
        } else {
            Err(self.range_err(field, v))
        }
    }

    /// Range-check and encode a display value.
    pub fn encode(&self, out: &mut [u8], v: i64, field: &'static str) -> Result<(), CodecError> {
        if !(self.min..=self.max).contains(&v) {
            return Err(self.range_err(field, v));
        }
        self.write_raw(out, v + self.ofs);
        Ok(())
    }
}

/// A Rust type a parameter decodes into.
pub trait Field: Sized {
    fn read(block: &[u8], p: &P, field: &'static str) -> Result<Self, CodecError>;
    fn write(&self, out: &mut [u8], p: &P, field: &'static str) -> Result<(), CodecError>;
    fn init(p: &P) -> Self;
}

/// A type whose value is one integer (the display value).
pub trait Display: Sized {
    fn from_display(v: i64) -> Option<Self>;
    fn to_display(&self) -> i64;
}

impl<T: Display> Field for T {
    fn read(block: &[u8], p: &P, field: &'static str) -> Result<Self, CodecError> {
        let v = p.decode(block, field)?;
        T::from_display(v).ok_or_else(|| p.range_err(field, v))
    }
    fn write(&self, out: &mut [u8], p: &P, field: &'static str) -> Result<(), CodecError> {
        p.encode(out, self.to_display(), field)
    }
    fn init(p: &P) -> Self {
        T::from_display(p.init)
            .or_else(|| T::from_display(p.min))
            .expect("map init/min is representable")
    }
}

/// Spoken form of a display value for a field's type: `on`/`off` for
/// switches, the variant name in words for enums (`normal mono return`), the
/// number otherwise. Catalog names are applied on top by [`crate::live`].
pub trait Show {
    fn show(v: i64) -> String;
}

impl<T: Display + serde::Serialize> Show for T {
    fn show(v: i64) -> String {
        match T::from_display(v).map(|x| serde_norway::to_value(&x)) {
            Some(Ok(serde_norway::Value::Bool(b))) => if b { "on" } else { "off" }.to_string(),
            Some(Ok(serde_norway::Value::String(s))) => s.replace('_', " "),
            _ => v.to_string(),
        }
    }
}

impl Show for String {
    fn show(_v: i64) -> String {
        String::new()
    }
}

macro_rules! int_display {
    ($($t:ty),*) => {$(
        impl Display for $t {
            fn from_display(v: i64) -> Option<Self> { <$t>::try_from(v).ok() }
            fn to_display(&self) -> i64 { *self as i64 }
        }
    )*};
}
int_display!(u8, i8, u16, i16);

impl Display for bool {
    fn from_display(v: i64) -> Option<Self> {
        match v {
            0 => Some(false),
            1 => Some(true),
            _ => None,
        }
    }
    fn to_display(&self) -> i64 {
        *self as i64
    }
}

/// Text: 7-bit ASCII (the pedal stores control bytes too), space padded on the wire, trailing spaces trimmed in
/// the document.
impl Field for String {
    fn read(block: &[u8], p: &P, field: &'static str) -> Result<Self, CodecError> {
        let n = p.enc.width();
        let b = &block[p.off..p.off + n];
        if b.iter().any(|&c| c >= 0x80) {
            return Err(CodecError::BadString {
                field,
                reason: "byte above 0x7F",
            });
        }
        Ok(String::from_utf8_lossy(b).trim_end().to_string())
    }
    fn write(&self, out: &mut [u8], p: &P, field: &'static str) -> Result<(), CodecError> {
        let n = p.enc.width();
        if self.len() > n {
            return Err(CodecError::BadString {
                field,
                reason: "too long",
            });
        }
        if self.bytes().any(|c| c >= 0x80) {
            return Err(CodecError::BadString {
                field,
                reason: "only ASCII is allowed",
            });
        }
        let mut b = self.as_bytes().to_vec();
        b.resize(n, b' ');
        out[p.off..p.off + n].copy_from_slice(&b);
        Ok(())
    }
    fn init(_p: &P) -> Self {
        String::new()
    }
}

/// A closed list of named values stored as one integer.
///
/// ```ignore
/// value_enum! {
///     /// Amp gain switch.
///     GainSwitch { Low = 0, Middle = 1, High = 2 }
/// }
/// ```
#[macro_export]
macro_rules! value_enum {
    ($(#[$m:meta])* $name:ident { $($(#[$vm:meta])* $v:ident = $n:expr),+ $(,)? }) => {
        $(#[$m])*
        #[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
        #[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[derive(Debug, Clone, Copy, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(rename_all = "snake_case")]
        pub enum $name { $($(#[$vm])* $v),+ }

        impl $crate::codec::Display for $name {
            fn from_display(v: i64) -> Option<Self> {
                match v { $(x if x == $n => Some(Self::$v),)+ _ => None }
            }
            fn to_display(&self) -> i64 {
                match self { $(Self::$v => $n),+ }
            }
        }
    };
}

/// A typed struct for one leaf block: every field names its [`P`].
///
/// Generates `from_bytes` / `to_bytes` / `init`. The struct may be reused for
/// several blocks with the same layout (EQ A/B, CTL1/2 …): `from_bytes` only
/// checks the length, the caller picks the address.
#[macro_export]
macro_rules! block {
    ($(#[$m:meta])* $name:ident = $blk:path { $( $(#[$fm:meta])* $f:ident : $t:ty = $p:path ),* $(,)? }) => {
        $(#[$m])*
        #[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
        #[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema", schemars(deny_unknown_fields))]
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name { $( $(#[$fm])* pub $f: $t, )* }

        impl $name {
            /// The reference block (offsets and length are shared by every
            /// block this struct describes).
            pub const BLOCK: &'static $crate::codec::Block = &$blk;

            pub fn from_bytes(b: &[u8]) -> Result<Self, $crate::codec::CodecError> {
                Self::BLOCK.check(b)?;
                Ok(Self { $( $f: <$t as $crate::codec::Field>::read(b, &$p, stringify!($f))?, )* })
            }

            pub fn to_bytes(&self) -> Result<Vec<u8>, $crate::codec::CodecError> {
                let mut out = vec![0u8; Self::BLOCK.len];
                $( $crate::codec::Field::write(&self.$f, &mut out, &$p, stringify!($f))?; )*
                Ok(out)
            }

            /// IR Loader's initial values.
            pub fn init() -> Self {
                Self { $( $f: <$t as $crate::codec::Field>::init(&$p), )* }
            }

            /// The parameters, in field order (for tests and metadata).
            pub fn params() -> &'static [&'static $crate::codec::P] {
                &[$(&$p),*]
            }

            /// `(field, spoken value)` for the parameter at `p.off` of any
            /// block this struct describes.
            pub fn describe_param(p: &$crate::codec::P, v: i64) -> Option<(&'static str, String)> {
                $( if p.off == $p.off {
                    return Some((stringify!($f), <$t as $crate::codec::Show>::show(v)));
                } )*
                None
            }
        }
    };
}

/// A whole area (system, memory) assembled from leaf blocks.
///
/// Each field is either `block(<map block>)`, a [`block!`] struct read from
/// that block, or `param(<map param>)`, a single value read from the block that
/// holds it (for the one-parameter blocks such as `EQ LINK`).
#[macro_export]
macro_rules! area {
    ($(#[$m:meta])* $name:ident { $( $(#[$fm:meta])* $f:ident : $t:ty = $kind:ident($p:path) ),* $(,)? }) => {
        $(#[$m])*
        #[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
        #[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
        #[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
        #[cfg_attr(feature = "schema", schemars(deny_unknown_fields))]
        #[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
        #[serde(deny_unknown_fields)]
        pub struct $name { $( $(#[$fm])* pub $f: $t, )* }

        impl $name {
            /// Every leaf block of the area, in address order.
            pub fn blocks() -> Vec<&'static $crate::codec::Block> {
                vec![$( $crate::area!(@block $kind $p) ),*]
            }

            /// Decode from a lookup that returns each block's bytes.
            pub fn from_blocks<'a>(
                get: impl Fn(&$crate::codec::Block) -> Result<&'a [u8], $crate::codec::CodecError>,
            ) -> Result<Self, $crate::codec::CodecError> {
                Ok(Self { $( $f: $crate::area!(@get $kind $p, $t, get, $f), )* })
            }

            /// Encode to `(block, bytes)` pairs, in address order.
            pub fn to_blocks(
                &self,
            ) -> Result<Vec<(&'static $crate::codec::Block, Vec<u8>)>, $crate::codec::CodecError> {
                Ok(vec![$( $crate::area!(@put $kind $p, self.$f, $f) ),*])
            }

            /// IR Loader's initial values.
            pub fn init() -> Self {
                Self { $( $f: $crate::area!(@init $kind $p, $t), )* }
            }

            /// `(serde path within the area, spoken value)` for parameter `p`
            /// (given at its map address) with display value `v`.
            pub fn describe_param(p: &$crate::codec::P, v: i64) -> Option<(String, String)> {
                $( if let Some(r) = $crate::area!(@desc $kind $p, $t, p, v, $f) { return Some(r); } )*
                None
            }
        }
    };
    (@desc block $p:path, $t:ty, $q:ident, $v:ident, $f:ident) => {
        if $q.block.addr == $p.addr {
            <$t>::describe_param($q, $v).map(|(n, s)| (format!("{}.{}", stringify!($f), n), s))
        } else {
            None
        }
    };
    (@desc param $p:path, $t:ty, $q:ident, $v:ident, $f:ident) => {
        if $q.id == $p.id {
            Some((stringify!($f).to_string(), <$t as $crate::codec::Show>::show($v)))
        } else {
            None
        }
    };
    (@block block $p:path) => { &$p };
    (@block param $p:path) => { $p.block };
    (@get block $p:path, $t:ty, $get:ident, $f:ident) => { <$t>::from_bytes($get(&$p)?)? };
    (@get param $p:path, $t:ty, $get:ident, $f:ident) => {{
        let b = $get($p.block)?;
        $p.block.check(b)?;
        <$t as $crate::codec::Field>::read(b, &$p, stringify!($f))?
    }};
    (@put block $p:path, $v:expr, $f:ident) => { (&$p, $v.to_bytes()?) };
    (@put param $p:path, $v:expr, $f:ident) => {{
        let mut out = vec![0u8; $p.block.len];
        $crate::codec::Field::write(&$v, &mut out, &$p, stringify!($f))?;
        ($p.block, out)
    }};
    (@init block $p:path, $t:ty) => { <$t>::init() };
    (@init param $p:path, $t:ty) => { <$t as $crate::codec::Field>::init(&$p) };
}

#[cfg(test)]
mod tests {
    use crate::map;

    #[test]
    fn nibble_encodings_round_trip() {
        let p = map::patch_sr::SEND_LEVEL; // N2, 0..200
        let mut b = vec![0u8; p.block.len];
        p.encode(&mut b, 200, "x").unwrap();
        assert_eq!(&b[2..4], &[0x0C, 0x08]);
        assert_eq!(p.decode(&b, "x").unwrap(), 200);

        let c = map::patch_cab_a::CAB_TYPE; // N4, 0..336
        let mut b = vec![0u8; c.block.len];
        c.encode(&mut b, 336, "x").unwrap();
        assert_eq!(&b[0..4], &[0x00, 0x01, 0x05, 0x00]);
        assert_eq!(c.decode(&b, "x").unwrap(), 336);
    }

    #[test]
    fn display_offsets() {
        let p = map::patch_eq_a::PEQ_LOW_GAIN; // ofs 20
        let mut b = vec![0u8; p.block.len];
        p.encode(&mut b, -20, "x").unwrap();
        assert_eq!(b[p.off], 0);
        p.encode(&mut b, 20, "x").unwrap();
        assert_eq!(b[p.off], 40);
        assert!(p.encode(&mut b, 21, "x").is_err());

        let m = map::system_memext::MIN; // ofs -1: 1-based stored 0-based
        let mut b = vec![0u8; m.block.len];
        m.encode(&mut b, 1, "x").unwrap();
        assert_eq!(b[0], 0);
    }

    #[test]
    fn relocation_carries_7bit() {
        let amp = &map::patch_amp::BLOCK;
        let to = crate::address::memory_base(8).unwrap();
        assert_eq!(
            amp.relocated(crate::address::TEMPORARY_BASE, to),
            [0x21, 0x00, 0x10, 0x00]
        );
    }

    #[test]
    fn display_from_single_dt1_data() {
        let p = map::patch_amp::GAIN;
        assert_eq!(p.display_from_data(&[72]), Some(72));
        let s = map::patch_sr::SEND_LEVEL;
        assert_eq!(s.display_from_data(&[0x0C, 0x08]), Some(200));
    }
}
