//! Parameter address map, **generated** by `tools/gen/gen_map.py` from IR
//! Loader's `address_map.js` (the IR-200's only parameter specification).
//! Do not edit by hand: regenerate instead.
//!
//! One module per leaf block. `BLOCK` is the block's absolute wire address
//! (memory blocks are given at the temporary memory, `20 00 00 00`) and byte
//! length; every other const is one parameter with its offset inside the
//! block, wire encoding, display offset (`display = raw - ofs`), display
//! range and IR Loader's init value.

#![allow(dead_code)]
#![cfg_attr(rustfmt, rustfmt_skip)]

use crate::codec::{Block, Enc, P};

/// `SETUP_COM` at `00 00 00 00`.
pub mod setup_com {
    use super::*;
    pub const BLOCK: Block = Block { name: "SETUP_COM", addr: 0x00000000, len: 2 };
    pub const PATCH_NUMBER: P = P { id: "SETUP_COM_PATCH_NUMBER", block: &BLOCK, off: 0, enc: Enc::N2, ofs: 0, min: 0, max: 127, init: 0 };
}

/// `SYSTEM_MODULE` at `10 00 00 00`.
pub mod system_module {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_MODULE", addr: 0x10000000, len: 3 };
    pub const AMP: P = P { id: "SYSTEM_MODULE_AMP", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
    pub const CABINET: P = P { id: "SYSTEM_MODULE_CABINET", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
    pub const AMBIENCE: P = P { id: "SYSTEM_MODULE_AMBIENCE", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
}

/// `SYSTEM_PHONES` at `10 00 10 00`.
pub mod system_phones {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_PHONES", addr: 0x10001000, len: 3 };
    pub const VOLUME: P = P { id: "SYSTEM_PHONES_VOLUME", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 50 };
    pub const SURROUND: P = P { id: "SYSTEM_PHONES_SURROUND", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
    pub const AMB_LEVEL: P = P { id: "SYSTEM_PHONES_AMB_LEVEL", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 50 };
}

/// `SYSTEM_EQ` at `10 00 20 00`.
pub mod system_eq {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_EQ", addr: 0x10002000, len: 1 };
    pub const LINK: P = P { id: "SYSTEM_EQ_LINK", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
}

/// `SYSTEM_EQ_A` at `10 00 30 00`.
pub mod system_eq_a {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_EQ_A", addr: 0x10003000, len: 22 };
    pub const SW: P = P { id: "SYSTEM_EQ_A_SW", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const TYPE: P = P { id: "SYSTEM_EQ_A_TYPE", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PEQ_LOW_GAIN: P = P { id: "SYSTEM_EQ_A_PEQ_LOW_GAIN", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_HIGH_GAIN: P = P { id: "SYSTEM_EQ_A_PEQ_HIGH_GAIN", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_LEVEL: P = P { id: "SYSTEM_EQ_A_PEQ_LEVEL", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_LOMID_FREQ: P = P { id: "SYSTEM_EQ_A_PEQ_LOMID_FREQ", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 29, init: 14 };
    pub const PEQ_LOMID_Q: P = P { id: "SYSTEM_EQ_A_PEQ_LOMID_Q", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 0, min: 0, max: 5, init: 1 };
    pub const PEQ_LOMID_GAIN: P = P { id: "SYSTEM_EQ_A_PEQ_LOMID_GAIN", block: &BLOCK, off: 7, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_HIMID_FREQ: P = P { id: "SYSTEM_EQ_A_PEQ_HIMID_FREQ", block: &BLOCK, off: 8, enc: Enc::I7, ofs: 0, min: 0, max: 29, init: 23 };
    pub const PEQ_HIMID_Q: P = P { id: "SYSTEM_EQ_A_PEQ_HIMID_Q", block: &BLOCK, off: 9, enc: Enc::I7, ofs: 0, min: 0, max: 5, init: 1 };
    pub const PEQ_HIMID_GAIN: P = P { id: "SYSTEM_EQ_A_PEQ_HIMID_GAIN", block: &BLOCK, off: 10, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_LEVEL: P = P { id: "SYSTEM_EQ_A_GEQ_LEVEL", block: &BLOCK, off: 11, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_31HZ_GAIN: P = P { id: "SYSTEM_EQ_A_GEQ_31Hz_GAIN", block: &BLOCK, off: 12, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_63HZ_GAIN: P = P { id: "SYSTEM_EQ_A_GEQ_63Hz_GAIN", block: &BLOCK, off: 13, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_125HZ_GAIN: P = P { id: "SYSTEM_EQ_A_GEQ_125Hz_GAIN", block: &BLOCK, off: 14, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_250HZ_GAIN: P = P { id: "SYSTEM_EQ_A_GEQ_250Hz_GAIN", block: &BLOCK, off: 15, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_500HZ_GAIN: P = P { id: "SYSTEM_EQ_A_GEQ_500Hz_GAIN", block: &BLOCK, off: 16, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_1KHZ_GAIN: P = P { id: "SYSTEM_EQ_A_GEQ_1kHz_GAIN", block: &BLOCK, off: 17, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_2KHZ_GAIN: P = P { id: "SYSTEM_EQ_A_GEQ_2kHz_GAIN", block: &BLOCK, off: 18, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_4KHZ_GAIN: P = P { id: "SYSTEM_EQ_A_GEQ_4kHz_GAIN", block: &BLOCK, off: 19, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_8KHZ_GAIN: P = P { id: "SYSTEM_EQ_A_GEQ_8kHz_GAIN", block: &BLOCK, off: 20, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_16KHZ_GAIN: P = P { id: "SYSTEM_EQ_A_GEQ_16kHz_GAIN", block: &BLOCK, off: 21, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
}

/// `SYSTEM_EQ_B` at `10 00 40 00`.
pub mod system_eq_b {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_EQ_B", addr: 0x10004000, len: 22 };
    pub const SW: P = P { id: "SYSTEM_EQ_B_SW", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const TYPE: P = P { id: "SYSTEM_EQ_B_TYPE", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PEQ_LOW_GAIN: P = P { id: "SYSTEM_EQ_B_PEQ_LOW_GAIN", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_HIGH_GAIN: P = P { id: "SYSTEM_EQ_B_PEQ_HIGH_GAIN", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_LEVEL: P = P { id: "SYSTEM_EQ_B_PEQ_LEVEL", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_LOMID_FREQ: P = P { id: "SYSTEM_EQ_B_PEQ_LOMID_FREQ", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 29, init: 14 };
    pub const PEQ_LOMID_Q: P = P { id: "SYSTEM_EQ_B_PEQ_LOMID_Q", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 0, min: 0, max: 5, init: 1 };
    pub const PEQ_LOMID_GAIN: P = P { id: "SYSTEM_EQ_B_PEQ_LOMID_GAIN", block: &BLOCK, off: 7, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_HIMID_FREQ: P = P { id: "SYSTEM_EQ_B_PEQ_HIMID_FREQ", block: &BLOCK, off: 8, enc: Enc::I7, ofs: 0, min: 0, max: 29, init: 23 };
    pub const PEQ_HIMID_Q: P = P { id: "SYSTEM_EQ_B_PEQ_HIMID_Q", block: &BLOCK, off: 9, enc: Enc::I7, ofs: 0, min: 0, max: 5, init: 1 };
    pub const PEQ_HIMID_GAIN: P = P { id: "SYSTEM_EQ_B_PEQ_HIMID_GAIN", block: &BLOCK, off: 10, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_LEVEL: P = P { id: "SYSTEM_EQ_B_GEQ_LEVEL", block: &BLOCK, off: 11, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_31HZ_GAIN: P = P { id: "SYSTEM_EQ_B_GEQ_31Hz_GAIN", block: &BLOCK, off: 12, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_63HZ_GAIN: P = P { id: "SYSTEM_EQ_B_GEQ_63Hz_GAIN", block: &BLOCK, off: 13, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_125HZ_GAIN: P = P { id: "SYSTEM_EQ_B_GEQ_125Hz_GAIN", block: &BLOCK, off: 14, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_250HZ_GAIN: P = P { id: "SYSTEM_EQ_B_GEQ_250Hz_GAIN", block: &BLOCK, off: 15, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_500HZ_GAIN: P = P { id: "SYSTEM_EQ_B_GEQ_500Hz_GAIN", block: &BLOCK, off: 16, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_1KHZ_GAIN: P = P { id: "SYSTEM_EQ_B_GEQ_1kHz_GAIN", block: &BLOCK, off: 17, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_2KHZ_GAIN: P = P { id: "SYSTEM_EQ_B_GEQ_2kHz_GAIN", block: &BLOCK, off: 18, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_4KHZ_GAIN: P = P { id: "SYSTEM_EQ_B_GEQ_4kHz_GAIN", block: &BLOCK, off: 19, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_8KHZ_GAIN: P = P { id: "SYSTEM_EQ_B_GEQ_8kHz_GAIN", block: &BLOCK, off: 20, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_16KHZ_GAIN: P = P { id: "SYSTEM_EQ_B_GEQ_16kHz_GAIN", block: &BLOCK, off: 21, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
}

/// `SYSTEM_OUT` at `10 00 50 00`.
pub mod system_out {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_OUT", addr: 0x10005000, len: 1 };
    pub const LINK: P = P { id: "SYSTEM_OUT_LINK", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
}

/// `SYSTEM_OUT_A` at `10 00 60 00`.
pub mod system_out_a {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_OUT_A", addr: 0x10006000, len: 1 };
    pub const TYPE: P = P { id: "SYSTEM_OUT_A_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 10, init: 0 };
}

/// `SYSTEM_OUT_B` at `10 00 70 00`.
pub mod system_out_b {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_OUT_B", addr: 0x10007000, len: 1 };
    pub const TYPE: P = P { id: "SYSTEM_OUT_B_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 10, init: 0 };
}

/// `SYSTEM_MEMEXT` at `10 01 00 00`.
pub mod system_memext {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_MEMEXT", addr: 0x10010000, len: 2 };
    pub const MIN: P = P { id: "SYSTEM_MEMEXT_MIN", block: &BLOCK, off: 0, enc: Enc::I7, ofs: -1, min: 1, max: 128, init: 1 };
    pub const MAX: P = P { id: "SYSTEM_MEMEXT_MAX", block: &BLOCK, off: 1, enc: Enc::I7, ofs: -1, min: 1, max: 128, init: 128 };
}

/// `SYSTEM_USB` at `10 01 10 00`.
pub mod system_usb {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_USB", addr: 0x10011000, len: 19 };
    pub const MODE: P = P { id: "SYSTEM_USB_MODE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: -1, min: 1, max: 2, init: 1 };
    pub const VENDOR_DRY_OUT_LEVEL: P = P { id: "SYSTEM_USB_VENDOR_DRY_OUT_LEVEL", block: &BLOCK, off: 1, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 100 };
    pub const VENDOR_EFX_IN_LEVEL: P = P { id: "SYSTEM_USB_VENDOR_EFX_IN_LEVEL", block: &BLOCK, off: 3, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 100 };
    pub const VENDOR_EFX_OUT_LEVEL: P = P { id: "SYSTEM_USB_VENDOR_EFX_OUT_LEVEL", block: &BLOCK, off: 5, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 100 };
    pub const VENDOR_MIX_IN_LEVEL: P = P { id: "SYSTEM_USB_VENDOR_MIX_IN_LEVEL", block: &BLOCK, off: 7, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 50 };
    pub const GENERIC_EFX_OUT_LEVEL: P = P { id: "SYSTEM_USB_GENERIC_EFX_OUT_LEVEL", block: &BLOCK, off: 9, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 100 };
    pub const GENERIC_MIX_IN_LEVEL: P = P { id: "SYSTEM_USB_GENERIC_MIX_IN_LEVEL", block: &BLOCK, off: 11, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 50 };
    pub const VENDOR_LOOPBACK_SW: P = P { id: "SYSTEM_USB_VENDOR_LOOPBACK_SW", block: &BLOCK, off: 13, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const VENDOR_LOOPBACK_LEVEL: P = P { id: "SYSTEM_USB_VENDOR_LOOPBACK_LEVEL", block: &BLOCK, off: 14, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 100 };
    pub const GENERIC_LOOPBACK_SW: P = P { id: "SYSTEM_USB_GENERIC_LOOPBACK_SW", block: &BLOCK, off: 16, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const GENERIC_LOOPBACK_LEVEL: P = P { id: "SYSTEM_USB_GENERIC_LOOPBACK_LEVEL", block: &BLOCK, off: 17, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 100 };
}

/// `SYSTEM_MIDI` at `10 01 20 00`.
pub mod system_midi {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_MIDI", addr: 0x10012000, len: 18 };
    pub const MIDI_THRU: P = P { id: "SYSTEM_MIDI_MIDI_THRU", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 3 };
    pub const USB_THRU: P = P { id: "SYSTEM_MIDI_USB_THRU", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 3 };
    pub const RX_CH: P = P { id: "SYSTEM_MIDI_RX_CH", block: &BLOCK, off: 2, enc: Enc::I7, ofs: -1, min: 1, max: 17, init: 1 };
    pub const TX_CH: P = P { id: "SYSTEM_MIDI_TX_CH", block: &BLOCK, off: 3, enc: Enc::I7, ofs: -1, min: 1, max: 18, init: 17 };
    pub const RX_PC: P = P { id: "SYSTEM_MIDI_RX_PC", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
    pub const TX_PC: P = P { id: "SYSTEM_MIDI_TX_PC", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
    pub const RX_CC: P = P { id: "SYSTEM_MIDI_RX_CC", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
    pub const TX_CC: P = P { id: "SYSTEM_MIDI_TX_CC", block: &BLOCK, off: 7, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const CC_MEMORY_DOWN: P = P { id: "SYSTEM_MIDI_CC_MEMORY_DOWN", block: &BLOCK, off: 8, enc: Enc::I7, ofs: 0, min: 0, max: 63, init: 50 };
    pub const CC_MEMORY_UP: P = P { id: "SYSTEM_MIDI_CC_MEMORY_UP", block: &BLOCK, off: 9, enc: Enc::I7, ofs: 0, min: 0, max: 63, init: 51 };
    pub const CC_AMP_GAIN: P = P { id: "SYSTEM_MIDI_CC_AMP_GAIN", block: &BLOCK, off: 10, enc: Enc::I7, ofs: 0, min: 0, max: 63, init: 17 };
    pub const CC_AMP_LEVEL: P = P { id: "SYSTEM_MIDI_CC_AMP_LEVEL", block: &BLOCK, off: 11, enc: Enc::I7, ofs: 0, min: 0, max: 63, init: 18 };
    pub const CC_AMP_SOLO_SW: P = P { id: "SYSTEM_MIDI_CC_AMP_SOLO_SW", block: &BLOCK, off: 12, enc: Enc::I7, ofs: 0, min: 0, max: 63, init: 52 };
    pub const CC_AMP_SOLO_LEVEL: P = P { id: "SYSTEM_MIDI_CC_AMP_SOLO_LEVEL", block: &BLOCK, off: 13, enc: Enc::I7, ofs: 0, min: 0, max: 63, init: 19 };
    pub const CC_CTL1: P = P { id: "SYSTEM_MIDI_CC_CTL1", block: &BLOCK, off: 14, enc: Enc::I7, ofs: 0, min: 0, max: 63, init: 48 };
    pub const CC_CTL2: P = P { id: "SYSTEM_MIDI_CC_CTL2", block: &BLOCK, off: 15, enc: Enc::I7, ofs: 0, min: 0, max: 63, init: 49 };
    pub const CC_EXP: P = P { id: "SYSTEM_MIDI_CC_EXP", block: &BLOCK, off: 16, enc: Enc::I7, ofs: 0, min: 0, max: 63, init: 16 };
    pub const CC_BYPASS: P = P { id: "SYSTEM_MIDI_CC_BYPASS", block: &BLOCK, off: 17, enc: Enc::I7, ofs: 0, min: 0, max: 63, init: 27 };
}

/// `SYSTEM_PREF` at `10 01 30 00`.
pub mod system_pref {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_PREF", addr: 0x10013000, len: 7 };
    pub const AUTO_OFF: P = P { id: "SYSTEM_PREF_AUTO_OFF", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
    pub const LCD_CONTRAST: P = P { id: "SYSTEM_PREF_LCD_CONTRAST", block: &BLOCK, off: 1, enc: Enc::I7, ofs: -1, min: 1, max: 16, init: 9 };
    pub const DISP_MODE: P = P { id: "SYSTEM_PREF_DISP_MODE", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PANEL_LOCK: P = P { id: "SYSTEM_PREF_PANEL_LOCK", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const DISP_CAB: P = P { id: "SYSTEM_PREF_DISP_CAB", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PLAY_CURSOR: P = P { id: "SYSTEM_PREF_PLAY_CURSOR", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const EXP_PDL_HOLD: P = P { id: "SYSTEM_PREF_EXP_PDL_HOLD", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
}

/// `SYSTEM_INPUT` at `10 01 40 00`.
pub mod system_input {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_INPUT", addr: 0x10014000, len: 11 };
    pub const PREF: P = P { id: "SYSTEM_INPUT_PREF", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 10, init: 0 };
    pub const SETTING1: P = P { id: "SYSTEM_INPUT_SETTING1", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const SETTING2: P = P { id: "SYSTEM_INPUT_SETTING2", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const SETTING3: P = P { id: "SYSTEM_INPUT_SETTING3", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const SETTING4: P = P { id: "SYSTEM_INPUT_SETTING4", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const SETTING5: P = P { id: "SYSTEM_INPUT_SETTING5", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const SETTING6: P = P { id: "SYSTEM_INPUT_SETTING6", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const SETTING7: P = P { id: "SYSTEM_INPUT_SETTING7", block: &BLOCK, off: 7, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const SETTING8: P = P { id: "SYSTEM_INPUT_SETTING8", block: &BLOCK, off: 8, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const SETTING9: P = P { id: "SYSTEM_INPUT_SETTING9", block: &BLOCK, off: 9, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const SETTING10: P = P { id: "SYSTEM_INPUT_SETTING10", block: &BLOCK, off: 10, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
}

/// `SYSTEM_CTRL_PREF` at `10 01 50 00`.
pub mod system_ctrl_pref {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_CTRL_PREF", addr: 0x10015000, len: 5 };
    pub const FSW_DOWN: P = P { id: "SYSTEM_CTRL_PREF_FSW_DOWN", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const FSW_UP: P = P { id: "SYSTEM_CTRL_PREF_FSW_UP", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const CTL1: P = P { id: "SYSTEM_CTRL_PREF_CTL1", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const CTL2: P = P { id: "SYSTEM_CTRL_PREF_CTL2", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const EXP: P = P { id: "SYSTEM_CTRL_PREF_EXP", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
}

/// `SYSTEM_CTRL_FSW_DOWN` at `10 01 60 00`.
pub mod system_ctrl_fsw_down {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_CTRL_FSW_DOWN", addr: 0x10016000, len: 6 };
    pub const FUNC_TYPE: P = P { id: "SYSTEM_CTRL_FSW_DOWN_FUNC_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 0 };
    pub const PRM_MEMORY: P = P { id: "SYSTEM_CTRL_FSW_DOWN_PRM_MEMORY", block: &BLOCK, off: 1, enc: Enc::N2, ofs: 0, min: 0, max: 129, init: 0 };
    pub const PRM_AMP_SOLO: P = P { id: "SYSTEM_CTRL_FSW_DOWN_PRM_AMP_SOLO", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PRM_EQ: P = P { id: "SYSTEM_CTRL_FSW_DOWN_PRM_EQ", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 2, init: 0 };
    pub const PRM_SNDRTN: P = P { id: "SYSTEM_CTRL_FSW_DOWN_PRM_SNDRTN", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 0, init: 0 };
}

/// `SYSTEM_CTRL_FSW_UP` at `10 01 70 00`.
pub mod system_ctrl_fsw_up {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_CTRL_FSW_UP", addr: 0x10017000, len: 6 };
    pub const FUNC_TYPE: P = P { id: "SYSTEM_CTRL_FSW_UP_FUNC_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 0 };
    pub const PRM_MEMORY: P = P { id: "SYSTEM_CTRL_FSW_UP_PRM_MEMORY", block: &BLOCK, off: 1, enc: Enc::N2, ofs: 0, min: 0, max: 129, init: 0 };
    pub const PRM_AMP_SOLO: P = P { id: "SYSTEM_CTRL_FSW_UP_PRM_AMP_SOLO", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PRM_EQ: P = P { id: "SYSTEM_CTRL_FSW_UP_PRM_EQ", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 2, init: 0 };
    pub const PRM_SNDRTN: P = P { id: "SYSTEM_CTRL_FSW_UP_PRM_SNDRTN", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 0, init: 0 };
}

/// `SYSTEM_CTRL_CTL1` at `10 02 00 00`.
pub mod system_ctrl_ctl1 {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_CTRL_CTL1", addr: 0x10020000, len: 6 };
    pub const FUNC_TYPE: P = P { id: "SYSTEM_CTRL_CTL1_FUNC_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 0 };
    pub const PRM_MEMORY: P = P { id: "SYSTEM_CTRL_CTL1_PRM_MEMORY", block: &BLOCK, off: 1, enc: Enc::N2, ofs: 0, min: 0, max: 129, init: 0 };
    pub const PRM_AMP_SOLO: P = P { id: "SYSTEM_CTRL_CTL1_PRM_AMP_SOLO", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PRM_EQ: P = P { id: "SYSTEM_CTRL_CTL1_PRM_EQ", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 2, init: 0 };
    pub const PRM_SNDRTN: P = P { id: "SYSTEM_CTRL_CTL1_PRM_SNDRTN", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 0, init: 0 };
}

/// `SYSTEM_CTRL_CTL2` at `10 02 10 00`.
pub mod system_ctrl_ctl2 {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_CTRL_CTL2", addr: 0x10021000, len: 6 };
    pub const FUNC_TYPE: P = P { id: "SYSTEM_CTRL_CTL2_FUNC_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 0 };
    pub const PRM_MEMORY: P = P { id: "SYSTEM_CTRL_CTL2_PRM_MEMORY", block: &BLOCK, off: 1, enc: Enc::N2, ofs: 0, min: 0, max: 129, init: 0 };
    pub const PRM_AMP_SOLO: P = P { id: "SYSTEM_CTRL_CTL2_PRM_AMP_SOLO", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PRM_EQ: P = P { id: "SYSTEM_CTRL_CTL2_PRM_EQ", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 2, init: 0 };
    pub const PRM_SNDRTN: P = P { id: "SYSTEM_CTRL_CTL2_PRM_SNDRTN", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 0, init: 0 };
}

/// `SYSTEM_CTRL_EXP` at `10 02 20 00`.
pub mod system_ctrl_exp {
    use super::*;
    pub const BLOCK: Block = Block { name: "SYSTEM_CTRL_EXP", addr: 0x10022000, len: 15 };
    pub const FUNC_TYPE: P = P { id: "SYSTEM_CTRL_EXP_FUNC_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 5, init: 1 };
    pub const AMP_GAIN_MIN: P = P { id: "SYSTEM_CTRL_EXP_AMP_GAIN_MIN", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 120, init: 0 };
    pub const AMP_GAIN_MAX: P = P { id: "SYSTEM_CTRL_EXP_AMP_GAIN_MAX", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 0, min: 0, max: 120, init: 120 };
    pub const AMP_LEVEL_MIN: P = P { id: "SYSTEM_CTRL_EXP_AMP_LEVEL_MIN", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 0 };
    pub const AMP_LEVEL_MAX: P = P { id: "SYSTEM_CTRL_EXP_AMP_LEVEL_MAX", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 100 };
    pub const AMP_SOLO_MIN: P = P { id: "SYSTEM_CTRL_EXP_AMP_SOLO_MIN", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 0 };
    pub const AMP_SOLO_MAX: P = P { id: "SYSTEM_CTRL_EXP_AMP_SOLO_MAX", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 100 };
    pub const SEND_LEVEL_MIN: P = P { id: "SYSTEM_CTRL_EXP_SEND_LEVEL_MIN", block: &BLOCK, off: 7, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 0 };
    pub const SEND_LEVEL_MAX: P = P { id: "SYSTEM_CTRL_EXP_SEND_LEVEL_MAX", block: &BLOCK, off: 9, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 200 };
    pub const RTN_LEVEL_MIN: P = P { id: "SYSTEM_CTRL_EXP_RTN_LEVEL_MIN", block: &BLOCK, off: 11, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 0 };
    pub const RTN_LEVEL_MAX: P = P { id: "SYSTEM_CTRL_EXP_RTN_LEVEL_MAX", block: &BLOCK, off: 13, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 200 };
}

/// `PATCH_COM` at `20 00 00 00`.
pub mod patch_com {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_COM", addr: 0x20000000, len: 16 };
    /// Text, 16 chars. IR Loader init: 'INIT MEMORY     '
    pub const NAME: P = P { id: "PATCH_COM_NAME", block: &BLOCK, off: 0, enc: Enc::Text(16), ofs: 0, min: 32, max: 127, init: 32 };
}

/// `PATCH_AMP` at `20 00 10 00`.
pub mod patch_amp {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_AMP", addr: 0x20001000, len: 9 };
    pub const TYPE: P = P { id: "PATCH_AMP_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 11, init: 1 };
    pub const GAIN: P = P { id: "PATCH_AMP_GAIN", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 120, init: 50 };
    pub const LEVEL: P = P { id: "PATCH_AMP_LEVEL", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 50 };
    pub const BASS: P = P { id: "PATCH_AMP_BASS", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 50 };
    pub const MIDDLE: P = P { id: "PATCH_AMP_MIDDLE", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 50 };
    pub const TREBLE: P = P { id: "PATCH_AMP_TREBLE", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 50 };
    pub const GAIN_SW: P = P { id: "PATCH_AMP_GAIN_SW", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 0, min: 0, max: 2, init: 1 };
    pub const SOLO_SW: P = P { id: "PATCH_AMP_SOLO_SW", block: &BLOCK, off: 7, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const SOLO_LEVEL: P = P { id: "PATCH_AMP_SOLO_LEVEL", block: &BLOCK, off: 8, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 50 };
}

/// `PATCH_NS` at `20 00 20 00`.
pub mod patch_ns {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_NS", addr: 0x20002000, len: 2 };
    pub const THRESHOLD: P = P { id: "PATCH_NS_THRESHOLD", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 30 };
    pub const RELEASE: P = P { id: "PATCH_NS_RELEASE", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 30 };
}

/// `PATCH_SR` at `20 00 30 00`.
pub mod patch_sr {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_SR", addr: 0x20003000, len: 8 };
    pub const SW: P = P { id: "PATCH_SR_SW", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const MODE: P = P { id: "PATCH_SR_MODE", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 0 };
    pub const SEND_LEVEL: P = P { id: "PATCH_SR_SEND_LEVEL", block: &BLOCK, off: 2, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 100 };
    pub const RETURN_LEVEL: P = P { id: "PATCH_SR_RETURN_LEVEL", block: &BLOCK, off: 4, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 100 };
    pub const ADJUST: P = P { id: "PATCH_SR_ADJUST", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 0 };
    pub const POS: P = P { id: "PATCH_SR_POS", block: &BLOCK, off: 7, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
}

/// `PATCH_CAB` at `20 00 40 00`.
pub mod patch_cab {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_CAB", addr: 0x20004000, len: 3 };
    pub const LINK: P = P { id: "PATCH_CAB_LINK", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
    pub const OUT_MODE: P = P { id: "PATCH_CAB_OUT_MODE", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
    pub const ADJUST: P = P { id: "PATCH_CAB_ADJUST", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 0 };
}

/// `PATCH_CAB_A` at `20 00 50 00`.
pub mod patch_cab_a {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_CAB_A", addr: 0x20005000, len: 6 };
    pub const CAB_TYPE: P = P { id: "PATCH_CAB_A_CAB_TYPE", block: &BLOCK, off: 0, enc: Enc::N4, ofs: 0, min: 0, max: 336, init: 1 };
    pub const MIC_LEVEL: P = P { id: "PATCH_CAB_A_MIC_LEVEL", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 100 };
    pub const DIR_MIX: P = P { id: "PATCH_CAB_A_DIR_MIX", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 0 };
}

/// `PATCH_CAB_B` at `20 00 60 00`.
pub mod patch_cab_b {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_CAB_B", addr: 0x20006000, len: 6 };
    pub const CAB_TYPE: P = P { id: "PATCH_CAB_B_CAB_TYPE", block: &BLOCK, off: 0, enc: Enc::N4, ofs: 0, min: 0, max: 336, init: 1 };
    pub const MIC_LEVEL: P = P { id: "PATCH_CAB_B_MIC_LEVEL", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 100 };
    pub const DIR_MIX: P = P { id: "PATCH_CAB_B_DIR_MIX", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 0 };
}

/// `PATCH_AMB` at `20 00 70 00`.
pub mod patch_amb {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_AMB", addr: 0x20007000, len: 5 };
    pub const MODE: P = P { id: "PATCH_AMB_MODE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const TYPE: P = P { id: "PATCH_AMB_TYPE", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 3 };
    pub const TIME: P = P { id: "PATCH_AMB_TIME", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 0, min: 1, max: 100, init: 26 };
    pub const TONE: P = P { id: "PATCH_AMB_TONE", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 50, min: -50, max: 50, init: 0 };
    pub const EFX_LEVEL: P = P { id: "PATCH_AMB_EFX_LEVEL", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 25 };
}

/// `PATCH_EQ` at `20 01 00 00`.
pub mod patch_eq {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_EQ", addr: 0x20010000, len: 1 };
    pub const LINK: P = P { id: "PATCH_EQ_LINK", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 1 };
}

/// `PATCH_EQ_A` at `20 01 10 00`.
pub mod patch_eq_a {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_EQ_A", addr: 0x20011000, len: 22 };
    pub const SW: P = P { id: "PATCH_EQ_A_SW", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const TYPE: P = P { id: "PATCH_EQ_A_TYPE", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PEQ_LOW_GAIN: P = P { id: "PATCH_EQ_A_PEQ_LOW_GAIN", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_HIGH_GAIN: P = P { id: "PATCH_EQ_A_PEQ_HIGH_GAIN", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_LEVEL: P = P { id: "PATCH_EQ_A_PEQ_LEVEL", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_LOMID_FREQ: P = P { id: "PATCH_EQ_A_PEQ_LOMID_FREQ", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 29, init: 14 };
    pub const PEQ_LOMID_Q: P = P { id: "PATCH_EQ_A_PEQ_LOMID_Q", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 0, min: 0, max: 5, init: 1 };
    pub const PEQ_LOMID_GAIN: P = P { id: "PATCH_EQ_A_PEQ_LOMID_GAIN", block: &BLOCK, off: 7, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_HIMID_FREQ: P = P { id: "PATCH_EQ_A_PEQ_HIMID_FREQ", block: &BLOCK, off: 8, enc: Enc::I7, ofs: 0, min: 0, max: 29, init: 23 };
    pub const PEQ_HIMID_Q: P = P { id: "PATCH_EQ_A_PEQ_HIMID_Q", block: &BLOCK, off: 9, enc: Enc::I7, ofs: 0, min: 0, max: 5, init: 1 };
    pub const PEQ_HIMID_GAIN: P = P { id: "PATCH_EQ_A_PEQ_HIMID_GAIN", block: &BLOCK, off: 10, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_LEVEL: P = P { id: "PATCH_EQ_A_GEQ_LEVEL", block: &BLOCK, off: 11, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_31HZ_GAIN: P = P { id: "PATCH_EQ_A_GEQ_31Hz_GAIN", block: &BLOCK, off: 12, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_63HZ_GAIN: P = P { id: "PATCH_EQ_A_GEQ_63Hz_GAIN", block: &BLOCK, off: 13, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_125HZ_GAIN: P = P { id: "PATCH_EQ_A_GEQ_125Hz_GAIN", block: &BLOCK, off: 14, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_250HZ_GAIN: P = P { id: "PATCH_EQ_A_GEQ_250Hz_GAIN", block: &BLOCK, off: 15, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_500HZ_GAIN: P = P { id: "PATCH_EQ_A_GEQ_500Hz_GAIN", block: &BLOCK, off: 16, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_1KHZ_GAIN: P = P { id: "PATCH_EQ_A_GEQ_1kHz_GAIN", block: &BLOCK, off: 17, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_2KHZ_GAIN: P = P { id: "PATCH_EQ_A_GEQ_2kHz_GAIN", block: &BLOCK, off: 18, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_4KHZ_GAIN: P = P { id: "PATCH_EQ_A_GEQ_4kHz_GAIN", block: &BLOCK, off: 19, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_8KHZ_GAIN: P = P { id: "PATCH_EQ_A_GEQ_8kHz_GAIN", block: &BLOCK, off: 20, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_16KHZ_GAIN: P = P { id: "PATCH_EQ_A_GEQ_16kHz_GAIN", block: &BLOCK, off: 21, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
}

/// `PATCH_EQ_B` at `20 01 20 00`.
pub mod patch_eq_b {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_EQ_B", addr: 0x20012000, len: 22 };
    pub const SW: P = P { id: "PATCH_EQ_B_SW", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const TYPE: P = P { id: "PATCH_EQ_B_TYPE", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PEQ_LOW_GAIN: P = P { id: "PATCH_EQ_B_PEQ_LOW_GAIN", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_HIGH_GAIN: P = P { id: "PATCH_EQ_B_PEQ_HIGH_GAIN", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_LEVEL: P = P { id: "PATCH_EQ_B_PEQ_LEVEL", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_LOMID_FREQ: P = P { id: "PATCH_EQ_B_PEQ_LOMID_FREQ", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 29, init: 14 };
    pub const PEQ_LOMID_Q: P = P { id: "PATCH_EQ_B_PEQ_LOMID_Q", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 0, min: 0, max: 5, init: 1 };
    pub const PEQ_LOMID_GAIN: P = P { id: "PATCH_EQ_B_PEQ_LOMID_GAIN", block: &BLOCK, off: 7, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const PEQ_HIMID_FREQ: P = P { id: "PATCH_EQ_B_PEQ_HIMID_FREQ", block: &BLOCK, off: 8, enc: Enc::I7, ofs: 0, min: 0, max: 29, init: 23 };
    pub const PEQ_HIMID_Q: P = P { id: "PATCH_EQ_B_PEQ_HIMID_Q", block: &BLOCK, off: 9, enc: Enc::I7, ofs: 0, min: 0, max: 5, init: 1 };
    pub const PEQ_HIMID_GAIN: P = P { id: "PATCH_EQ_B_PEQ_HIMID_GAIN", block: &BLOCK, off: 10, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_LEVEL: P = P { id: "PATCH_EQ_B_GEQ_LEVEL", block: &BLOCK, off: 11, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_31HZ_GAIN: P = P { id: "PATCH_EQ_B_GEQ_31Hz_GAIN", block: &BLOCK, off: 12, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_63HZ_GAIN: P = P { id: "PATCH_EQ_B_GEQ_63Hz_GAIN", block: &BLOCK, off: 13, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_125HZ_GAIN: P = P { id: "PATCH_EQ_B_GEQ_125Hz_GAIN", block: &BLOCK, off: 14, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_250HZ_GAIN: P = P { id: "PATCH_EQ_B_GEQ_250Hz_GAIN", block: &BLOCK, off: 15, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_500HZ_GAIN: P = P { id: "PATCH_EQ_B_GEQ_500Hz_GAIN", block: &BLOCK, off: 16, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_1KHZ_GAIN: P = P { id: "PATCH_EQ_B_GEQ_1kHz_GAIN", block: &BLOCK, off: 17, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_2KHZ_GAIN: P = P { id: "PATCH_EQ_B_GEQ_2kHz_GAIN", block: &BLOCK, off: 18, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_4KHZ_GAIN: P = P { id: "PATCH_EQ_B_GEQ_4kHz_GAIN", block: &BLOCK, off: 19, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_8KHZ_GAIN: P = P { id: "PATCH_EQ_B_GEQ_8kHz_GAIN", block: &BLOCK, off: 20, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
    pub const GEQ_16KHZ_GAIN: P = P { id: "PATCH_EQ_B_GEQ_16kHz_GAIN", block: &BLOCK, off: 21, enc: Enc::I7, ofs: 20, min: -20, max: 20, init: 0 };
}

/// `PATCH_INPUT` at `20 01 30 00`.
pub mod patch_input {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_INPUT", addr: 0x20013000, len: 1 };
    pub const PREF: P = P { id: "PATCH_INPUT_PREF", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 1, max: 10, init: 1 };
}

/// `PATCH_CTRL_FSW_DOWN` at `20 01 40 00`.
pub mod patch_ctrl_fsw_down {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_CTRL_FSW_DOWN", addr: 0x20014000, len: 6 };
    pub const FUNC_TYPE: P = P { id: "PATCH_CTRL_FSW_DOWN_FUNC_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 0 };
    pub const PRM_MEMORY: P = P { id: "PATCH_CTRL_FSW_DOWN_PRM_MEMORY", block: &BLOCK, off: 1, enc: Enc::N2, ofs: 0, min: 0, max: 129, init: 0 };
    pub const PRM_AMP_SOLO: P = P { id: "PATCH_CTRL_FSW_DOWN_PRM_AMP_SOLO", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PRM_EQ: P = P { id: "PATCH_CTRL_FSW_DOWN_PRM_EQ", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 2, init: 0 };
    pub const PRM_SNDRTN: P = P { id: "PATCH_CTRL_FSW_DOWN_PRM_SNDRTN", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 0, init: 0 };
}

/// `PATCH_CTRL_FSW_UP` at `20 01 50 00`.
pub mod patch_ctrl_fsw_up {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_CTRL_FSW_UP", addr: 0x20015000, len: 6 };
    pub const FUNC_TYPE: P = P { id: "PATCH_CTRL_FSW_UP_FUNC_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 0 };
    pub const PRM_MEMORY: P = P { id: "PATCH_CTRL_FSW_UP_PRM_MEMORY", block: &BLOCK, off: 1, enc: Enc::N2, ofs: 0, min: 0, max: 129, init: 0 };
    pub const PRM_AMP_SOLO: P = P { id: "PATCH_CTRL_FSW_UP_PRM_AMP_SOLO", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PRM_EQ: P = P { id: "PATCH_CTRL_FSW_UP_PRM_EQ", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 2, init: 0 };
    pub const PRM_SNDRTN: P = P { id: "PATCH_CTRL_FSW_UP_PRM_SNDRTN", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 0, init: 0 };
}

/// `PATCH_CTRL_CTL1` at `20 01 60 00`.
pub mod patch_ctrl_ctl1 {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_CTRL_CTL1", addr: 0x20016000, len: 6 };
    pub const FUNC_TYPE: P = P { id: "PATCH_CTRL_CTL1_FUNC_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 0 };
    pub const PRM_MEMORY: P = P { id: "PATCH_CTRL_CTL1_PRM_MEMORY", block: &BLOCK, off: 1, enc: Enc::N2, ofs: 0, min: 0, max: 129, init: 0 };
    pub const PRM_AMP_SOLO: P = P { id: "PATCH_CTRL_CTL1_PRM_AMP_SOLO", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PRM_EQ: P = P { id: "PATCH_CTRL_CTL1_PRM_EQ", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 2, init: 0 };
    pub const PRM_SNDRTN: P = P { id: "PATCH_CTRL_CTL1_PRM_SNDRTN", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 0, init: 0 };
}

/// `PATCH_CTRL_CTL2` at `20 01 70 00`.
pub mod patch_ctrl_ctl2 {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_CTRL_CTL2", addr: 0x20017000, len: 6 };
    pub const FUNC_TYPE: P = P { id: "PATCH_CTRL_CTL2_FUNC_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 3, init: 0 };
    pub const PRM_MEMORY: P = P { id: "PATCH_CTRL_CTL2_PRM_MEMORY", block: &BLOCK, off: 1, enc: Enc::N2, ofs: 0, min: 0, max: 129, init: 0 };
    pub const PRM_AMP_SOLO: P = P { id: "PATCH_CTRL_CTL2_PRM_AMP_SOLO", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 1, init: 0 };
    pub const PRM_EQ: P = P { id: "PATCH_CTRL_CTL2_PRM_EQ", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 2, init: 0 };
    pub const PRM_SNDRTN: P = P { id: "PATCH_CTRL_CTL2_PRM_SNDRTN", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 0, init: 0 };
}

/// `PATCH_CTRL_EXP` at `20 02 00 00`.
pub mod patch_ctrl_exp {
    use super::*;
    pub const BLOCK: Block = Block { name: "PATCH_CTRL_EXP", addr: 0x20020000, len: 15 };
    pub const FUNC_TYPE: P = P { id: "PATCH_CTRL_EXP_FUNC_TYPE", block: &BLOCK, off: 0, enc: Enc::I7, ofs: 0, min: 0, max: 5, init: 1 };
    pub const AMP_GAIN_MIN: P = P { id: "PATCH_CTRL_EXP_AMP_GAIN_MIN", block: &BLOCK, off: 1, enc: Enc::I7, ofs: 0, min: 0, max: 120, init: 0 };
    pub const AMP_GAIN_MAX: P = P { id: "PATCH_CTRL_EXP_AMP_GAIN_MAX", block: &BLOCK, off: 2, enc: Enc::I7, ofs: 0, min: 0, max: 120, init: 120 };
    pub const AMP_LEVEL_MIN: P = P { id: "PATCH_CTRL_EXP_AMP_LEVEL_MIN", block: &BLOCK, off: 3, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 0 };
    pub const AMP_LEVEL_MAX: P = P { id: "PATCH_CTRL_EXP_AMP_LEVEL_MAX", block: &BLOCK, off: 4, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 100 };
    pub const AMP_SOLO_MIN: P = P { id: "PATCH_CTRL_EXP_AMP_SOLO_MIN", block: &BLOCK, off: 5, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 0 };
    pub const AMP_SOLO_MAX: P = P { id: "PATCH_CTRL_EXP_AMP_SOLO_MAX", block: &BLOCK, off: 6, enc: Enc::I7, ofs: 0, min: 0, max: 100, init: 100 };
    pub const SEND_LEVEL_MIN: P = P { id: "PATCH_CTRL_EXP_SEND_LEVEL_MIN", block: &BLOCK, off: 7, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 0 };
    pub const SEND_LEVEL_MAX: P = P { id: "PATCH_CTRL_EXP_SEND_LEVEL_MAX", block: &BLOCK, off: 9, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 200 };
    pub const RTN_LEVEL_MIN: P = P { id: "PATCH_CTRL_EXP_RTN_LEVEL_MIN", block: &BLOCK, off: 11, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 0 };
    pub const RTN_LEVEL_MAX: P = P { id: "PATCH_CTRL_EXP_RTN_LEVEL_MAX", block: &BLOCK, off: 13, enc: Enc::N2, ofs: 0, min: 0, max: 200, init: 200 };
}

/// `MIDI_PCMAP` at `50 00 00 00`.
pub mod midi_pcmap {
    use super::*;
    pub const BLOCK: Block = Block { name: "MIDI_PCMAP", addr: 0x50000000, len: 256 };
    pub const PC1: P = P { id: "MIDI_PCMAP_PC1", block: &BLOCK, off: 0, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC2: P = P { id: "MIDI_PCMAP_PC2", block: &BLOCK, off: 2, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC3: P = P { id: "MIDI_PCMAP_PC3", block: &BLOCK, off: 4, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC4: P = P { id: "MIDI_PCMAP_PC4", block: &BLOCK, off: 6, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC5: P = P { id: "MIDI_PCMAP_PC5", block: &BLOCK, off: 8, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC6: P = P { id: "MIDI_PCMAP_PC6", block: &BLOCK, off: 10, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC7: P = P { id: "MIDI_PCMAP_PC7", block: &BLOCK, off: 12, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC8: P = P { id: "MIDI_PCMAP_PC8", block: &BLOCK, off: 14, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC9: P = P { id: "MIDI_PCMAP_PC9", block: &BLOCK, off: 16, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC10: P = P { id: "MIDI_PCMAP_PC10", block: &BLOCK, off: 18, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC11: P = P { id: "MIDI_PCMAP_PC11", block: &BLOCK, off: 20, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC12: P = P { id: "MIDI_PCMAP_PC12", block: &BLOCK, off: 22, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC13: P = P { id: "MIDI_PCMAP_PC13", block: &BLOCK, off: 24, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC14: P = P { id: "MIDI_PCMAP_PC14", block: &BLOCK, off: 26, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC15: P = P { id: "MIDI_PCMAP_PC15", block: &BLOCK, off: 28, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC16: P = P { id: "MIDI_PCMAP_PC16", block: &BLOCK, off: 30, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC17: P = P { id: "MIDI_PCMAP_PC17", block: &BLOCK, off: 32, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC18: P = P { id: "MIDI_PCMAP_PC18", block: &BLOCK, off: 34, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC19: P = P { id: "MIDI_PCMAP_PC19", block: &BLOCK, off: 36, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC20: P = P { id: "MIDI_PCMAP_PC20", block: &BLOCK, off: 38, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC21: P = P { id: "MIDI_PCMAP_PC21", block: &BLOCK, off: 40, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC22: P = P { id: "MIDI_PCMAP_PC22", block: &BLOCK, off: 42, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC23: P = P { id: "MIDI_PCMAP_PC23", block: &BLOCK, off: 44, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC24: P = P { id: "MIDI_PCMAP_PC24", block: &BLOCK, off: 46, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC25: P = P { id: "MIDI_PCMAP_PC25", block: &BLOCK, off: 48, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC26: P = P { id: "MIDI_PCMAP_PC26", block: &BLOCK, off: 50, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC27: P = P { id: "MIDI_PCMAP_PC27", block: &BLOCK, off: 52, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC28: P = P { id: "MIDI_PCMAP_PC28", block: &BLOCK, off: 54, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC29: P = P { id: "MIDI_PCMAP_PC29", block: &BLOCK, off: 56, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC30: P = P { id: "MIDI_PCMAP_PC30", block: &BLOCK, off: 58, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC31: P = P { id: "MIDI_PCMAP_PC31", block: &BLOCK, off: 60, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC32: P = P { id: "MIDI_PCMAP_PC32", block: &BLOCK, off: 62, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC33: P = P { id: "MIDI_PCMAP_PC33", block: &BLOCK, off: 64, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC34: P = P { id: "MIDI_PCMAP_PC34", block: &BLOCK, off: 66, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC35: P = P { id: "MIDI_PCMAP_PC35", block: &BLOCK, off: 68, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC36: P = P { id: "MIDI_PCMAP_PC36", block: &BLOCK, off: 70, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC37: P = P { id: "MIDI_PCMAP_PC37", block: &BLOCK, off: 72, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC38: P = P { id: "MIDI_PCMAP_PC38", block: &BLOCK, off: 74, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC39: P = P { id: "MIDI_PCMAP_PC39", block: &BLOCK, off: 76, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC40: P = P { id: "MIDI_PCMAP_PC40", block: &BLOCK, off: 78, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC41: P = P { id: "MIDI_PCMAP_PC41", block: &BLOCK, off: 80, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC42: P = P { id: "MIDI_PCMAP_PC42", block: &BLOCK, off: 82, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC43: P = P { id: "MIDI_PCMAP_PC43", block: &BLOCK, off: 84, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC44: P = P { id: "MIDI_PCMAP_PC44", block: &BLOCK, off: 86, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC45: P = P { id: "MIDI_PCMAP_PC45", block: &BLOCK, off: 88, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC46: P = P { id: "MIDI_PCMAP_PC46", block: &BLOCK, off: 90, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC47: P = P { id: "MIDI_PCMAP_PC47", block: &BLOCK, off: 92, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC48: P = P { id: "MIDI_PCMAP_PC48", block: &BLOCK, off: 94, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC49: P = P { id: "MIDI_PCMAP_PC49", block: &BLOCK, off: 96, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC50: P = P { id: "MIDI_PCMAP_PC50", block: &BLOCK, off: 98, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC51: P = P { id: "MIDI_PCMAP_PC51", block: &BLOCK, off: 100, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC52: P = P { id: "MIDI_PCMAP_PC52", block: &BLOCK, off: 102, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC53: P = P { id: "MIDI_PCMAP_PC53", block: &BLOCK, off: 104, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC54: P = P { id: "MIDI_PCMAP_PC54", block: &BLOCK, off: 106, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC55: P = P { id: "MIDI_PCMAP_PC55", block: &BLOCK, off: 108, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC56: P = P { id: "MIDI_PCMAP_PC56", block: &BLOCK, off: 110, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC57: P = P { id: "MIDI_PCMAP_PC57", block: &BLOCK, off: 112, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC58: P = P { id: "MIDI_PCMAP_PC58", block: &BLOCK, off: 114, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC59: P = P { id: "MIDI_PCMAP_PC59", block: &BLOCK, off: 116, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC60: P = P { id: "MIDI_PCMAP_PC60", block: &BLOCK, off: 118, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC61: P = P { id: "MIDI_PCMAP_PC61", block: &BLOCK, off: 120, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC62: P = P { id: "MIDI_PCMAP_PC62", block: &BLOCK, off: 122, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC63: P = P { id: "MIDI_PCMAP_PC63", block: &BLOCK, off: 124, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC64: P = P { id: "MIDI_PCMAP_PC64", block: &BLOCK, off: 126, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC65: P = P { id: "MIDI_PCMAP_PC65", block: &BLOCK, off: 128, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC66: P = P { id: "MIDI_PCMAP_PC66", block: &BLOCK, off: 130, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC67: P = P { id: "MIDI_PCMAP_PC67", block: &BLOCK, off: 132, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC68: P = P { id: "MIDI_PCMAP_PC68", block: &BLOCK, off: 134, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC69: P = P { id: "MIDI_PCMAP_PC69", block: &BLOCK, off: 136, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC70: P = P { id: "MIDI_PCMAP_PC70", block: &BLOCK, off: 138, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC71: P = P { id: "MIDI_PCMAP_PC71", block: &BLOCK, off: 140, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC72: P = P { id: "MIDI_PCMAP_PC72", block: &BLOCK, off: 142, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC73: P = P { id: "MIDI_PCMAP_PC73", block: &BLOCK, off: 144, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC74: P = P { id: "MIDI_PCMAP_PC74", block: &BLOCK, off: 146, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC75: P = P { id: "MIDI_PCMAP_PC75", block: &BLOCK, off: 148, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC76: P = P { id: "MIDI_PCMAP_PC76", block: &BLOCK, off: 150, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC77: P = P { id: "MIDI_PCMAP_PC77", block: &BLOCK, off: 152, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC78: P = P { id: "MIDI_PCMAP_PC78", block: &BLOCK, off: 154, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC79: P = P { id: "MIDI_PCMAP_PC79", block: &BLOCK, off: 156, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC80: P = P { id: "MIDI_PCMAP_PC80", block: &BLOCK, off: 158, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC81: P = P { id: "MIDI_PCMAP_PC81", block: &BLOCK, off: 160, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC82: P = P { id: "MIDI_PCMAP_PC82", block: &BLOCK, off: 162, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC83: P = P { id: "MIDI_PCMAP_PC83", block: &BLOCK, off: 164, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC84: P = P { id: "MIDI_PCMAP_PC84", block: &BLOCK, off: 166, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC85: P = P { id: "MIDI_PCMAP_PC85", block: &BLOCK, off: 168, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC86: P = P { id: "MIDI_PCMAP_PC86", block: &BLOCK, off: 170, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC87: P = P { id: "MIDI_PCMAP_PC87", block: &BLOCK, off: 172, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC88: P = P { id: "MIDI_PCMAP_PC88", block: &BLOCK, off: 174, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC89: P = P { id: "MIDI_PCMAP_PC89", block: &BLOCK, off: 176, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC90: P = P { id: "MIDI_PCMAP_PC90", block: &BLOCK, off: 178, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC91: P = P { id: "MIDI_PCMAP_PC91", block: &BLOCK, off: 180, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC92: P = P { id: "MIDI_PCMAP_PC92", block: &BLOCK, off: 182, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC93: P = P { id: "MIDI_PCMAP_PC93", block: &BLOCK, off: 184, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC94: P = P { id: "MIDI_PCMAP_PC94", block: &BLOCK, off: 186, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC95: P = P { id: "MIDI_PCMAP_PC95", block: &BLOCK, off: 188, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC96: P = P { id: "MIDI_PCMAP_PC96", block: &BLOCK, off: 190, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC97: P = P { id: "MIDI_PCMAP_PC97", block: &BLOCK, off: 192, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC98: P = P { id: "MIDI_PCMAP_PC98", block: &BLOCK, off: 194, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC99: P = P { id: "MIDI_PCMAP_PC99", block: &BLOCK, off: 196, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC100: P = P { id: "MIDI_PCMAP_PC100", block: &BLOCK, off: 198, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC101: P = P { id: "MIDI_PCMAP_PC101", block: &BLOCK, off: 200, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC102: P = P { id: "MIDI_PCMAP_PC102", block: &BLOCK, off: 202, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC103: P = P { id: "MIDI_PCMAP_PC103", block: &BLOCK, off: 204, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC104: P = P { id: "MIDI_PCMAP_PC104", block: &BLOCK, off: 206, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC105: P = P { id: "MIDI_PCMAP_PC105", block: &BLOCK, off: 208, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC106: P = P { id: "MIDI_PCMAP_PC106", block: &BLOCK, off: 210, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC107: P = P { id: "MIDI_PCMAP_PC107", block: &BLOCK, off: 212, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC108: P = P { id: "MIDI_PCMAP_PC108", block: &BLOCK, off: 214, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC109: P = P { id: "MIDI_PCMAP_PC109", block: &BLOCK, off: 216, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC110: P = P { id: "MIDI_PCMAP_PC110", block: &BLOCK, off: 218, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC111: P = P { id: "MIDI_PCMAP_PC111", block: &BLOCK, off: 220, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC112: P = P { id: "MIDI_PCMAP_PC112", block: &BLOCK, off: 222, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC113: P = P { id: "MIDI_PCMAP_PC113", block: &BLOCK, off: 224, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC114: P = P { id: "MIDI_PCMAP_PC114", block: &BLOCK, off: 226, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC115: P = P { id: "MIDI_PCMAP_PC115", block: &BLOCK, off: 228, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC116: P = P { id: "MIDI_PCMAP_PC116", block: &BLOCK, off: 230, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC117: P = P { id: "MIDI_PCMAP_PC117", block: &BLOCK, off: 232, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC118: P = P { id: "MIDI_PCMAP_PC118", block: &BLOCK, off: 234, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC119: P = P { id: "MIDI_PCMAP_PC119", block: &BLOCK, off: 236, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC120: P = P { id: "MIDI_PCMAP_PC120", block: &BLOCK, off: 238, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC121: P = P { id: "MIDI_PCMAP_PC121", block: &BLOCK, off: 240, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC122: P = P { id: "MIDI_PCMAP_PC122", block: &BLOCK, off: 242, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC123: P = P { id: "MIDI_PCMAP_PC123", block: &BLOCK, off: 244, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC124: P = P { id: "MIDI_PCMAP_PC124", block: &BLOCK, off: 246, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC125: P = P { id: "MIDI_PCMAP_PC125", block: &BLOCK, off: 248, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC126: P = P { id: "MIDI_PCMAP_PC126", block: &BLOCK, off: 250, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC127: P = P { id: "MIDI_PCMAP_PC127", block: &BLOCK, off: 252, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
    pub const PC128: P = P { id: "MIDI_PCMAP_PC128", block: &BLOCK, off: 254, enc: Enc::N2, ofs: 0, min: 0, max: 128, init: 0 };
}

/// Every numeric parameter in SETUP, SYSTEM, the temporary memory and the
/// PC map, in address order. Used to describe single-parameter DT1s.
pub static ALL: &[P] = &[
    setup_com::PATCH_NUMBER,
    system_module::AMP,
    system_module::CABINET,
    system_module::AMBIENCE,
    system_phones::VOLUME,
    system_phones::SURROUND,
    system_phones::AMB_LEVEL,
    system_eq::LINK,
    system_eq_a::SW,
    system_eq_a::TYPE,
    system_eq_a::PEQ_LOW_GAIN,
    system_eq_a::PEQ_HIGH_GAIN,
    system_eq_a::PEQ_LEVEL,
    system_eq_a::PEQ_LOMID_FREQ,
    system_eq_a::PEQ_LOMID_Q,
    system_eq_a::PEQ_LOMID_GAIN,
    system_eq_a::PEQ_HIMID_FREQ,
    system_eq_a::PEQ_HIMID_Q,
    system_eq_a::PEQ_HIMID_GAIN,
    system_eq_a::GEQ_LEVEL,
    system_eq_a::GEQ_31HZ_GAIN,
    system_eq_a::GEQ_63HZ_GAIN,
    system_eq_a::GEQ_125HZ_GAIN,
    system_eq_a::GEQ_250HZ_GAIN,
    system_eq_a::GEQ_500HZ_GAIN,
    system_eq_a::GEQ_1KHZ_GAIN,
    system_eq_a::GEQ_2KHZ_GAIN,
    system_eq_a::GEQ_4KHZ_GAIN,
    system_eq_a::GEQ_8KHZ_GAIN,
    system_eq_a::GEQ_16KHZ_GAIN,
    system_eq_b::SW,
    system_eq_b::TYPE,
    system_eq_b::PEQ_LOW_GAIN,
    system_eq_b::PEQ_HIGH_GAIN,
    system_eq_b::PEQ_LEVEL,
    system_eq_b::PEQ_LOMID_FREQ,
    system_eq_b::PEQ_LOMID_Q,
    system_eq_b::PEQ_LOMID_GAIN,
    system_eq_b::PEQ_HIMID_FREQ,
    system_eq_b::PEQ_HIMID_Q,
    system_eq_b::PEQ_HIMID_GAIN,
    system_eq_b::GEQ_LEVEL,
    system_eq_b::GEQ_31HZ_GAIN,
    system_eq_b::GEQ_63HZ_GAIN,
    system_eq_b::GEQ_125HZ_GAIN,
    system_eq_b::GEQ_250HZ_GAIN,
    system_eq_b::GEQ_500HZ_GAIN,
    system_eq_b::GEQ_1KHZ_GAIN,
    system_eq_b::GEQ_2KHZ_GAIN,
    system_eq_b::GEQ_4KHZ_GAIN,
    system_eq_b::GEQ_8KHZ_GAIN,
    system_eq_b::GEQ_16KHZ_GAIN,
    system_out::LINK,
    system_out_a::TYPE,
    system_out_b::TYPE,
    system_memext::MIN,
    system_memext::MAX,
    system_usb::MODE,
    system_usb::VENDOR_DRY_OUT_LEVEL,
    system_usb::VENDOR_EFX_IN_LEVEL,
    system_usb::VENDOR_EFX_OUT_LEVEL,
    system_usb::VENDOR_MIX_IN_LEVEL,
    system_usb::GENERIC_EFX_OUT_LEVEL,
    system_usb::GENERIC_MIX_IN_LEVEL,
    system_usb::VENDOR_LOOPBACK_SW,
    system_usb::VENDOR_LOOPBACK_LEVEL,
    system_usb::GENERIC_LOOPBACK_SW,
    system_usb::GENERIC_LOOPBACK_LEVEL,
    system_midi::MIDI_THRU,
    system_midi::USB_THRU,
    system_midi::RX_CH,
    system_midi::TX_CH,
    system_midi::RX_PC,
    system_midi::TX_PC,
    system_midi::RX_CC,
    system_midi::TX_CC,
    system_midi::CC_MEMORY_DOWN,
    system_midi::CC_MEMORY_UP,
    system_midi::CC_AMP_GAIN,
    system_midi::CC_AMP_LEVEL,
    system_midi::CC_AMP_SOLO_SW,
    system_midi::CC_AMP_SOLO_LEVEL,
    system_midi::CC_CTL1,
    system_midi::CC_CTL2,
    system_midi::CC_EXP,
    system_midi::CC_BYPASS,
    system_pref::AUTO_OFF,
    system_pref::LCD_CONTRAST,
    system_pref::DISP_MODE,
    system_pref::PANEL_LOCK,
    system_pref::DISP_CAB,
    system_pref::PLAY_CURSOR,
    system_pref::EXP_PDL_HOLD,
    system_input::PREF,
    system_input::SETTING1,
    system_input::SETTING2,
    system_input::SETTING3,
    system_input::SETTING4,
    system_input::SETTING5,
    system_input::SETTING6,
    system_input::SETTING7,
    system_input::SETTING8,
    system_input::SETTING9,
    system_input::SETTING10,
    system_ctrl_pref::FSW_DOWN,
    system_ctrl_pref::FSW_UP,
    system_ctrl_pref::CTL1,
    system_ctrl_pref::CTL2,
    system_ctrl_pref::EXP,
    system_ctrl_fsw_down::FUNC_TYPE,
    system_ctrl_fsw_down::PRM_MEMORY,
    system_ctrl_fsw_down::PRM_AMP_SOLO,
    system_ctrl_fsw_down::PRM_EQ,
    system_ctrl_fsw_down::PRM_SNDRTN,
    system_ctrl_fsw_up::FUNC_TYPE,
    system_ctrl_fsw_up::PRM_MEMORY,
    system_ctrl_fsw_up::PRM_AMP_SOLO,
    system_ctrl_fsw_up::PRM_EQ,
    system_ctrl_fsw_up::PRM_SNDRTN,
    system_ctrl_ctl1::FUNC_TYPE,
    system_ctrl_ctl1::PRM_MEMORY,
    system_ctrl_ctl1::PRM_AMP_SOLO,
    system_ctrl_ctl1::PRM_EQ,
    system_ctrl_ctl1::PRM_SNDRTN,
    system_ctrl_ctl2::FUNC_TYPE,
    system_ctrl_ctl2::PRM_MEMORY,
    system_ctrl_ctl2::PRM_AMP_SOLO,
    system_ctrl_ctl2::PRM_EQ,
    system_ctrl_ctl2::PRM_SNDRTN,
    system_ctrl_exp::FUNC_TYPE,
    system_ctrl_exp::AMP_GAIN_MIN,
    system_ctrl_exp::AMP_GAIN_MAX,
    system_ctrl_exp::AMP_LEVEL_MIN,
    system_ctrl_exp::AMP_LEVEL_MAX,
    system_ctrl_exp::AMP_SOLO_MIN,
    system_ctrl_exp::AMP_SOLO_MAX,
    system_ctrl_exp::SEND_LEVEL_MIN,
    system_ctrl_exp::SEND_LEVEL_MAX,
    system_ctrl_exp::RTN_LEVEL_MIN,
    system_ctrl_exp::RTN_LEVEL_MAX,
    patch_amp::TYPE,
    patch_amp::GAIN,
    patch_amp::LEVEL,
    patch_amp::BASS,
    patch_amp::MIDDLE,
    patch_amp::TREBLE,
    patch_amp::GAIN_SW,
    patch_amp::SOLO_SW,
    patch_amp::SOLO_LEVEL,
    patch_ns::THRESHOLD,
    patch_ns::RELEASE,
    patch_sr::SW,
    patch_sr::MODE,
    patch_sr::SEND_LEVEL,
    patch_sr::RETURN_LEVEL,
    patch_sr::ADJUST,
    patch_sr::POS,
    patch_cab::LINK,
    patch_cab::OUT_MODE,
    patch_cab::ADJUST,
    patch_cab_a::CAB_TYPE,
    patch_cab_a::MIC_LEVEL,
    patch_cab_a::DIR_MIX,
    patch_cab_b::CAB_TYPE,
    patch_cab_b::MIC_LEVEL,
    patch_cab_b::DIR_MIX,
    patch_amb::MODE,
    patch_amb::TYPE,
    patch_amb::TIME,
    patch_amb::TONE,
    patch_amb::EFX_LEVEL,
    patch_eq::LINK,
    patch_eq_a::SW,
    patch_eq_a::TYPE,
    patch_eq_a::PEQ_LOW_GAIN,
    patch_eq_a::PEQ_HIGH_GAIN,
    patch_eq_a::PEQ_LEVEL,
    patch_eq_a::PEQ_LOMID_FREQ,
    patch_eq_a::PEQ_LOMID_Q,
    patch_eq_a::PEQ_LOMID_GAIN,
    patch_eq_a::PEQ_HIMID_FREQ,
    patch_eq_a::PEQ_HIMID_Q,
    patch_eq_a::PEQ_HIMID_GAIN,
    patch_eq_a::GEQ_LEVEL,
    patch_eq_a::GEQ_31HZ_GAIN,
    patch_eq_a::GEQ_63HZ_GAIN,
    patch_eq_a::GEQ_125HZ_GAIN,
    patch_eq_a::GEQ_250HZ_GAIN,
    patch_eq_a::GEQ_500HZ_GAIN,
    patch_eq_a::GEQ_1KHZ_GAIN,
    patch_eq_a::GEQ_2KHZ_GAIN,
    patch_eq_a::GEQ_4KHZ_GAIN,
    patch_eq_a::GEQ_8KHZ_GAIN,
    patch_eq_a::GEQ_16KHZ_GAIN,
    patch_eq_b::SW,
    patch_eq_b::TYPE,
    patch_eq_b::PEQ_LOW_GAIN,
    patch_eq_b::PEQ_HIGH_GAIN,
    patch_eq_b::PEQ_LEVEL,
    patch_eq_b::PEQ_LOMID_FREQ,
    patch_eq_b::PEQ_LOMID_Q,
    patch_eq_b::PEQ_LOMID_GAIN,
    patch_eq_b::PEQ_HIMID_FREQ,
    patch_eq_b::PEQ_HIMID_Q,
    patch_eq_b::PEQ_HIMID_GAIN,
    patch_eq_b::GEQ_LEVEL,
    patch_eq_b::GEQ_31HZ_GAIN,
    patch_eq_b::GEQ_63HZ_GAIN,
    patch_eq_b::GEQ_125HZ_GAIN,
    patch_eq_b::GEQ_250HZ_GAIN,
    patch_eq_b::GEQ_500HZ_GAIN,
    patch_eq_b::GEQ_1KHZ_GAIN,
    patch_eq_b::GEQ_2KHZ_GAIN,
    patch_eq_b::GEQ_4KHZ_GAIN,
    patch_eq_b::GEQ_8KHZ_GAIN,
    patch_eq_b::GEQ_16KHZ_GAIN,
    patch_input::PREF,
    patch_ctrl_fsw_down::FUNC_TYPE,
    patch_ctrl_fsw_down::PRM_MEMORY,
    patch_ctrl_fsw_down::PRM_AMP_SOLO,
    patch_ctrl_fsw_down::PRM_EQ,
    patch_ctrl_fsw_down::PRM_SNDRTN,
    patch_ctrl_fsw_up::FUNC_TYPE,
    patch_ctrl_fsw_up::PRM_MEMORY,
    patch_ctrl_fsw_up::PRM_AMP_SOLO,
    patch_ctrl_fsw_up::PRM_EQ,
    patch_ctrl_fsw_up::PRM_SNDRTN,
    patch_ctrl_ctl1::FUNC_TYPE,
    patch_ctrl_ctl1::PRM_MEMORY,
    patch_ctrl_ctl1::PRM_AMP_SOLO,
    patch_ctrl_ctl1::PRM_EQ,
    patch_ctrl_ctl1::PRM_SNDRTN,
    patch_ctrl_ctl2::FUNC_TYPE,
    patch_ctrl_ctl2::PRM_MEMORY,
    patch_ctrl_ctl2::PRM_AMP_SOLO,
    patch_ctrl_ctl2::PRM_EQ,
    patch_ctrl_ctl2::PRM_SNDRTN,
    patch_ctrl_exp::FUNC_TYPE,
    patch_ctrl_exp::AMP_GAIN_MIN,
    patch_ctrl_exp::AMP_GAIN_MAX,
    patch_ctrl_exp::AMP_LEVEL_MIN,
    patch_ctrl_exp::AMP_LEVEL_MAX,
    patch_ctrl_exp::AMP_SOLO_MIN,
    patch_ctrl_exp::AMP_SOLO_MAX,
    patch_ctrl_exp::SEND_LEVEL_MIN,
    patch_ctrl_exp::SEND_LEVEL_MAX,
    patch_ctrl_exp::RTN_LEVEL_MIN,
    patch_ctrl_exp::RTN_LEVEL_MAX,
    midi_pcmap::PC1,
    midi_pcmap::PC2,
    midi_pcmap::PC3,
    midi_pcmap::PC4,
    midi_pcmap::PC5,
    midi_pcmap::PC6,
    midi_pcmap::PC7,
    midi_pcmap::PC8,
    midi_pcmap::PC9,
    midi_pcmap::PC10,
    midi_pcmap::PC11,
    midi_pcmap::PC12,
    midi_pcmap::PC13,
    midi_pcmap::PC14,
    midi_pcmap::PC15,
    midi_pcmap::PC16,
    midi_pcmap::PC17,
    midi_pcmap::PC18,
    midi_pcmap::PC19,
    midi_pcmap::PC20,
    midi_pcmap::PC21,
    midi_pcmap::PC22,
    midi_pcmap::PC23,
    midi_pcmap::PC24,
    midi_pcmap::PC25,
    midi_pcmap::PC26,
    midi_pcmap::PC27,
    midi_pcmap::PC28,
    midi_pcmap::PC29,
    midi_pcmap::PC30,
    midi_pcmap::PC31,
    midi_pcmap::PC32,
    midi_pcmap::PC33,
    midi_pcmap::PC34,
    midi_pcmap::PC35,
    midi_pcmap::PC36,
    midi_pcmap::PC37,
    midi_pcmap::PC38,
    midi_pcmap::PC39,
    midi_pcmap::PC40,
    midi_pcmap::PC41,
    midi_pcmap::PC42,
    midi_pcmap::PC43,
    midi_pcmap::PC44,
    midi_pcmap::PC45,
    midi_pcmap::PC46,
    midi_pcmap::PC47,
    midi_pcmap::PC48,
    midi_pcmap::PC49,
    midi_pcmap::PC50,
    midi_pcmap::PC51,
    midi_pcmap::PC52,
    midi_pcmap::PC53,
    midi_pcmap::PC54,
    midi_pcmap::PC55,
    midi_pcmap::PC56,
    midi_pcmap::PC57,
    midi_pcmap::PC58,
    midi_pcmap::PC59,
    midi_pcmap::PC60,
    midi_pcmap::PC61,
    midi_pcmap::PC62,
    midi_pcmap::PC63,
    midi_pcmap::PC64,
    midi_pcmap::PC65,
    midi_pcmap::PC66,
    midi_pcmap::PC67,
    midi_pcmap::PC68,
    midi_pcmap::PC69,
    midi_pcmap::PC70,
    midi_pcmap::PC71,
    midi_pcmap::PC72,
    midi_pcmap::PC73,
    midi_pcmap::PC74,
    midi_pcmap::PC75,
    midi_pcmap::PC76,
    midi_pcmap::PC77,
    midi_pcmap::PC78,
    midi_pcmap::PC79,
    midi_pcmap::PC80,
    midi_pcmap::PC81,
    midi_pcmap::PC82,
    midi_pcmap::PC83,
    midi_pcmap::PC84,
    midi_pcmap::PC85,
    midi_pcmap::PC86,
    midi_pcmap::PC87,
    midi_pcmap::PC88,
    midi_pcmap::PC89,
    midi_pcmap::PC90,
    midi_pcmap::PC91,
    midi_pcmap::PC92,
    midi_pcmap::PC93,
    midi_pcmap::PC94,
    midi_pcmap::PC95,
    midi_pcmap::PC96,
    midi_pcmap::PC97,
    midi_pcmap::PC98,
    midi_pcmap::PC99,
    midi_pcmap::PC100,
    midi_pcmap::PC101,
    midi_pcmap::PC102,
    midi_pcmap::PC103,
    midi_pcmap::PC104,
    midi_pcmap::PC105,
    midi_pcmap::PC106,
    midi_pcmap::PC107,
    midi_pcmap::PC108,
    midi_pcmap::PC109,
    midi_pcmap::PC110,
    midi_pcmap::PC111,
    midi_pcmap::PC112,
    midi_pcmap::PC113,
    midi_pcmap::PC114,
    midi_pcmap::PC115,
    midi_pcmap::PC116,
    midi_pcmap::PC117,
    midi_pcmap::PC118,
    midi_pcmap::PC119,
    midi_pcmap::PC120,
    midi_pcmap::PC121,
    midi_pcmap::PC122,
    midi_pcmap::PC123,
    midi_pcmap::PC124,
    midi_pcmap::PC125,
    midi_pcmap::PC126,
    midi_pcmap::PC127,
    midi_pcmap::PC128,
];
