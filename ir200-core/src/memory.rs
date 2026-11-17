//! A memory (patch): the temporary memory at `20 00 00 00` or a stored memory
//! 1–128. Seventeen leaf blocks; offsets and ranges come from [`crate::map`].

use crate::map;
use crate::types::*;
use crate::{area, block};

/// Length of a memory name.
pub const NAME_LEN: usize = 16;

block! {
    /// AMP.
    Amp = map::patch_amp::BLOCK {
        /// Amp type (0–11, see the `amp_type` catalog).
        amp_type: u8 = map::patch_amp::TYPE,
        /// Gain, 0–120.
        gain: u8 = map::patch_amp::GAIN,
        /// Level, 0–100.
        level: u8 = map::patch_amp::LEVEL,
        bass: u8 = map::patch_amp::BASS,
        middle: u8 = map::patch_amp::MIDDLE,
        treble: u8 = map::patch_amp::TREBLE,
        gain_switch: GainSwitch = map::patch_amp::GAIN_SW,
        /// SOLO SW.
        solo: bool = map::patch_amp::SOLO_SW,
        solo_level: u8 = map::patch_amp::SOLO_LEVEL,
    }
}

block! {
    /// NOISE SUPPRESSOR.
    NoiseSuppressor = map::patch_ns::BLOCK {
        /// Threshold, 0 = OFF, 1–100.
        threshold: u8 = map::patch_ns::THRESHOLD,
        release: u8 = map::patch_ns::RELEASE,
    }
}

block! {
    /// SEND/RETURN.
    SendReturn = map::patch_sr::BLOCK {
        on: bool = map::patch_sr::SW,
        mode: SendReturnMode = map::patch_sr::MODE,
        /// 0–200.
        send_level: u8 = map::patch_sr::SEND_LEVEL,
        /// 0–200.
        return_level: u8 = map::patch_sr::RETURN_LEVEL,
        /// Time lag between the internal sound and the external effect, 0–100.
        adjust: u8 = map::patch_sr::ADJUST,
        position: SendReturnPosition = map::patch_sr::POS,
    }
}

block! {
    /// CABINET settings shared by A and B.
    CabinetCommon = map::patch_cab::BLOCK {
        /// A/B LINK: edit both cabinets together.
        ab_link: bool = map::patch_cab::LINK,
        out_mode: CabinetOutMode = map::patch_cab::OUT_MODE,
        /// Time lag of cabinet B against A, 0–100.
        adjust: u8 = map::patch_cab::ADJUST,
    }
}

block! {
    /// CABINET A or B.
    Cabinet = map::patch_cab_a::BLOCK {
        /// Cab type, 0–336: presets, then USER IR 1–128 from 209 (see the
        /// `cab_type` catalog).
        cab_type: u16 = map::patch_cab_a::CAB_TYPE,
        mic_level: u8 = map::patch_cab_a::MIC_LEVEL,
        /// Direct sound (speaker off), 0–100.
        direct_mix: u8 = map::patch_cab_a::DIR_MIX,
    }
}

block! {
    /// AMBIENCE.
    Ambience = map::patch_amb::BLOCK {
        mode: AmbienceMode = map::patch_amb::MODE,
        ambience_type: AmbienceType = map::patch_amb::TYPE,
        /// Reverb time in tenths of a second, 1–100 (0.1–10.0 s).
        time: u8 = map::patch_amb::TIME,
        /// Tone, -50–+50.
        tone: i8 = map::patch_amb::TONE,
        /// Effect level, 0–100.
        level: u8 = map::patch_amb::EFX_LEVEL,
    }
}

block! {
    /// EQ A or B (memory and global EQ share the layout).
    Eq = map::patch_eq_a::BLOCK {
        on: bool = map::patch_eq_a::SW,
        eq_type: EqType = map::patch_eq_a::TYPE,
        /// Parametric low gain, dB.
        low_gain: i8 = map::patch_eq_a::PEQ_LOW_GAIN,
        /// Parametric high gain, dB.
        high_gain: i8 = map::patch_eq_a::PEQ_HIGH_GAIN,
        /// Parametric level, dB.
        level: i8 = map::patch_eq_a::PEQ_LEVEL,
        /// 0–29, see the `eq_freq` catalog.
        low_mid_freq: u8 = map::patch_eq_a::PEQ_LOMID_FREQ,
        /// 0–5, see the `eq_q` catalog.
        low_mid_q: u8 = map::patch_eq_a::PEQ_LOMID_Q,
        low_mid_gain: i8 = map::patch_eq_a::PEQ_LOMID_GAIN,
        high_mid_freq: u8 = map::patch_eq_a::PEQ_HIMID_FREQ,
        high_mid_q: u8 = map::patch_eq_a::PEQ_HIMID_Q,
        high_mid_gain: i8 = map::patch_eq_a::PEQ_HIMID_GAIN,
        /// Graphic level, dB.
        geq_level: i8 = map::patch_eq_a::GEQ_LEVEL,
        geq_31hz: i8 = map::patch_eq_a::GEQ_31HZ_GAIN,
        geq_63hz: i8 = map::patch_eq_a::GEQ_63HZ_GAIN,
        geq_125hz: i8 = map::patch_eq_a::GEQ_125HZ_GAIN,
        geq_250hz: i8 = map::patch_eq_a::GEQ_250HZ_GAIN,
        geq_500hz: i8 = map::patch_eq_a::GEQ_500HZ_GAIN,
        geq_1khz: i8 = map::patch_eq_a::GEQ_1KHZ_GAIN,
        geq_2khz: i8 = map::patch_eq_a::GEQ_2KHZ_GAIN,
        geq_4khz: i8 = map::patch_eq_a::GEQ_4KHZ_GAIN,
        geq_8khz: i8 = map::patch_eq_a::GEQ_8KHZ_GAIN,
        geq_16khz: i8 = map::patch_eq_a::GEQ_16KHZ_GAIN,
    }
}

block! {
    /// A footswitch or CTL jack: DOWN/UP FUNCTION, CTL1/CTL2 FUNCTION.
    Switch = map::patch_ctrl_fsw_down::BLOCK {
        function: SwitchFunction = map::patch_ctrl_fsw_down::FUNC_TYPE,
        /// MEMORY function target: 0 = DEC, 1 = INC, 2–129 = memory 1–128
        /// (`switch_memory` catalog).
        memory: u8 = map::patch_ctrl_fsw_down::PRM_MEMORY,
        /// AMP SOLO function parameter, 0–1.
        amp_solo: u8 = map::patch_ctrl_fsw_down::PRM_AMP_SOLO,
        /// EQ function target: 0 = A, 1 = B, 2 = A+B (`switch_eq` catalog).
        eq: u8 = map::patch_ctrl_fsw_down::PRM_EQ,
        /// SEND/RET function parameter (always 0).
        send_return: u8 = map::patch_ctrl_fsw_down::PRM_SNDRTN,
    }
}

block! {
    /// EXP PDL FUNCTION.
    Expression = map::patch_ctrl_exp::BLOCK {
        function: ExpFunction = map::patch_ctrl_exp::FUNC_TYPE,
        amp_gain_min: u8 = map::patch_ctrl_exp::AMP_GAIN_MIN,
        amp_gain_max: u8 = map::patch_ctrl_exp::AMP_GAIN_MAX,
        amp_level_min: u8 = map::patch_ctrl_exp::AMP_LEVEL_MIN,
        amp_level_max: u8 = map::patch_ctrl_exp::AMP_LEVEL_MAX,
        amp_solo_level_min: u8 = map::patch_ctrl_exp::AMP_SOLO_MIN,
        amp_solo_level_max: u8 = map::patch_ctrl_exp::AMP_SOLO_MAX,
        send_level_min: u8 = map::patch_ctrl_exp::SEND_LEVEL_MIN,
        send_level_max: u8 = map::patch_ctrl_exp::SEND_LEVEL_MAX,
        return_level_min: u8 = map::patch_ctrl_exp::RTN_LEVEL_MIN,
        return_level_max: u8 = map::patch_ctrl_exp::RTN_LEVEL_MAX,
    }
}

area! {
    /// One memory: what the pedal plays.
    Memory {
        /// Up to 16 printable ASCII characters.
        name: String = param(map::patch_com::NAME),
        amp: Amp = block(map::patch_amp::BLOCK),
        noise_suppressor: NoiseSuppressor = block(map::patch_ns::BLOCK),
        send_return: SendReturn = block(map::patch_sr::BLOCK),
        cabinet: CabinetCommon = block(map::patch_cab::BLOCK),
        cabinet_a: Cabinet = block(map::patch_cab_a::BLOCK),
        cabinet_b: Cabinet = block(map::patch_cab_b::BLOCK),
        ambience: Ambience = block(map::patch_amb::BLOCK),
        /// EQ A/B LINK: edit both EQs together.
        eq_link: bool = param(map::patch_eq::LINK),
        eq_a: Eq = block(map::patch_eq_a::BLOCK),
        eq_b: Eq = block(map::patch_eq_b::BLOCK),
        /// INPUT SETTING number, 1–10 (used when SYSTEM > INPUT > PREF is MEMORY).
        input_setting: u8 = param(map::patch_input::PREF),
        fsw_down: Switch = block(map::patch_ctrl_fsw_down::BLOCK),
        fsw_up: Switch = block(map::patch_ctrl_fsw_up::BLOCK),
        ctl1: Switch = block(map::patch_ctrl_ctl1::BLOCK),
        ctl2: Switch = block(map::patch_ctrl_ctl2::BLOCK),
        exp: Expression = block(map::patch_ctrl_exp::BLOCK),
    }
}

impl Memory {
    /// IR Loader's INIT MEMORY.
    pub fn init_memory() -> Self {
        let mut m = Self::init();
        m.name = "INIT MEMORY".into();
        m
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::codec::{Block, CodecError};

    fn same_layout(a: &Block, b: &Block) {
        assert_eq!(a.len, b.len, "{} vs {}", a.name, b.name);
    }

    #[test]
    fn shared_structs_cover_identical_blocks() {
        // A struct reused for several blocks must match each of them.
        same_layout(&map::patch_cab_a::BLOCK, &map::patch_cab_b::BLOCK);
        same_layout(&map::patch_eq_a::BLOCK, &map::patch_eq_b::BLOCK);
        same_layout(&map::patch_eq_a::BLOCK, &map::system_eq_a::BLOCK);
        same_layout(&map::patch_eq_a::BLOCK, &map::system_eq_b::BLOCK);
        for b in [
            &map::patch_ctrl_fsw_up::BLOCK,
            &map::patch_ctrl_ctl1::BLOCK,
            &map::patch_ctrl_ctl2::BLOCK,
            &map::system_ctrl_fsw_down::BLOCK,
            &map::system_ctrl_fsw_up::BLOCK,
            &map::system_ctrl_ctl1::BLOCK,
            &map::system_ctrl_ctl2::BLOCK,
        ] {
            same_layout(&map::patch_ctrl_fsw_down::BLOCK, b);
        }
        same_layout(&map::patch_ctrl_exp::BLOCK, &map::system_ctrl_exp::BLOCK);

        // And every parameter of the reused blocks sits at the same offset with
        // the same encoding and range.
        let pairs: &[(&[&crate::codec::P], &[&str])] = &[
            (Eq::params(), &["PATCH_EQ_B", "SYSTEM_EQ_A", "SYSTEM_EQ_B"]),
            (
                Switch::params(),
                &[
                    "PATCH_CTRL_FSW_UP",
                    "PATCH_CTRL_CTL1",
                    "PATCH_CTRL_CTL2",
                    "SYSTEM_CTRL_FSW_DOWN",
                    "SYSTEM_CTRL_FSW_UP",
                    "SYSTEM_CTRL_CTL1",
                    "SYSTEM_CTRL_CTL2",
                ],
            ),
            (Expression::params(), &["SYSTEM_CTRL_EXP"]),
            (Cabinet::params(), &["PATCH_CAB_B"]),
        ];
        for (params, others) in pairs {
            for (p, other) in params
                .iter()
                .flat_map(|p| others.iter().map(move |o| (p, o)))
            {
                let suffix = p.id.strip_prefix(p.block.name).unwrap();
                let id = format!("{other}{suffix}");
                let q = crate::map::ALL
                    .iter()
                    .find(|q| q.id == id)
                    .unwrap_or_else(|| panic!("{id} not in map"));
                assert_eq!(
                    (p.off, p.enc, p.ofs, p.min, p.max),
                    (q.off, q.enc, q.ofs, q.min, q.max),
                    "{id}"
                );
            }
        }
    }

    #[test]
    fn init_memory_round_trips() {
        let m = Memory::init_memory();
        let blocks = m.to_blocks().unwrap();
        assert_eq!(blocks.len(), 17);
        let got = Memory::from_blocks(|b| {
            blocks
                .iter()
                .find(|(x, _)| x.addr == b.addr)
                .map(|(_, d)| d.as_slice())
                .ok_or(CodecError::MissingBlock("x"))
        })
        .unwrap();
        assert_eq!(got, m);
        assert_eq!(m.amp.gain, 50);
        assert_eq!(m.ambience.ambience_type, AmbienceType::Hall);
        assert_eq!(m.cabinet_a.cab_type, 1);
    }

    #[test]
    fn name_is_space_padded() {
        let mut m = Memory::init();
        m.name = "CLEAN".into();
        let (_, b) = &m.to_blocks().unwrap()[0];
        assert_eq!(b.as_slice(), b"CLEAN           ");
        m.name = "THIS NAME IS WAY TOO LONG".into();
        assert!(m.to_blocks().is_err());
    }
}
