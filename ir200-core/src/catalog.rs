//! Value catalogs and editor-facing parameter metadata for the IR-200.
//!
//! IR Loader's address map has no value names, so they come from the IR-200
//! owner's manual (parameter list) and the BOSS sound list (amp and cabinet
//! types). The small closed lists are serde enums in [`crate::types`]; the
//! integer-coded fields below get name↔number catalogs so a memory can be
//! written by name (`cab_type: 412 BRIT STK DYN57-S`) and the editor can show
//! the names.
//!
//! The amp and cab type numbering was calibrated on hardware (2026-11-17)
//! against the factory memories 1-32: amp type 1 is NATURAL, cab type 1 is
//! the first BOSS IR, and BOSS cabinets are not in sound-list order. Index 0
//! of both lists and cab types 156-208 have no known name. See
//! `docs/sysex-notes.md`.

use midi_access_core::meta::{Kind, Level, ParamMeta};
use midi_access_core::{Catalogs, Params};
use serde_yaml::{Mapping, Value};

/// The IR-200's catalogs.
pub struct Ir200Catalogs;

/// The singleton the [`Device`](midi_access_core::Device) impl hands out.
pub static IR200_CATALOGS: Ir200Catalogs = Ir200Catalogs;

/// First cab type that is a user IR (IR Loader: the favourite flag of USER IR 1
/// is `CABINFO%COM%209`).
pub const USER_IR_FIRST_CAB_TYPE: u16 = 209;

/// Amp types by stored value. 0 has no known name; 1-8 are the guitar amps,
/// 9-11 the bass amps (hardware-verified against factory memories).
pub const AMP_TYPES: [&str; 12] = [
    "---",
    "NATURAL",
    "JC-120",
    "TWIN COMBO",
    "DIAMOND AMP",
    "TWEED COMBO",
    "X-HI GAIN",
    "BRITISH STACK",
    "BGR UB METAL",
    "NATURAL BASS",
    "X-DRIVE BASS",
    "CONCERT BASS",
];

/// BOSS cabinets in stored order (cab type `1 + 12 * cabinet + mic`).
/// Hardware-verified from the factory memories except 412 BRIT STK, B118 BAS
/// REF and B410 TWEETER, which fill the remaining slots.
const BOSS_CABINETS: [&str; 12] = [
    "112 NATURAL",
    "212 TWIN",
    "212 DIAMOND",
    "212 JC-120",
    "410 TWEED",
    "412 BGR UB",
    "412 CLASS ST",
    "412 BRIT STK",
    "B118 BAS REF",
    "B215 WIDE",
    "B410 TWEETER",
    "B810 A 81",
];

/// Mic and distance suffixes, 12 per BOSS cabinet.
const MICS: [&str; 12] = [
    "DYN57-S",
    "DYN57-L",
    "DYN421-S",
    "DYN421-L",
    "CND451-S",
    "CND451-L",
    "RBN121-S",
    "RBN121-L",
    "D57+R121-S",
    "D57+R121-L",
    "CND87-S",
    "CND87-L",
];

/// Celestion Digital IRs, in sound-list order.
const CELESTION: [&str; 11] = [
    "112 CEL-CREM R121 DARK",
    "112 CEL-RUBY MULTI",
    "212 CEL-BLUE D421 BALANC",
    "212 CEL-G12 R121 BALANC",
    "412 CEL-V30 D57 FAT",
    "412 CEL-G12T R121 BALANC",
    "412 CEL-CRB D57 BRIGHT",
    "412 CEL-GRB R121 BRIGHT",
    "412 CEL-G12 D421+R121",
    "412 CEL-V30 MULTI+ROOM L",
    "412 CEL-V30 MULTI+ROOM R",
];

/// The name of cab type `i` (0–336).
pub fn cab_type_name(i: u16) -> Option<String> {
    let boss = BOSS_CABINETS.len() * MICS.len();
    let i = i as usize;
    Some(if i == 0 {
        "---".to_string()
    } else if i <= boss {
        let i = i - 1;
        format!("{} {}", BOSS_CABINETS[i / MICS.len()], MICS[i % MICS.len()])
    } else if i <= boss + CELESTION.len() {
        CELESTION[i - boss - 1].to_string()
    } else if i < USER_IR_FIRST_CAB_TYPE as usize {
        format!("CAB {}", i + 1)
    } else if i < crate::address::CAB_TYPE_COUNT {
        format!("USER IR {}", i - USER_IR_FIRST_CAB_TYPE as usize + 1)
    } else {
        return None;
    })
}

/// EQ frequencies (1/3 octave, 20 Hz–16 kHz), index 0–29.
pub const EQ_FREQS: [&str; 30] = [
    "20.0 Hz", "25.0 Hz", "31.5 Hz", "40.0 Hz", "50.0 Hz", "63.0 Hz", "80.0 Hz", "100 Hz",
    "125 Hz", "160 Hz", "200 Hz", "250 Hz", "315 Hz", "400 Hz", "500 Hz", "630 Hz", "800 Hz",
    "1.00 kHz", "1.25 kHz", "1.60 kHz", "2.00 kHz", "2.50 kHz", "3.15 kHz", "4.00 kHz", "5.00 kHz",
    "6.30 kHz", "8.00 kHz", "10.0 kHz", "12.5 kHz", "16.0 kHz",
];

/// EQ Q, index 0–5.
pub const EQ_QS: [&str; 6] = ["0.5", "1", "2", "4", "8", "16"];

/// Footswitch EQ target, index 0–2.
pub const SWITCH_EQ: [&str; 3] = ["A", "B", "A+B"];

const CATALOG_NAMES: &[&str] = &[
    "amp_type",
    "cab_type",
    "eq_freq",
    "eq_q",
    "cc_number",
    "rx_channel",
    "tx_channel",
    "input_pref",
    "switch_memory",
    "switch_eq",
    "ambience_time",
];

/// A catalog as a list indexed by stored value.
fn list(cat: &str) -> Option<Vec<String>> {
    let s = |xs: &[&str]| xs.iter().map(|x| x.to_string()).collect::<Vec<_>>();
    Some(match cat {
        "amp_type" => s(&AMP_TYPES),
        "cab_type" => (0..crate::address::CAB_TYPE_COUNT as u16)
            .filter_map(cab_type_name)
            .collect(),
        "eq_freq" => s(&EQ_FREQS),
        "eq_q" => s(&EQ_QS),
        "cc_number" => std::iter::once("OFF".to_string())
            .chain((1..=31).map(|n| format!("CC#{n}")))
            .chain((64..=95).map(|n| format!("CC#{n}")))
            .collect(),
        // Indexed by the stored display value; 0 is unused.
        "rx_channel" => std::iter::once("-".to_string())
            .chain((1..=16).map(|n| n.to_string()))
            .chain(["OFF".to_string()])
            .collect(),
        "tx_channel" => std::iter::once("-".to_string())
            .chain((1..=16).map(|n| n.to_string()))
            .chain(["RX".to_string(), "OFF".to_string()])
            .collect(),
        "input_pref" => std::iter::once("MEMORY".to_string())
            .chain((1..=10).map(|n| format!("SETTING {n}")))
            .collect(),
        "switch_memory" => ["DEC".to_string(), "INC".to_string()]
            .into_iter()
            .chain((1..=128).map(|n| format!("MEMORY {n}")))
            .collect(),
        "switch_eq" => s(&SWITCH_EQ),
        "ambience_time" => std::iter::once("-".to_string())
            .chain((1..=100).map(|n| format!("{}.{} s", n / 10, n % 10)))
            .collect(),
        _ => return None,
    })
}

impl Catalogs for Ir200Catalogs {
    fn resolve(&self, cat: &str, name: &str) -> Option<i64> {
        list(cat)?
            .iter()
            .position(|s| s.eq_ignore_ascii_case(name.trim()))
            .map(|i| i as i64)
    }
    fn label(&self, cat: &str, value: i64) -> Option<String> {
        list(cat)?.get(usize::try_from(value).ok()?).cloned()
    }
    fn names(&self) -> &[&str] {
        CATALOG_NAMES
    }
    fn as_value(&self) -> Value {
        let mut m = Mapping::new();
        for name in CATALOG_NAMES {
            let seq = list(name)
                .unwrap_or_default()
                .into_iter()
                .map(Value::String)
                .collect();
            m.insert(Value::String((*name).into()), Value::Sequence(seq));
        }
        Value::Mapping(m)
    }
}

// === ParamMeta constructors ===

const fn p(
    path: &'static str,
    label: &'static str,
    group: &'static str,
    help: &'static str,
) -> ParamMeta {
    ParamMeta {
        path,
        label,
        group,
        help,
        level: Level::Plain,
        kind: None,
        catalog: None,
    }
}

/// A 0–N magnitude.
const fn pl(
    path: &'static str,
    label: &'static str,
    group: &'static str,
    help: &'static str,
    max: i64,
) -> ParamMeta {
    ParamMeta {
        path,
        label,
        group,
        help,
        level: Level::Magnitude,
        kind: Some(Kind::Range {
            min: 0,
            max,
            unit: None,
        }),
        catalog: None,
    }
}

/// A ±20 dB gain.
const fn db(
    path: &'static str,
    label: &'static str,
    group: &'static str,
    help: &'static str,
) -> ParamMeta {
    ParamMeta {
        path,
        label,
        group,
        help,
        level: Level::Plain,
        kind: Some(Kind::Range {
            min: -20,
            max: 20,
            unit: Some("dB"),
        }),
        catalog: None,
    }
}

const fn pc(
    path: &'static str,
    label: &'static str,
    group: &'static str,
    help: &'static str,
    catalog: &'static str,
) -> ParamMeta {
    ParamMeta {
        path,
        label,
        group,
        help,
        level: Level::Plain,
        kind: None,
        catalog: Some(catalog),
    }
}

const fn tg(
    path: &'static str,
    label: &'static str,
    group: &'static str,
    help: &'static str,
) -> ParamMeta {
    ParamMeta {
        path,
        label,
        group,
        help,
        level: Level::Plain,
        kind: Some(Kind::Toggle),
        catalog: None,
    }
}

/// Paths are area-prefixed serde paths. Structures that repeat use one shared
/// prefix the editor reuses for every instance: `eq.*` (memory EQ A/B and
/// global EQ A/B), `cabinet.*` (cabinet A/B), `switch.*` (DOWN, UP, CTL1, CTL2
/// in memories and system), `exp.*` (memory and system EXP).
static PARAMS: &[ParamMeta] = &[
    // ===== Memory =====
    p("memory.name", "Name", "Memory", "Memory name, up to 16 characters."),
    pc("memory.amp.amp_type", "Amp type", "Amp", "The amp model. Eight guitar amps and three bass amps.", "amp_type"),
    pl("memory.amp.gain", "Gain", "Amp", "Amount of distortion, 0 to 120.", 120),
    pl("memory.amp.level", "Level", "Amp", "Volume of the whole preamp. Be careful not to raise it too high.", 100),
    pl("memory.amp.bass", "Bass", "Amp", "Tone of the low frequency range.", 100),
    pl("memory.amp.middle", "Middle", "Amp", "Tone of the middle frequency range.", 100),
    pl("memory.amp.treble", "Treble", "Amp", "Tone of the high frequency range.", 100),
    p("memory.amp.gain_switch", "Gain switch", "Amp", "Gain range: low, middle or high."),
    tg("memory.amp.solo", "Solo", "Amp", "Switches the tone to one suitable for solos."),
    pl("memory.amp.solo_level", "Solo level", "Amp", "Volume when Solo is on.", 100),
    pl("memory.noise_suppressor.threshold", "Threshold", "Noise suppressor", "0 is off. Raise it for noisier setups until the guitar's decay still sounds natural. High values can mute playing at low guitar volume.", 100),
    pl("memory.noise_suppressor.release", "Release", "Noise suppressor", "Time from when the suppressor starts working until the noise reaches zero.", 100),
    tg("memory.send_return.on", "Send/return", "Send/return", "Turns the external effect loop on or off."),
    p("memory.send_return.mode", "Mode", "Send/return", "Normal puts the external effect in series. Direct mix blends the return with the direct sound. Mono or stereo return."),
    pl("memory.send_return.send_level", "Send level", "Send/return", "Level sent to the external effect, 0 to 200.", 200),
    pl("memory.send_return.return_level", "Return level", "Send/return", "Level coming back from the external effect, 0 to 200.", 200),
    pl("memory.send_return.adjust", "Adjust", "Send/return", "Time lag between the internal sound and the external effect.", 100),
    p("memory.send_return.position", "Position", "Send/return", "Before the cabinets, or after the EQs."),
    tg("memory.cabinet.ab_link", "Cabinet A/B link", "Cabinet", "When on, cabinet A's settings apply to both cabinets."),
    p("memory.cabinet.out_mode", "Output mode", "Cabinet", "Mono mix mixes cabinets A and B; stereo sends A and B to the two outputs."),
    pl("memory.cabinet.adjust", "Adjust", "Cabinet", "Time lag of cabinet B against cabinet A.", 100),
    pc("cabinet.cab_type", "Cab type", "Cabinet", "BOSS and Celestion cabinet IRs, then user IRs 1 to 128.", "cab_type"),
    pl("cabinet.mic_level", "Mic level", "Cabinet", "Volume of the mic.", 100),
    pl("cabinet.direct_mix", "Direct mix", "Cabinet", "Volume of the direct sound, with the speaker turned off.", 100),
    p("memory.ambience.mode", "Mode", "Ambience", "Stereo, or dual mono when outputs A and B go to different devices."),
    p("memory.ambience.ambience_type", "Type", "Ambience", "Thru, room, studio or hall."),
    pc("memory.ambience.time", "Time", "Ambience", "Reverberation length, 0.1 to 10 seconds.", "ambience_time"),
    ParamMeta {
        path: "memory.ambience.tone",
        label: "Tone",
        group: "Ambience",
        help: "Tonal character of the reverberation, -50 to +50.",
        level: Level::Plain,
        kind: Some(Kind::Range { min: -50, max: 50, unit: None }),
        catalog: None,
    },
    pl("memory.ambience.level", "Level", "Ambience", "Reverberation volume.", 100),
    tg("memory.eq_link", "EQ A/B link", "EQ", "When on, EQ A's settings apply to both EQs."),
    p("memory.input_setting", "Input setting", "Input", "Input setting number 1 to 10, used when the system input preference is Memory. Set the levels in System, Input."),
    tg("eq.on", "EQ", "EQ", "Turns the equalizer on or off."),
    p("eq.eq_type", "Type", "EQ", "Parametric: four bands. Graphic: ten bands."),
    db("eq.low_gain", "Low gain", "EQ", "Parametric: low frequency range."),
    db("eq.high_gain", "High gain", "EQ", "Parametric: high frequency range."),
    db("eq.level", "Level", "EQ", "Parametric: overall volume of the equalizer."),
    pc("eq.low_mid_freq", "Low-mid frequency", "EQ", "Centre of the range Low-mid gain adjusts.", "eq_freq"),
    pc("eq.low_mid_q", "Low-mid Q", "EQ", "Width around the low-mid frequency. Higher is narrower.", "eq_q"),
    db("eq.low_mid_gain", "Low-mid gain", "EQ", "Low-middle frequency range."),
    pc("eq.high_mid_freq", "High-mid frequency", "EQ", "Centre of the range High-mid gain adjusts.", "eq_freq"),
    pc("eq.high_mid_q", "High-mid Q", "EQ", "Width around the high-mid frequency. Higher is narrower.", "eq_q"),
    db("eq.high_mid_gain", "High-mid gain", "EQ", "High-middle frequency range."),
    db("eq.geq_level", "Graphic level", "EQ", "Graphic: overall volume of the equalizer."),
    db("eq.geq_31hz", "31.5 Hz", "EQ", "Graphic band."),
    db("eq.geq_63hz", "63 Hz", "EQ", "Graphic band."),
    db("eq.geq_125hz", "125 Hz", "EQ", "Graphic band."),
    db("eq.geq_250hz", "250 Hz", "EQ", "Graphic band."),
    db("eq.geq_500hz", "500 Hz", "EQ", "Graphic band."),
    db("eq.geq_1khz", "1 kHz", "EQ", "Graphic band."),
    db("eq.geq_2khz", "2 kHz", "EQ", "Graphic band."),
    db("eq.geq_4khz", "4 kHz", "EQ", "Graphic band."),
    db("eq.geq_8khz", "8 kHz", "EQ", "Graphic band."),
    db("eq.geq_16khz", "16 kHz", "EQ", "Graphic band."),
    p("switch.function", "Function", "Controls", "What the footswitch does: change memory, amp solo, EQ on/off, or send/return on/off."),
    pc("switch.memory", "Memory", "Controls", "For the memory function: previous, next, or a memory number.", "switch_memory"),
    p("switch.amp_solo", "Amp solo", "Controls", "For the amp solo function."),
    pc("switch.eq", "EQ", "Controls", "For the EQ function: which EQ to switch.", "switch_eq"),
    p("exp.function", "Expression pedal function", "Controls", "What the expression pedal controls."),
    pl("exp.amp_gain_min", "Amp gain min", "Controls", "Heel position.", 120),
    pl("exp.amp_gain_max", "Amp gain max", "Controls", "Toe position.", 120),
    pl("exp.amp_level_min", "Amp level min", "Controls", "Heel position.", 100),
    pl("exp.amp_level_max", "Amp level max", "Controls", "Toe position.", 100),
    pl("exp.amp_solo_level_min", "Solo level min", "Controls", "Heel position.", 100),
    pl("exp.amp_solo_level_max", "Solo level max", "Controls", "Toe position.", 100),
    pl("exp.send_level_min", "Send level min", "Controls", "Heel position.", 200),
    pl("exp.send_level_max", "Send level max", "Controls", "Toe position.", 200),
    pl("exp.return_level_min", "Return level min", "Controls", "Heel position.", 200),
    pl("exp.return_level_max", "Return level max", "Controls", "Toe position.", 200),
    // ===== System =====
    tg("system.modules.amp", "Amp", "Modules", "Amp on or off for every memory. Same as holding the AMP knob."),
    tg("system.modules.cabinet", "Cabinet", "Modules", "Cabinet on or off for every memory."),
    tg("system.modules.ambience", "Ambience", "Modules", "Ambience on or off for every memory."),
    pl("system.phones.volume", "Phones volume", "Phones", "Headphone volume.", 100),
    tg("system.phones.surround", "Surround", "Phones", "Makes the headphones sound as if the amp were in front of you."),
    pl("system.phones.ambience_level", "Phones ambience level", "Phones", "Volume of the headphone ambience.", 100),
    tg("system.eq_link", "Global EQ A/B link", "Global EQ", "When on, global EQ A's settings apply to both."),
    tg("system.output_link", "Output A/B link", "Output select", "When on, output A's type applies to both outputs."),
    p("system.output_a", "Output A type", "Output select", "The kind of amp or device connected to output A."),
    p("system.output_b", "Output B type", "Output select", "The kind of amp or device connected to output B."),
    p("system.memory_extent.min", "Lowest memory", "Memory extent", "The pedal steps through memories in this range."),
    p("system.memory_extent.max", "Highest memory", "Memory extent", "The pedal steps through memories in this range."),
    p("system.usb.mode", "USB mode", "USB", "Vendor for the IR-200 driver on Windows and macOS; generic for iOS."),
    pl("system.usb.vendor_dry_out_level", "Dry out level", "USB", "Unprocessed guitar sound sent to the computer (vendor driver).", 200),
    pl("system.usb.vendor_efx_in_level", "Effect in level", "USB", "Computer audio into the IR-200's effects (vendor driver).", 200),
    pl("system.usb.vendor_efx_out_level", "Effect out level", "USB", "Processed sound sent to the computer (vendor driver).", 200),
    pl("system.usb.vendor_mix_in_level", "Mix in level", "USB", "Computer audio mixed into the output (vendor driver).", 200),
    pl("system.usb.generic_efx_out_level", "Generic effect out level", "USB", "Processed sound sent to the device (generic mode).", 200),
    pl("system.usb.generic_mix_in_level", "Generic mix in level", "USB", "Device audio mixed into the output (generic mode).", 200),
    tg("system.usb.vendor_loopback", "Loopback", "USB", "Sends computer audio back to the computer (vendor driver)."),
    pl("system.usb.vendor_loopback_level", "Loopback level", "USB", "Level of the looped-back sound (vendor driver).", 200),
    tg("system.usb.generic_loopback", "Generic loopback", "USB", "Sends device audio back to the device (generic mode)."),
    pl("system.usb.generic_loopback_level", "Generic loopback level", "USB", "Level of the looped-back sound (generic mode).", 200),
    p("system.midi.midi_thru", "MIDI thru", "MIDI", "Where messages arriving at MIDI IN are echoed."),
    p("system.midi.usb_thru", "USB thru", "MIDI", "Where messages arriving over USB are echoed."),
    pc("system.midi.rx_channel", "Receive channel", "MIDI", "MIDI channel the pedal listens on, or off.", "rx_channel"),
    pc("system.midi.tx_channel", "Transmit channel", "MIDI", "MIDI channel the pedal sends on: a channel, the receive channel, or off.", "tx_channel"),
    tg("system.midi.rx_pc", "Receive program change", "MIDI", "Switch memories from incoming program changes."),
    tg("system.midi.tx_pc", "Send program change", "MIDI", "Send a program change when the memory changes."),
    tg("system.midi.rx_cc", "Receive control change", "MIDI", "Respond to incoming control changes."),
    tg("system.midi.tx_cc", "Send control change", "MIDI", "Send control changes from the pedal's controls."),
    pc("system.midi.cc_memory_down", "Down switch CC", "MIDI", "CC number for the down switch.", "cc_number"),
    pc("system.midi.cc_memory_up", "Up switch CC", "MIDI", "CC number for the up switch.", "cc_number"),
    pc("system.midi.cc_amp_gain", "Amp gain CC", "MIDI", "CC number for amp gain.", "cc_number"),
    pc("system.midi.cc_amp_level", "Amp level CC", "MIDI", "CC number for amp level.", "cc_number"),
    pc("system.midi.cc_amp_solo", "Solo switch CC", "MIDI", "CC number for the solo switch.", "cc_number"),
    pc("system.midi.cc_amp_solo_level", "Solo level CC", "MIDI", "CC number for the solo level.", "cc_number"),
    pc("system.midi.cc_ctl1", "CTL 1 CC", "MIDI", "CC number for CTL 1.", "cc_number"),
    pc("system.midi.cc_ctl2", "CTL 2 CC", "MIDI", "CC number for CTL 2.", "cc_number"),
    pc("system.midi.cc_exp", "Expression CC", "MIDI", "CC number for the expression pedal.", "cc_number"),
    pc("system.midi.cc_bypass", "Bypass CC", "MIDI", "CC number for bypass on/off.", "cc_number"),
    tg("system.preferences.auto_off", "Auto off", "Preferences", "Turns the power off after a long time without use."),
    p("system.preferences.lcd_contrast", "LCD contrast", "Preferences", "Brightness of the display characters, 1 to 16."),
    p("system.preferences.display_mode", "Display mode", "Preferences", "Play screen layout."),
    tg("system.preferences.panel_lock", "Panel lock", "Preferences", "Locks the panel controls."),
    p("system.preferences.display_cabinet", "Cabinet display", "Preferences", "How the cabinet is shown on the play screen."),
    p("system.preferences.play_cursor", "Play cursor", "Preferences", "Play screen cursor."),
    tg("system.preferences.exp_pedal_hold", "Expression pedal hold", "Preferences", "Keep the expression pedal's effect when switching memories."),
    pc("system.input.preference", "Input preference", "Input", "Memory: each memory picks its input setting. Setting 1 to 10: one setting for all memories.", "input_pref"),
    db("system.input.setting_1", "Setting 1 level", "Input", "Input level for setting 1. Match it to your guitar's output."),
    db("system.input.setting_2", "Setting 2 level", "Input", "Input level for setting 2."),
    db("system.input.setting_3", "Setting 3 level", "Input", "Input level for setting 3."),
    db("system.input.setting_4", "Setting 4 level", "Input", "Input level for setting 4."),
    db("system.input.setting_5", "Setting 5 level", "Input", "Input level for setting 5."),
    db("system.input.setting_6", "Setting 6 level", "Input", "Input level for setting 6."),
    db("system.input.setting_7", "Setting 7 level", "Input", "Input level for setting 7."),
    db("system.input.setting_8", "Setting 8 level", "Input", "Input level for setting 8."),
    db("system.input.setting_9", "Setting 9 level", "Input", "Input level for setting 9."),
    db("system.input.setting_10", "Setting 10 level", "Input", "Input level for setting 10."),
    p("system.control_preferences.fsw_down", "Down switch settings", "Controls", "Memory: each memory sets the down switch. System: one setting for all."),
    p("system.control_preferences.fsw_up", "Up switch settings", "Controls", "Memory: each memory sets the up switch. System: one setting for all."),
    p("system.control_preferences.ctl1", "CTL 1 settings", "Controls", "Memory: each memory sets CTL 1. System: one setting for all."),
    p("system.control_preferences.ctl2", "CTL 2 settings", "Controls", "Memory: each memory sets CTL 2. System: one setting for all."),
    p("system.control_preferences.exp", "Expression pedal settings", "Controls", "Memory: each memory sets the expression pedal. System: one setting for all."),
    // ===== Setup / PC map =====
    p("setup.memory_index", "Current memory", "Setup", "The selected memory, stored 0 to 127 for memories 1 to 128."),
    p("pc_map.memory", "Program change map", "MIDI", "The memory, 1 to 128, each program change number selects."),
];

/// The parameter table.
pub fn params() -> Params {
    Params(PARAMS)
}

/// The catalog bundle `{ device, params, catalogs, defaults }`.
pub fn bundle() -> midi_access_core::Bundle {
    use midi_access_core::Device;
    let mut defaults = Mapping::new();
    for a in crate::device::Ir200::areas() {
        if let Some(d) = crate::device::Ir200::defaults(a.name) {
            defaults.insert(Value::String(a.name.into()), d);
        }
    }
    midi_access_core::Bundle {
        device: crate::device::Ir200::NAME,
        params: params(),
        catalogs: IR200_CATALOGS.as_value(),
        defaults: Value::Mapping(defaults),
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;

    #[test]
    fn cab_types() {
        // Factory memories: NATURAL 1 = 9, JC-120 1 = 43, TWIN COMBO 1 = 23,
        // BGR UB METAL 1 = 154/155, NATURAL BASS = 119.
        assert_eq!(cab_type_name(9).unwrap(), "112 NATURAL D57+R121-S");
        assert_eq!(cab_type_name(43).unwrap(), "212 JC-120 RBN121-S");
        assert_eq!(cab_type_name(23).unwrap(), "212 TWIN CND87-S");
        assert_eq!(cab_type_name(154).unwrap(), "412 CEL-V30 MULTI+ROOM L");
        assert_eq!(cab_type_name(155).unwrap(), "412 CEL-V30 MULTI+ROOM R");
        assert_eq!(cab_type_name(119).unwrap(), "B215 WIDE CND87-S");
        assert_eq!(cab_type_name(144).unwrap(), "B810 A 81 CND87-L");
        assert_eq!(cab_type_name(145).unwrap(), "112 CEL-CREM R121 DARK");
        assert_eq!(cab_type_name(156).unwrap(), "CAB 157");
        assert_eq!(cab_type_name(209).unwrap(), "USER IR 1");
        assert_eq!(cab_type_name(336).unwrap(), "USER IR 128");
        assert!(cab_type_name(337).is_none());
        assert_eq!(list("cab_type").unwrap().len(), 337);
    }

    #[test]
    fn resolve_and_label() {
        let c = &IR200_CATALOGS;
        assert_eq!(c.resolve("cab_type", "412 brit stk dyn57-s"), Some(85));
        assert_eq!(c.label("cc_number", 32).as_deref(), Some("CC#64"));
        assert_eq!(c.label("tx_channel", 17).as_deref(), Some("RX"));
        assert_eq!(c.label("ambience_time", 26).as_deref(), Some("2.6 s"));
        assert_eq!(c.resolve("switch_memory", "MEMORY 1"), Some(2));
        for name in c.names() {
            assert!(list(name).is_some(), "{name}");
        }
    }

    #[test]
    fn params_unique_and_catalogs_known() {
        let mut seen = HashSet::new();
        for m in PARAMS {
            assert!(seen.insert(m.path), "duplicate {}", m.path);
            if let Some(c) = m.catalog {
                assert!(CATALOG_NAMES.contains(&c), "{c}");
            }
        }
    }
}
