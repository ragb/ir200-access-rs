//! System area (`10 00 00 00`): settings shared by every memory. Nineteen leaf
//! blocks; offsets and ranges come from [`crate::map`].

use crate::map;
use crate::memory::{Eq, Expression, Switch};
use crate::types::*;
use crate::{area, block};

block! {
    /// Module on/off: the long-press switches on the AMP, CABINET and AMBIENCE knobs.
    Modules = map::system_module::BLOCK {
        amp: bool = map::system_module::AMP,
        cabinet: bool = map::system_module::CABINET,
        ambience: bool = map::system_module::AMBIENCE,
    }
}

block! {
    /// PHONES.
    Phones = map::system_phones::BLOCK {
        volume: u8 = map::system_phones::VOLUME,
        surround: bool = map::system_phones::SURROUND,
        ambience_level: u8 = map::system_phones::AMB_LEVEL,
    }
}

block! {
    /// MEMORY EXTENT: the range of memories the pedal steps through.
    MemoryExtent = map::system_memext::BLOCK {
        /// 1–128.
        min: u8 = map::system_memext::MIN,
        /// 1–128.
        max: u8 = map::system_memext::MAX,
    }
}

block! {
    /// USB.
    Usb = map::system_usb::BLOCK {
        mode: UsbMode = map::system_usb::MODE,
        /// VENDOR DRY OUT level, 0–200.
        vendor_dry_out_level: u8 = map::system_usb::VENDOR_DRY_OUT_LEVEL,
        vendor_efx_in_level: u8 = map::system_usb::VENDOR_EFX_IN_LEVEL,
        vendor_efx_out_level: u8 = map::system_usb::VENDOR_EFX_OUT_LEVEL,
        vendor_mix_in_level: u8 = map::system_usb::VENDOR_MIX_IN_LEVEL,
        generic_efx_out_level: u8 = map::system_usb::GENERIC_EFX_OUT_LEVEL,
        generic_mix_in_level: u8 = map::system_usb::GENERIC_MIX_IN_LEVEL,
        vendor_loopback: bool = map::system_usb::VENDOR_LOOPBACK_SW,
        vendor_loopback_level: u8 = map::system_usb::VENDOR_LOOPBACK_LEVEL,
        generic_loopback: bool = map::system_usb::GENERIC_LOOPBACK_SW,
        generic_loopback_level: u8 = map::system_usb::GENERIC_LOOPBACK_LEVEL,
    }
}

block! {
    /// MIDI.
    Midi = map::system_midi::BLOCK {
        midi_thru: MidiThru = map::system_midi::MIDI_THRU,
        usb_thru: MidiThru = map::system_midi::USB_THRU,
        /// 1–16, 17 = OFF (`rx_channel` catalog).
        rx_channel: u8 = map::system_midi::RX_CH,
        /// 1–16, 17 = Rx, 18 = OFF (`tx_channel` catalog).
        tx_channel: u8 = map::system_midi::TX_CH,
        rx_pc: bool = map::system_midi::RX_PC,
        tx_pc: bool = map::system_midi::TX_PC,
        rx_cc: bool = map::system_midi::RX_CC,
        tx_cc: bool = map::system_midi::TX_CC,
        /// CC numbers: 0 = OFF, 1–31 = CC#1–31, 32–63 = CC#64–95
        /// (`cc_number` catalog).
        cc_memory_down: u8 = map::system_midi::CC_MEMORY_DOWN,
        cc_memory_up: u8 = map::system_midi::CC_MEMORY_UP,
        cc_amp_gain: u8 = map::system_midi::CC_AMP_GAIN,
        cc_amp_level: u8 = map::system_midi::CC_AMP_LEVEL,
        cc_amp_solo: u8 = map::system_midi::CC_AMP_SOLO_SW,
        cc_amp_solo_level: u8 = map::system_midi::CC_AMP_SOLO_LEVEL,
        cc_ctl1: u8 = map::system_midi::CC_CTL1,
        cc_ctl2: u8 = map::system_midi::CC_CTL2,
        cc_exp: u8 = map::system_midi::CC_EXP,
        cc_bypass: u8 = map::system_midi::CC_BYPASS,
    }
}

block! {
    /// Preferences (OTHER and display settings).
    Preferences = map::system_pref::BLOCK {
        auto_off: bool = map::system_pref::AUTO_OFF,
        /// 1–16.
        lcd_contrast: u8 = map::system_pref::LCD_CONTRAST,
        /// Play screen display mode, 0–1.
        display_mode: u8 = map::system_pref::DISP_MODE,
        panel_lock: bool = map::system_pref::PANEL_LOCK,
        /// Cabinet display, 0–1.
        display_cabinet: u8 = map::system_pref::DISP_CAB,
        /// Play cursor, 0–1.
        play_cursor: u8 = map::system_pref::PLAY_CURSOR,
        /// EXP PDL HOLD: keep the pedal's effect when switching memories.
        exp_pedal_hold: bool = map::system_pref::EXP_PDL_HOLD,
    }
}

block! {
    /// INPUT: preference and the ten input level settings.
    Input = map::system_input::BLOCK {
        /// 0 = MEMORY (each memory picks a setting), 1–10 = that setting for
        /// all memories (`input_pref` catalog).
        preference: u8 = map::system_input::PREF,
        /// Input level of each setting, -20–+20 dB.
        setting_1: i8 = map::system_input::SETTING1,
        setting_2: i8 = map::system_input::SETTING2,
        setting_3: i8 = map::system_input::SETTING3,
        setting_4: i8 = map::system_input::SETTING4,
        setting_5: i8 = map::system_input::SETTING5,
        setting_6: i8 = map::system_input::SETTING6,
        setting_7: i8 = map::system_input::SETTING7,
        setting_8: i8 = map::system_input::SETTING8,
        setting_9: i8 = map::system_input::SETTING9,
        setting_10: i8 = map::system_input::SETTING10,
    }
}

block! {
    /// Controller PREF: per-memory or system-wide footswitch, CTL and EXP settings.
    ControlPreferences = map::system_ctrl_pref::BLOCK {
        fsw_down: Preference = map::system_ctrl_pref::FSW_DOWN,
        fsw_up: Preference = map::system_ctrl_pref::FSW_UP,
        ctl1: Preference = map::system_ctrl_pref::CTL1,
        ctl2: Preference = map::system_ctrl_pref::CTL2,
        exp: Preference = map::system_ctrl_pref::EXP,
    }
}

area! {
    /// Global settings.
    System {
        modules: Modules = block(map::system_module::BLOCK),
        phones: Phones = block(map::system_phones::BLOCK),
        /// GLOBAL EQ A/B LINK.
        eq_link: bool = param(map::system_eq::LINK),
        eq_a: Eq = block(map::system_eq_a::BLOCK),
        eq_b: Eq = block(map::system_eq_b::BLOCK),
        /// OUTPUT SELECT A/B LINK.
        output_link: bool = param(map::system_out::LINK),
        output_a: OutputType = param(map::system_out_a::TYPE),
        output_b: OutputType = param(map::system_out_b::TYPE),
        memory_extent: MemoryExtent = block(map::system_memext::BLOCK),
        usb: Usb = block(map::system_usb::BLOCK),
        midi: Midi = block(map::system_midi::BLOCK),
        preferences: Preferences = block(map::system_pref::BLOCK),
        input: Input = block(map::system_input::BLOCK),
        control_preferences: ControlPreferences = block(map::system_ctrl_pref::BLOCK),
        fsw_down: Switch = block(map::system_ctrl_fsw_down::BLOCK),
        fsw_up: Switch = block(map::system_ctrl_fsw_up::BLOCK),
        ctl1: Switch = block(map::system_ctrl_ctl1::BLOCK),
        ctl2: Switch = block(map::system_ctrl_ctl2::BLOCK),
        exp: Expression = block(map::system_ctrl_exp::BLOCK),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::CodecError;

    #[test]
    fn init_round_trips_through_nineteen_blocks() {
        let s = System::init();
        let blocks = s.to_blocks().unwrap();
        assert_eq!(blocks.len(), 19);
        assert_eq!(System::blocks().len(), 19);
        let got = System::from_blocks(|b| {
            blocks
                .iter()
                .find(|(x, _)| x.addr == b.addr)
                .map(|(_, d)| d.as_slice())
                .ok_or(CodecError::MissingBlock("x"))
        })
        .unwrap();
        assert_eq!(got, s);
        assert_eq!(s.midi.tx_channel, 17);
        assert_eq!(s.usb.mode, UsbMode::Vendor);
        assert_eq!(s.memory_extent.max, 128);
    }

    #[test]
    fn usb_mode_is_stored_zero_based() {
        let u = Usb::init();
        assert_eq!(u.to_bytes().unwrap()[0], 0);
    }
}
