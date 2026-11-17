//! Named values shared by the memory and system models.
//!
//! Names and order come from the IR-200 owner's manual parameter list; the
//! numeric ranges come from IR Loader's address map. Lists the manual does not
//! order explicitly are marked *unverified* until checked on hardware (see
//! `docs/sysex-notes.md`).

use crate::value_enum;

value_enum! {
    /// Amp GAIN SW.
    GainSwitch { Low = 0, Middle = 1, High = 2 }
}

value_enum! {
    /// SEND/RETURN MODE.
    SendReturnMode {
        /// NORMAL MONO RET: in series, mono return.
        NormalMonoReturn = 0,
        /// NORMAL STEREO RET: in series, stereo return.
        NormalStereoReturn = 1,
        /// DIR MIX MONO RET: return mixed with the direct sound, mono return.
        DirectMixMonoReturn = 2,
        /// DIR MIX STEREO RET: return mixed with the direct sound, stereo return.
        DirectMixStereoReturn = 3,
    }
}

value_enum! {
    /// SEND/RETURN POSITION.
    SendReturnPosition {
        /// Before CABINET A and B.
        PreCabinet = 0,
        /// After EQ A and B.
        PostEq = 1,
    }
}

value_enum! {
    /// CABINET OUT MODE.
    CabinetOutMode {
        /// Cabinet A and B mixed together.
        MonoMix = 0,
        /// Cabinet A and B output in stereo.
        Stereo = 1,
    }
}

value_enum! {
    /// AMBIENCE MODE.
    AmbienceMode { Stereo = 0, DualMono = 1 }
}

value_enum! {
    /// AMBIENCE TYPE.
    AmbienceType { Thru = 0, Room = 1, Studio = 2, Hall = 3 }
}

value_enum! {
    /// EQ TYPE.
    EqType {
        /// Four-band parametric.
        Parametric = 0,
        /// Ten-band graphic.
        Graphic = 1,
    }
}

value_enum! {
    /// Footswitch / CTL FUNCTION.
    SwitchFunction { Memory = 0, AmpSolo = 1, Eq = 2, SendReturn = 3 }
}

value_enum! {
    /// EXP PDL FUNCTION.
    ExpFunction {
        Off = 0,
        AmpGain = 1,
        AmpLevel = 2,
        AmpSoloLevel = 3,
        SendLevel = 4,
        ReturnLevel = 5,
    }
}

value_enum! {
    /// Where a controller's settings come from (PREF).
    Preference {
        /// Set per memory.
        Memory = 0,
        /// Shared by all memories (SYSTEM).
        System = 1,
    }
}

value_enum! {
    /// OUTPUT SELECT TYPE: the kind of amp connected to OUTPUT A or B.
    OutputType {
        Line = 0,
        Jc120Return = 1,
        Jc120Input = 2,
        Combo1Return = 3,
        Combo1Input = 4,
        Combo2Return = 5,
        Combo2Input = 6,
        StackReturn = 7,
        StackInput = 8,
        BassAmpWithTweeter = 9,
        BassAmp = 10,
    }
}

value_enum! {
    /// USB MODE.
    UsbMode {
        /// IR-200 driver (Windows, macOS).
        Vendor = 1,
        /// Class compliant (iOS).
        Generic = 2,
    }
}

value_enum! {
    /// MIDI THRU / USB THRU: where incoming MIDI is echoed.
    MidiThru { Off = 0, Usb = 1, Midi = 2, UsbAndMidi = 3 }
}
