//! Setup area (`00 00 00 00`): the current memory number.

use crate::block;
use crate::map;

block! {
    /// The currently selected memory. Writing it switches memories.
    Setup = map::setup_com::BLOCK {
        /// 0–127 = memory 1–128.
        memory_index: u8 = map::setup_com::PATCH_NUMBER,
    }
}

impl Setup {
    /// The memory number as the pedal shows it, 1–128.
    pub fn memory(&self) -> u8 {
        self.memory_index + 1
    }

    /// Setup selecting memory `n` (1–128).
    pub fn for_memory(n: u8) -> Option<Self> {
        (1..=128).contains(&n).then(|| Self {
            memory_index: n - 1,
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn two_nibbles() {
        let s = Setup::for_memory(100).unwrap();
        assert_eq!(s.to_bytes().unwrap(), vec![0x06, 0x03]);
        assert_eq!(Setup::from_bytes(&[0x06, 0x03]).unwrap().memory(), 100);
        assert!(Setup::for_memory(0).is_none());
    }
}
