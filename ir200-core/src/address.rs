//! IR-200 address space.
//!
//! | Address | Area |
//! |---|---|
//! | `00 00 00 00` | Setup (current memory number) |
//! | `10 00 00 00` | System (19 leaf blocks, `10 00 00 00` … `10 02 20 00`) |
//! | `20 00 00 00` | Temporary memory (the edit buffer) |
//! | `20 10 00 00` … `30 00 00 00` | Stored memories 1–128, stride `00 10 00 00` |
//! | `40 00 00 00` | Cab info: 337 favourite flags, one per cab type |
//! | `50 00 00 00` | MIDI Program Change map |
//! | `60 00 00 00` | IR info: 128 slot indices |
//! | `68 00 00 00` | User IR data, one block per slot, stride `00 02 00 00` |
//! | `7F 00 00 00` | Commands (editor mode, running mode, patch write …) |
//!
//! Roland addresses are four **7-bit** bytes: a 28-bit big-endian value that
//! carries at `0x80`. All offset arithmetic goes through [`to_u28`] /
//! [`from_u28`] (IR Loader's `nibble()` / `_7bitize()`).

/// Setup area base.
pub const SETUP_BASE: [u8; 4] = [0x00, 0x00, 0x00, 0x00];
/// System area base.
pub const SYSTEM_BASE: [u8; 4] = [0x10, 0x00, 0x00, 0x00];
/// Temporary memory (edit buffer) base.
pub const TEMPORARY_BASE: [u8; 4] = [0x20, 0x00, 0x00, 0x00];
/// Stride between stored memories.
pub const MEMORY_STRIDE: [u8; 4] = [0x00, 0x10, 0x00, 0x00];
/// Number of stored memories.
pub const MEMORY_COUNT: u8 = 128;
/// Cab info (favourite flags) base.
pub const CAB_INFO_BASE: [u8; 4] = [0x40, 0x00, 0x00, 0x00];
/// Number of cab types (and favourite flags).
pub const CAB_TYPE_COUNT: usize = 337;
/// MIDI Program Change map base.
pub const PC_MAP_BASE: [u8; 4] = [0x50, 0x00, 0x00, 0x00];
/// IR info base.
pub const IR_INFO_BASE: [u8; 4] = [0x60, 0x00, 0x00, 0x00];
/// User IR data base (slot 1).
pub const IR_DATA_BASE: [u8; 4] = [0x68, 0x00, 0x00, 0x00];
/// Stride between user IR data slots.
pub const IR_DATA_STRIDE: [u8; 4] = [0x00, 0x02, 0x00, 0x00];
/// Number of user IR slots.
pub const IR_SLOT_COUNT: u8 = 128;
/// Length of a user IR name.
pub const IR_NAME_LEN: usize = 32;
/// Offset of the IR file-name length (`INTEGER4x4`, 0 = slot empty) in a slot.
pub const IR_FILE_OFFSET: [u8; 4] = [0x00, 0x00, 0x00, 0x20];

/// Editor communication level (RQ1, size 1; IR Loader expects `0`).
pub const CMD_COMM_LEVEL: [u8; 4] = [0x7F, 0x00, 0x00, 0x00];
/// Editor communication mode (DT1 `01` on, `00` off).
pub const CMD_COMM_MODE: [u8; 4] = [0x7F, 0x00, 0x00, 0x01];
/// Running mode. **`04` is a factory reset**: never write it.
pub const CMD_RUNNING_MODE: [u8; 4] = [0x7F, 0x00, 0x00, 0x02];
/// Editor communication revision.
pub const CMD_COMM_REVISION: [u8; 4] = [0x7F, 0x00, 0x00, 0x03];
/// Patch select.
pub const CMD_PATCH_SELECT: [u8; 4] = [0x7F, 0x00, 0x01, 0x00];
/// Patch stored.
pub const CMD_PATCH_STORED: [u8; 4] = [0x7F, 0x00, 0x01, 0x02];
/// Patch write (store the temporary memory).
pub const CMD_PATCH_WRITE: [u8; 4] = [0x7F, 0x00, 0x01, 0x04];
/// Clear the temporary memory.
pub const CMD_PATCH_TEMP_CLEAR: [u8; 4] = [0x7F, 0x01, 0x00, 0x03];
/// Initialise IR data.
pub const CMD_IR_DATA_INIT: [u8; 4] = [0x7F, 0x00, 0x02, 0x00];

/// Pack a 4-byte Roland address into its 28-bit value (7 bits per byte).
pub fn to_u28(addr: [u8; 4]) -> u32 {
    ((addr[0] as u32 & 0x7F) << 21)
        | ((addr[1] as u32 & 0x7F) << 14)
        | ((addr[2] as u32 & 0x7F) << 7)
        | (addr[3] as u32 & 0x7F)
}

/// Unpack a 28-bit value into a 4-byte Roland address.
pub fn from_u28(v: u32) -> [u8; 4] {
    [
        ((v >> 21) & 0x7F) as u8,
        ((v >> 14) & 0x7F) as u8,
        ((v >> 7) & 0x7F) as u8,
        (v & 0x7F) as u8,
    ]
}

/// Add an offset to a base address with 7-bit carry.
pub fn add_offset(base: [u8; 4], offset: [u8; 4]) -> [u8; 4] {
    from_u28(to_u28(base) + to_u28(offset))
}

/// A byte count as RQ1's 4-byte, 7-bit size field.
pub fn size_field(n: usize) -> [u8; 4] {
    from_u28(n as u32)
}

/// Base of stored memory `n` (1..=128), or `None` out of range.
///
/// ```
/// use ir200_core::address::memory_base;
/// assert_eq!(memory_base(1), Some([0x20, 0x10, 0x00, 0x00]));
/// assert_eq!(memory_base(8), Some([0x21, 0x00, 0x00, 0x00]));
/// assert_eq!(memory_base(128), Some([0x30, 0x00, 0x00, 0x00]));
/// assert_eq!(memory_base(0), None);
/// ```
pub fn memory_base(n: u8) -> Option<[u8; 4]> {
    (1..=MEMORY_COUNT)
        .contains(&n)
        .then(|| from_u28(to_u28(TEMPORARY_BASE) + n as u32 * to_u28(MEMORY_STRIDE)))
}

/// Base of user IR slot `n` (1..=128): the 32-character IR name lives here.
///
/// ```
/// use ir200_core::address::ir_slot_base;
/// assert_eq!(ir_slot_base(1), Some([0x68, 0x00, 0x00, 0x00]));
/// assert_eq!(ir_slot_base(2), Some([0x68, 0x02, 0x00, 0x00]));
/// assert_eq!(ir_slot_base(65), Some([0x69, 0x00, 0x00, 0x00]));
/// ```
pub fn ir_slot_base(n: u8) -> Option<[u8; 4]> {
    (1..=IR_SLOT_COUNT)
        .contains(&n)
        .then(|| from_u28(to_u28(IR_DATA_BASE) + (n as u32 - 1) * to_u28(IR_DATA_STRIDE)))
}

/// Which stored memory (1..=128) a wire address falls in, `Some(0)` for the
/// temporary memory, `None` outside memory space.
pub fn memory_of(addr: [u8; 4]) -> Option<u8> {
    let a = to_u28(addr);
    let base = to_u28(TEMPORARY_BASE);
    let stride = to_u28(MEMORY_STRIDE);
    if a < base {
        return None;
    }
    let n = (a - base) / stride;
    (n <= MEMORY_COUNT as u32).then_some(n as u8)
}

/// Top-level partition of the address space.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AddressSpace {
    Setup,
    System,
    /// Temporary memory.
    Temporary,
    /// Stored memory 1..=128.
    Memory(u8),
    CabInfo,
    PcMap,
    IrInfo,
    IrData,
    Command,
    Unknown,
}

impl AddressSpace {
    pub fn classify(addr: [u8; 4]) -> Self {
        match addr[0] {
            0x00 => Self::Setup,
            0x10 => Self::System,
            0x20..=0x30 => match memory_of(addr) {
                Some(0) => Self::Temporary,
                Some(n) => Self::Memory(n),
                None => Self::Unknown,
            },
            0x40 => Self::CabInfo,
            0x50 => Self::PcMap,
            0x60 => Self::IrInfo,
            0x68..=0x6F => Self::IrData,
            0x7F => Self::Command,
            _ => Self::Unknown,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn u28_round_trips() {
        for a in [
            [0, 0, 0, 0],
            [0x7F, 0x7F, 0x7F, 0x7F],
            [0x21, 0x00, 0x10, 0x00],
        ] {
            assert_eq!(from_u28(to_u28(a)), a);
        }
    }

    #[test]
    fn memory_addresses_match_ir_loader_map() {
        // IR Loader's root list: PATCH(1) 20100000, PATCH(8) 21000000,
        // PATCH(9) 21100000, PATCH(123) 2F300000, PATCH(128) 30000000.
        assert_eq!(memory_base(9), Some([0x21, 0x10, 0x00, 0x00]));
        assert_eq!(memory_base(123), Some([0x2F, 0x30, 0x00, 0x00]));
        assert_eq!(memory_base(129), None);
        for n in 1..=128u8 {
            assert_eq!(memory_of(memory_base(n).unwrap()), Some(n));
        }
        assert_eq!(memory_of(TEMPORARY_BASE), Some(0));
        assert_eq!(memory_of([0x20, 0x01, 0x10, 0x00]), Some(0));
    }

    #[test]
    fn classify() {
        assert_eq!(
            AddressSpace::classify([0x20, 0x00, 0x10, 0x00]),
            AddressSpace::Temporary
        );
        assert_eq!(
            AddressSpace::classify([0x30, 0x00, 0x10, 0x00]),
            AddressSpace::Memory(128)
        );
        assert_eq!(AddressSpace::classify(CMD_COMM_MODE), AddressSpace::Command);
        assert_eq!(
            AddressSpace::classify([0x69, 0, 0, 0]),
            AddressSpace::IrData
        );
    }

    #[test]
    fn size_field_is_7bit() {
        assert_eq!(size_field(256), [0, 0, 0x02, 0x00]);
        assert_eq!(size_field(337), [0, 0, 0x02, 0x51]);
    }
}
