//! Describe inbound parameter changes.
//!
//! In editor communication mode the pedal reports its own changes as DT1s
//! (to be confirmed on hardware, see `docs/sysex-notes.md`). [`describe`] turns
//! one DT1 into the parameters it touches with their display values, so a host
//! can say "Amp gain 72" as the knob turns.

use serde::Serialize;

use crate::address::{to_u28, AddressSpace, TEMPORARY_BASE};
use crate::codec::{Enc, P};
use crate::map;

/// One parameter a DT1 changed.
#[cfg_attr(feature = "tsify", derive(tsify_next::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Change {
    /// IR Loader's parameter id, e.g. `PATCH_AMP_GAIN`.
    pub id: String,
    /// Area: `setup`, `system`, `memory` (temporary) or `pc-map`.
    pub area: String,
    /// Stored memory 1–128 when the DT1 addressed a stored memory, else `None`.
    pub memory: Option<u8>,
    /// Display value (numbers), or `None` for text.
    pub value: Option<i64>,
    /// Text value (memory names).
    pub text: Option<String>,
}

fn area_of(p: &P) -> &'static str {
    match p.block.addr >> 24 {
        0x00 => "setup",
        0x10 => "system",
        0x20 => "memory",
        0x50 => "pc-map",
        _ => "unknown",
    }
}

/// Every parameter, numeric and text, that [`describe`] knows.
fn all() -> impl Iterator<Item = &'static P> {
    map::ALL
        .iter()
        .chain(std::iter::once(&map::patch_com::NAME))
}

/// The parameters a DT1 of `data` at `address` touches. Writes to a stored
/// memory are mapped onto the memory layout (with [`Change::memory`] set).
pub fn describe(address: [u8; 4], data: &[u8]) -> Vec<Change> {
    let (lookup, memory) = match AddressSpace::classify(address) {
        AddressSpace::Memory(n) => {
            let base = crate::address::memory_base(n).unwrap();
            let rel = to_u28(address) - to_u28(base);
            (to_u28(TEMPORARY_BASE) + rel, Some(n))
        }
        _ => (to_u28(address), None),
    };
    let end = lookup + data.len() as u32;
    let mut out = Vec::new();
    for p in all() {
        let a = to_u28(p.address());
        let w = p.enc.width() as u32;
        if a < lookup || a + w > end {
            continue;
        }
        let slice = &data[(a - lookup) as usize..(a - lookup + w) as usize];
        let (value, text) = match p.enc {
            Enc::Text(_) => (
                None,
                Some(String::from_utf8_lossy(slice).trim_end().to_string()),
            ),
            _ => (p.display_from_data(slice), None),
        };
        out.push(Change {
            id: p.id.to_string(),
            area: area_of(p).to_string(),
            memory,
            value,
            text,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn single_knob() {
        let c = describe([0x20, 0x00, 0x10, 0x01], &[72]);
        assert_eq!(c.len(), 1);
        assert_eq!(c[0].id, "PATCH_AMP_GAIN");
        assert_eq!(c[0].value, Some(72));
        assert_eq!(c[0].memory, None);
    }

    #[test]
    fn setup_and_signed_values() {
        let c = describe([0, 0, 0, 0], &[0x00, 0x05]);
        assert_eq!(c[0].id, "SETUP_COM_PATCH_NUMBER");
        assert_eq!(c[0].value, Some(5));
        let c = describe([0x10, 0x00, 0x30, 0x02], &[0]);
        assert_eq!(c[0].id, "SYSTEM_EQ_A_PEQ_LOW_GAIN");
        assert_eq!(c[0].value, Some(-20));
    }

    #[test]
    fn whole_block_and_stored_memory() {
        let c = describe([0x21, 0x00, 0x10, 0x00], &[1, 50, 50, 50, 50, 50, 1, 0, 50]);
        assert_eq!(c.len(), 9);
        assert!(c.iter().all(|x| x.memory == Some(8)));
        let name = describe([0x20, 0, 0, 0], b"CLEAN           ");
        assert_eq!(name[0].text.as_deref(), Some("CLEAN"));
    }
}
