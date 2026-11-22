//! MIDI PC MAP (`50 00 00 00`): which memory each Program Change selects.
//!
//! 128 entries, two nibbles each (256 bytes). Each entry stores the memory
//! number 1–128 directly (hardware: a stock map reads PC#n = n). IR Loader's
//! range also allows 0, which a real map contained for one entry; its meaning
//! is unknown.

use serde::{Deserialize, Serialize};

use crate::codec::{CodecError, P};
use crate::map;

/// Number of Program Change entries.
pub const PC_COUNT: usize = 128;

/// The map as stored: `memory[i]` is the stored value for PC#`i + 1`.
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi, from_wasm_abi))]
#[cfg_attr(feature = "schema", derive(schemars::JsonSchema))]
#[cfg_attr(feature = "schema", schemars(deny_unknown_fields))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ProgramChangeMap {
    /// Memory number (1–128) for PC#1 … PC#128.
    pub memory: Vec<u8>,
}

fn entry(i: usize) -> P {
    let mut p = map::midi_pcmap::PC1;
    p.off = i * 2;
    p
}

impl ProgramChangeMap {
    pub fn from_bytes(b: &[u8]) -> Result<Self, CodecError> {
        map::midi_pcmap::BLOCK.check(b)?;
        let memory = (0..PC_COUNT)
            .map(|i| entry(i).decode(b, "memory").map(|v| v as u8))
            .collect::<Result<_, _>>()?;
        Ok(Self { memory })
    }

    pub fn to_bytes(&self) -> Result<Vec<u8>, CodecError> {
        if self.memory.len() != PC_COUNT {
            return Err(CodecError::WrongLength {
                expected: PC_COUNT,
                actual: self.memory.len(),
            });
        }
        let mut out = vec![0u8; map::midi_pcmap::BLOCK.len];
        for (i, &m) in self.memory.iter().enumerate() {
            entry(i).encode(&mut out, m as i64, "memory")?;
        }
        Ok(out)
    }

    /// Identity map: PC#n selects memory n.
    pub fn init() -> Self {
        Self {
            memory: (1..=PC_COUNT as u8).collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn round_trip_and_layout() {
        let m = ProgramChangeMap::init();
        let b = m.to_bytes().unwrap();
        assert_eq!(b.len(), 256);
        assert_eq!(&b[254..], &[0x08, 0x00]);
        assert_eq!(ProgramChangeMap::from_bytes(&b).unwrap(), m);
        assert_eq!(map::midi_pcmap::PC128.off, 254);
    }
}
