"""IR-200 screen reader for Windows.

Talks to the BOSS IR-200 over USB MIDI the way BOSS's own IR Loader does,
and speaks what changes on the pedal: memory changes with their names, and
every knob, switch and setting the pedal reports.

Setup, once:
    py -m pip install mido python-rtmidi accessible_output2 pywin32

Run, with the IR-200 on USB and IR Loader closed:
    py ir200_reader.py

Keys while it runs (in its console window):
    R   read the current memory: number, name, amp type and amp knobs
    S   say the last thing again
    Q   quit (Ctrl+C works too)

Everything the pedal sends is logged to ir200_reader_log.txt next to this
script, with anything the reader doesn't recognise marked UNKNOWN. Send
that file back if something is missing or misnamed.

How it works: Roland's editor protocol. The reader sends the standard MIDI
identity request, reads the editor communication level (address 7F000000),
then writes 1 to 7F000001, "editor communication mode", which is what makes
the pedal report its own changes. It writes 0 there again when it quits,
exactly as IR Loader does when it disconnects. Apart from that one switch it
only ever reads; it never changes a setting on the pedal.

The parameter table below was generated from IR Loader's address_map.js.
Value names the app doesn't contain (amp types, cabinet types and so on)
are spoken as numbers; add names to LABELS to have them spoken instead.
"""

import sys
import time
from datetime import datetime
from pathlib import Path

try:
    import mido
except ImportError:
    sys.exit("Missing mido. Run: py -m pip install mido python-rtmidi")

LOG = Path(__file__).with_name("ir200_reader_log.txt")

ROLAND = 0x41
MODEL_ID = [0x00, 0x00, 0x00, 0x00, 0x06]
RQ1, DT1 = 0x11, 0x12

ADDR_PATCH_NUMBER = 0x00000000
ADDR_COMM_LEVEL = 0x7F000000
ADDR_COMM_MODE = 0x7F000001
ADDR_TEMP_NAME = 0x20000000
ADDR_TEMP_AMP = 0x20001000

# Words to say instead of numbers, by parameter. Fill these in from the
# pedal's screen or manual, e.g. "PATCH_AMP_TYPE": {0: "Natural", 1: ...}.
LABELS = {
    "PATCH_AMP_GAIN_SW": {0: "low", 1: "middle", 2: "high"},
}

# Spoken names where the automatic one reads badly.
NAMES = {
    "SETUP_COM_PATCH_NUMBER": "Memory",
    "PATCH_COM_NAME": "Name",
    "PATCH_SR_SW": "Send return",
    "PATCH_AMB_EFX_LEVEL": "Ambience level",
    "PATCH_CAB_A_DIR_MIX": "Cab A direct mix",
    "PATCH_CAB_B_DIR_MIX": "Cab B direct mix",
}

PARAMS = {
    0x00000000: ('n', 2, 0, 0, 127, 'SETUP_COM_PATCH_NUMBER'),
    0x10000000: ('i7', 1, 0, 0, 1, 'SYSTEM_MODULE_AMP'),
    0x10000001: ('i7', 1, 0, 0, 1, 'SYSTEM_MODULE_CABINET'),
    0x10000002: ('i7', 1, 0, 0, 1, 'SYSTEM_MODULE_AMBIENCE'),
    0x10001000: ('i7', 1, 0, 0, 100, 'SYSTEM_PHONES_VOLUME'),
    0x10001001: ('i7', 1, 0, 0, 1, 'SYSTEM_PHONES_SURROUND'),
    0x10001002: ('i7', 1, 0, 0, 100, 'SYSTEM_PHONES_AMB_LEVEL'),
    0x10002000: ('i7', 1, 0, 0, 1, 'SYSTEM_EQ_LINK'),
    0x10003000: ('i7', 1, 0, 0, 1, 'SYSTEM_EQ_A_SW'),
    0x10003001: ('i7', 1, 0, 0, 1, 'SYSTEM_EQ_A_TYPE'),
    0x10003002: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_PEQ_LOW_GAIN'),
    0x10003003: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_PEQ_HIGH_GAIN'),
    0x10003004: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_PEQ_LEVEL'),
    0x10003005: ('i7', 1, 0, 0, 29, 'SYSTEM_EQ_A_PEQ_LOMID_FREQ'),
    0x10003006: ('i7', 1, 0, 0, 5, 'SYSTEM_EQ_A_PEQ_LOMID_Q'),
    0x10003007: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_PEQ_LOMID_GAIN'),
    0x10003008: ('i7', 1, 0, 0, 29, 'SYSTEM_EQ_A_PEQ_HIMID_FREQ'),
    0x10003009: ('i7', 1, 0, 0, 5, 'SYSTEM_EQ_A_PEQ_HIMID_Q'),
    0x1000300A: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_PEQ_HIMID_GAIN'),
    0x1000300B: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_LEVEL'),
    0x1000300C: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_31Hz_GAIN'),
    0x1000300D: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_63Hz_GAIN'),
    0x1000300E: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_125Hz_GAIN'),
    0x1000300F: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_250Hz_GAIN'),
    0x10003010: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_500Hz_GAIN'),
    0x10003011: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_1kHz_GAIN'),
    0x10003012: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_2kHz_GAIN'),
    0x10003013: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_4kHz_GAIN'),
    0x10003014: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_8kHz_GAIN'),
    0x10003015: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_A_GEQ_16kHz_GAIN'),
    0x10004000: ('i7', 1, 0, 0, 1, 'SYSTEM_EQ_B_SW'),
    0x10004001: ('i7', 1, 0, 0, 1, 'SYSTEM_EQ_B_TYPE'),
    0x10004002: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_PEQ_LOW_GAIN'),
    0x10004003: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_PEQ_HIGH_GAIN'),
    0x10004004: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_PEQ_LEVEL'),
    0x10004005: ('i7', 1, 0, 0, 29, 'SYSTEM_EQ_B_PEQ_LOMID_FREQ'),
    0x10004006: ('i7', 1, 0, 0, 5, 'SYSTEM_EQ_B_PEQ_LOMID_Q'),
    0x10004007: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_PEQ_LOMID_GAIN'),
    0x10004008: ('i7', 1, 0, 0, 29, 'SYSTEM_EQ_B_PEQ_HIMID_FREQ'),
    0x10004009: ('i7', 1, 0, 0, 5, 'SYSTEM_EQ_B_PEQ_HIMID_Q'),
    0x1000400A: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_PEQ_HIMID_GAIN'),
    0x1000400B: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_LEVEL'),
    0x1000400C: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_31Hz_GAIN'),
    0x1000400D: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_63Hz_GAIN'),
    0x1000400E: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_125Hz_GAIN'),
    0x1000400F: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_250Hz_GAIN'),
    0x10004010: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_500Hz_GAIN'),
    0x10004011: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_1kHz_GAIN'),
    0x10004012: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_2kHz_GAIN'),
    0x10004013: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_4kHz_GAIN'),
    0x10004014: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_8kHz_GAIN'),
    0x10004015: ('i7', 1, 20, -20, 20, 'SYSTEM_EQ_B_GEQ_16kHz_GAIN'),
    0x10005000: ('i7', 1, 0, 0, 1, 'SYSTEM_OUT_LINK'),
    0x10006000: ('i7', 1, 0, 0, 10, 'SYSTEM_OUT_A_TYPE'),
    0x10007000: ('i7', 1, 0, 0, 10, 'SYSTEM_OUT_B_TYPE'),
    0x10010000: ('i7', 1, -1, 1, 128, 'SYSTEM_MEMEXT_MIN'),
    0x10010001: ('i7', 1, -1, 1, 128, 'SYSTEM_MEMEXT_MAX'),
    0x10011000: ('i7', 1, -1, 1, 2, 'SYSTEM_USB_MODE'),
    0x10011001: ('n', 2, 0, 0, 200, 'SYSTEM_USB_VENDOR_DRY_OUT_LEVEL'),
    0x10011003: ('n', 2, 0, 0, 200, 'SYSTEM_USB_VENDOR_EFX_IN_LEVEL'),
    0x10011005: ('n', 2, 0, 0, 200, 'SYSTEM_USB_VENDOR_EFX_OUT_LEVEL'),
    0x10011007: ('n', 2, 0, 0, 200, 'SYSTEM_USB_VENDOR_MIX_IN_LEVEL'),
    0x10011009: ('n', 2, 0, 0, 200, 'SYSTEM_USB_GENERIC_EFX_OUT_LEVEL'),
    0x1001100B: ('n', 2, 0, 0, 200, 'SYSTEM_USB_GENERIC_MIX_IN_LEVEL'),
    0x1001100D: ('i7', 1, 0, 0, 1, 'SYSTEM_USB_VENDOR_LOOPBACK_SW'),
    0x1001100E: ('n', 2, 0, 0, 200, 'SYSTEM_USB_VENDOR_LOOPBACK_LEVEL'),
    0x10011010: ('i7', 1, 0, 0, 1, 'SYSTEM_USB_GENERIC_LOOPBACK_SW'),
    0x10011011: ('n', 2, 0, 0, 200, 'SYSTEM_USB_GENERIC_LOOPBACK_LEVEL'),
    0x10012000: ('i7', 1, 0, 0, 3, 'SYSTEM_MIDI_MIDI_THRU'),
    0x10012001: ('i7', 1, 0, 0, 3, 'SYSTEM_MIDI_USB_THRU'),
    0x10012002: ('i7', 1, -1, 1, 17, 'SYSTEM_MIDI_RX_CH'),
    0x10012003: ('i7', 1, -1, 1, 18, 'SYSTEM_MIDI_TX_CH'),
    0x10012004: ('i7', 1, 0, 0, 1, 'SYSTEM_MIDI_RX_PC'),
    0x10012005: ('i7', 1, 0, 0, 1, 'SYSTEM_MIDI_TX_PC'),
    0x10012006: ('i7', 1, 0, 0, 1, 'SYSTEM_MIDI_RX_CC'),
    0x10012007: ('i7', 1, 0, 0, 1, 'SYSTEM_MIDI_TX_CC'),
    0x10012008: ('i7', 1, 0, 0, 63, 'SYSTEM_MIDI_CC_MEMORY_DOWN'),
    0x10012009: ('i7', 1, 0, 0, 63, 'SYSTEM_MIDI_CC_MEMORY_UP'),
    0x1001200A: ('i7', 1, 0, 0, 63, 'SYSTEM_MIDI_CC_AMP_GAIN'),
    0x1001200B: ('i7', 1, 0, 0, 63, 'SYSTEM_MIDI_CC_AMP_LEVEL'),
    0x1001200C: ('i7', 1, 0, 0, 63, 'SYSTEM_MIDI_CC_AMP_SOLO_SW'),
    0x1001200D: ('i7', 1, 0, 0, 63, 'SYSTEM_MIDI_CC_AMP_SOLO_LEVEL'),
    0x1001200E: ('i7', 1, 0, 0, 63, 'SYSTEM_MIDI_CC_CTL1'),
    0x1001200F: ('i7', 1, 0, 0, 63, 'SYSTEM_MIDI_CC_CTL2'),
    0x10012010: ('i7', 1, 0, 0, 63, 'SYSTEM_MIDI_CC_EXP'),
    0x10012011: ('i7', 1, 0, 0, 63, 'SYSTEM_MIDI_CC_BYPASS'),
    0x10013000: ('i7', 1, 0, 0, 1, 'SYSTEM_PREF_AUTO_OFF'),
    0x10013001: ('i7', 1, -1, 1, 16, 'SYSTEM_PREF_LCD_CONTRAST'),
    0x10013002: ('i7', 1, 0, 0, 1, 'SYSTEM_PREF_DISP_MODE'),
    0x10013003: ('i7', 1, 0, 0, 1, 'SYSTEM_PREF_PANEL_LOCK'),
    0x10013004: ('i7', 1, 0, 0, 1, 'SYSTEM_PREF_DISP_CAB'),
    0x10013005: ('i7', 1, 0, 0, 1, 'SYSTEM_PREF_PLAY_CURSOR'),
    0x10013006: ('i7', 1, 0, 0, 1, 'SYSTEM_PREF_EXP_PDL_HOLD'),
    0x10014000: ('i7', 1, 0, 0, 10, 'SYSTEM_INPUT_PREF'),
    0x10014001: ('i7', 1, 20, -20, 20, 'SYSTEM_INPUT_SETTING1'),
    0x10014002: ('i7', 1, 20, -20, 20, 'SYSTEM_INPUT_SETTING2'),
    0x10014003: ('i7', 1, 20, -20, 20, 'SYSTEM_INPUT_SETTING3'),
    0x10014004: ('i7', 1, 20, -20, 20, 'SYSTEM_INPUT_SETTING4'),
    0x10014005: ('i7', 1, 20, -20, 20, 'SYSTEM_INPUT_SETTING5'),
    0x10014006: ('i7', 1, 20, -20, 20, 'SYSTEM_INPUT_SETTING6'),
    0x10014007: ('i7', 1, 20, -20, 20, 'SYSTEM_INPUT_SETTING7'),
    0x10014008: ('i7', 1, 20, -20, 20, 'SYSTEM_INPUT_SETTING8'),
    0x10014009: ('i7', 1, 20, -20, 20, 'SYSTEM_INPUT_SETTING9'),
    0x1001400A: ('i7', 1, 20, -20, 20, 'SYSTEM_INPUT_SETTING10'),
    0x10015000: ('i7', 1, 0, 0, 1, 'SYSTEM_CTRL_PREF_FSW_DOWN'),
    0x10015001: ('i7', 1, 0, 0, 1, 'SYSTEM_CTRL_PREF_FSW_UP'),
    0x10015002: ('i7', 1, 0, 0, 1, 'SYSTEM_CTRL_PREF_CTL1'),
    0x10015003: ('i7', 1, 0, 0, 1, 'SYSTEM_CTRL_PREF_CTL2'),
    0x10015004: ('i7', 1, 0, 0, 1, 'SYSTEM_CTRL_PREF_EXP'),
    0x10016000: ('i7', 1, 0, 0, 3, 'SYSTEM_CTRL_FSW_DOWN_FUNC_TYPE'),
    0x10016001: ('n', 2, 0, 0, 129, 'SYSTEM_CTRL_FSW_DOWN_PRM_MEMORY'),
    0x10016003: ('i7', 1, 0, 0, 1, 'SYSTEM_CTRL_FSW_DOWN_PRM_AMP_SOLO'),
    0x10016004: ('i7', 1, 0, 0, 2, 'SYSTEM_CTRL_FSW_DOWN_PRM_EQ'),
    0x10016005: ('i7', 1, 0, 0, 0, 'SYSTEM_CTRL_FSW_DOWN_PRM_SNDRTN'),
    0x10017000: ('i7', 1, 0, 0, 3, 'SYSTEM_CTRL_FSW_UP_FUNC_TYPE'),
    0x10017001: ('n', 2, 0, 0, 129, 'SYSTEM_CTRL_FSW_UP_PRM_MEMORY'),
    0x10017003: ('i7', 1, 0, 0, 1, 'SYSTEM_CTRL_FSW_UP_PRM_AMP_SOLO'),
    0x10017004: ('i7', 1, 0, 0, 2, 'SYSTEM_CTRL_FSW_UP_PRM_EQ'),
    0x10017005: ('i7', 1, 0, 0, 0, 'SYSTEM_CTRL_FSW_UP_PRM_SNDRTN'),
    0x10020000: ('i7', 1, 0, 0, 3, 'SYSTEM_CTRL_CTL1_FUNC_TYPE'),
    0x10020001: ('n', 2, 0, 0, 129, 'SYSTEM_CTRL_CTL1_PRM_MEMORY'),
    0x10020003: ('i7', 1, 0, 0, 1, 'SYSTEM_CTRL_CTL1_PRM_AMP_SOLO'),
    0x10020004: ('i7', 1, 0, 0, 2, 'SYSTEM_CTRL_CTL1_PRM_EQ'),
    0x10020005: ('i7', 1, 0, 0, 0, 'SYSTEM_CTRL_CTL1_PRM_SNDRTN'),
    0x10021000: ('i7', 1, 0, 0, 3, 'SYSTEM_CTRL_CTL2_FUNC_TYPE'),
    0x10021001: ('n', 2, 0, 0, 129, 'SYSTEM_CTRL_CTL2_PRM_MEMORY'),
    0x10021003: ('i7', 1, 0, 0, 1, 'SYSTEM_CTRL_CTL2_PRM_AMP_SOLO'),
    0x10021004: ('i7', 1, 0, 0, 2, 'SYSTEM_CTRL_CTL2_PRM_EQ'),
    0x10021005: ('i7', 1, 0, 0, 0, 'SYSTEM_CTRL_CTL2_PRM_SNDRTN'),
    0x10022000: ('i7', 1, 0, 0, 5, 'SYSTEM_CTRL_EXP_FUNC_TYPE'),
    0x10022001: ('i7', 1, 0, 0, 120, 'SYSTEM_CTRL_EXP_AMP_GAIN_MIN'),
    0x10022002: ('i7', 1, 0, 0, 120, 'SYSTEM_CTRL_EXP_AMP_GAIN_MAX'),
    0x10022003: ('i7', 1, 0, 0, 100, 'SYSTEM_CTRL_EXP_AMP_LEVEL_MIN'),
    0x10022004: ('i7', 1, 0, 0, 100, 'SYSTEM_CTRL_EXP_AMP_LEVEL_MAX'),
    0x10022005: ('i7', 1, 0, 0, 100, 'SYSTEM_CTRL_EXP_AMP_SOLO_MIN'),
    0x10022006: ('i7', 1, 0, 0, 100, 'SYSTEM_CTRL_EXP_AMP_SOLO_MAX'),
    0x10022007: ('n', 2, 0, 0, 200, 'SYSTEM_CTRL_EXP_SEND_LEVEL_MIN'),
    0x10022009: ('n', 2, 0, 0, 200, 'SYSTEM_CTRL_EXP_SEND_LEVEL_MAX'),
    0x1002200B: ('n', 2, 0, 0, 200, 'SYSTEM_CTRL_EXP_RTN_LEVEL_MIN'),
    0x1002200D: ('n', 2, 0, 0, 200, 'SYSTEM_CTRL_EXP_RTN_LEVEL_MAX'),
    0x20000000: ('text', 16, 0, 0, 127, 'PATCH_COM_NAME'),
    0x20001000: ('i7', 1, 0, 0, 11, 'PATCH_AMP_TYPE'),
    0x20001001: ('i7', 1, 0, 0, 120, 'PATCH_AMP_GAIN'),
    0x20001002: ('i7', 1, 0, 0, 100, 'PATCH_AMP_LEVEL'),
    0x20001003: ('i7', 1, 0, 0, 100, 'PATCH_AMP_BASS'),
    0x20001004: ('i7', 1, 0, 0, 100, 'PATCH_AMP_MIDDLE'),
    0x20001005: ('i7', 1, 0, 0, 100, 'PATCH_AMP_TREBLE'),
    0x20001006: ('i7', 1, 0, 0, 2, 'PATCH_AMP_GAIN_SW'),
    0x20001007: ('i7', 1, 0, 0, 1, 'PATCH_AMP_SOLO_SW'),
    0x20001008: ('i7', 1, 0, 0, 100, 'PATCH_AMP_SOLO_LEVEL'),
    0x20002000: ('i7', 1, 0, 0, 100, 'PATCH_NS_THRESHOLD'),
    0x20002001: ('i7', 1, 0, 0, 100, 'PATCH_NS_RELEASE'),
    0x20003000: ('i7', 1, 0, 0, 1, 'PATCH_SR_SW'),
    0x20003001: ('i7', 1, 0, 0, 3, 'PATCH_SR_MODE'),
    0x20003002: ('n', 2, 0, 0, 200, 'PATCH_SR_SEND_LEVEL'),
    0x20003004: ('n', 2, 0, 0, 200, 'PATCH_SR_RETURN_LEVEL'),
    0x20003006: ('i7', 1, 0, 0, 100, 'PATCH_SR_ADJUST'),
    0x20003007: ('i7', 1, 0, 0, 1, 'PATCH_SR_POS'),
    0x20004000: ('i7', 1, 0, 0, 1, 'PATCH_CAB_LINK'),
    0x20004001: ('i7', 1, 0, 0, 1, 'PATCH_CAB_OUT_MODE'),
    0x20004002: ('i7', 1, 0, 0, 100, 'PATCH_CAB_ADJUST'),
    0x20005000: ('n', 4, 0, 0, 336, 'PATCH_CAB_A_CAB_TYPE'),
    0x20005004: ('i7', 1, 0, 0, 100, 'PATCH_CAB_A_MIC_LEVEL'),
    0x20005005: ('i7', 1, 0, 0, 100, 'PATCH_CAB_A_DIR_MIX'),
    0x20006000: ('n', 4, 0, 0, 336, 'PATCH_CAB_B_CAB_TYPE'),
    0x20006004: ('i7', 1, 0, 0, 100, 'PATCH_CAB_B_MIC_LEVEL'),
    0x20006005: ('i7', 1, 0, 0, 100, 'PATCH_CAB_B_DIR_MIX'),
    0x20007000: ('i7', 1, 0, 0, 1, 'PATCH_AMB_MODE'),
    0x20007001: ('i7', 1, 0, 0, 3, 'PATCH_AMB_TYPE'),
    0x20007002: ('i7', 1, 0, 1, 100, 'PATCH_AMB_TIME'),
    0x20007003: ('i7', 1, 50, -50, 50, 'PATCH_AMB_TONE'),
    0x20007004: ('i7', 1, 0, 0, 100, 'PATCH_AMB_EFX_LEVEL'),
    0x20010000: ('i7', 1, 0, 0, 1, 'PATCH_EQ_LINK'),
    0x20011000: ('i7', 1, 0, 0, 1, 'PATCH_EQ_A_SW'),
    0x20011001: ('i7', 1, 0, 0, 1, 'PATCH_EQ_A_TYPE'),
    0x20011002: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_PEQ_LOW_GAIN'),
    0x20011003: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_PEQ_HIGH_GAIN'),
    0x20011004: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_PEQ_LEVEL'),
    0x20011005: ('i7', 1, 0, 0, 29, 'PATCH_EQ_A_PEQ_LOMID_FREQ'),
    0x20011006: ('i7', 1, 0, 0, 5, 'PATCH_EQ_A_PEQ_LOMID_Q'),
    0x20011007: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_PEQ_LOMID_GAIN'),
    0x20011008: ('i7', 1, 0, 0, 29, 'PATCH_EQ_A_PEQ_HIMID_FREQ'),
    0x20011009: ('i7', 1, 0, 0, 5, 'PATCH_EQ_A_PEQ_HIMID_Q'),
    0x2001100A: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_PEQ_HIMID_GAIN'),
    0x2001100B: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_LEVEL'),
    0x2001100C: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_31Hz_GAIN'),
    0x2001100D: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_63Hz_GAIN'),
    0x2001100E: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_125Hz_GAIN'),
    0x2001100F: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_250Hz_GAIN'),
    0x20011010: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_500Hz_GAIN'),
    0x20011011: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_1kHz_GAIN'),
    0x20011012: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_2kHz_GAIN'),
    0x20011013: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_4kHz_GAIN'),
    0x20011014: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_8kHz_GAIN'),
    0x20011015: ('i7', 1, 20, -20, 20, 'PATCH_EQ_A_GEQ_16kHz_GAIN'),
    0x20012000: ('i7', 1, 0, 0, 1, 'PATCH_EQ_B_SW'),
    0x20012001: ('i7', 1, 0, 0, 1, 'PATCH_EQ_B_TYPE'),
    0x20012002: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_PEQ_LOW_GAIN'),
    0x20012003: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_PEQ_HIGH_GAIN'),
    0x20012004: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_PEQ_LEVEL'),
    0x20012005: ('i7', 1, 0, 0, 29, 'PATCH_EQ_B_PEQ_LOMID_FREQ'),
    0x20012006: ('i7', 1, 0, 0, 5, 'PATCH_EQ_B_PEQ_LOMID_Q'),
    0x20012007: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_PEQ_LOMID_GAIN'),
    0x20012008: ('i7', 1, 0, 0, 29, 'PATCH_EQ_B_PEQ_HIMID_FREQ'),
    0x20012009: ('i7', 1, 0, 0, 5, 'PATCH_EQ_B_PEQ_HIMID_Q'),
    0x2001200A: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_PEQ_HIMID_GAIN'),
    0x2001200B: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_LEVEL'),
    0x2001200C: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_31Hz_GAIN'),
    0x2001200D: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_63Hz_GAIN'),
    0x2001200E: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_125Hz_GAIN'),
    0x2001200F: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_250Hz_GAIN'),
    0x20012010: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_500Hz_GAIN'),
    0x20012011: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_1kHz_GAIN'),
    0x20012012: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_2kHz_GAIN'),
    0x20012013: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_4kHz_GAIN'),
    0x20012014: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_8kHz_GAIN'),
    0x20012015: ('i7', 1, 20, -20, 20, 'PATCH_EQ_B_GEQ_16kHz_GAIN'),
    0x20013000: ('i7', 1, 0, 1, 10, 'PATCH_INPUT_PREF'),
    0x20014000: ('i7', 1, 0, 0, 3, 'PATCH_CTRL_FSW_DOWN_FUNC_TYPE'),
    0x20014001: ('n', 2, 0, 0, 129, 'PATCH_CTRL_FSW_DOWN_PRM_MEMORY'),
    0x20014003: ('i7', 1, 0, 0, 1, 'PATCH_CTRL_FSW_DOWN_PRM_AMP_SOLO'),
    0x20014004: ('i7', 1, 0, 0, 2, 'PATCH_CTRL_FSW_DOWN_PRM_EQ'),
    0x20014005: ('i7', 1, 0, 0, 0, 'PATCH_CTRL_FSW_DOWN_PRM_SNDRTN'),
    0x20015000: ('i7', 1, 0, 0, 3, 'PATCH_CTRL_FSW_UP_FUNC_TYPE'),
    0x20015001: ('n', 2, 0, 0, 129, 'PATCH_CTRL_FSW_UP_PRM_MEMORY'),
    0x20015003: ('i7', 1, 0, 0, 1, 'PATCH_CTRL_FSW_UP_PRM_AMP_SOLO'),
    0x20015004: ('i7', 1, 0, 0, 2, 'PATCH_CTRL_FSW_UP_PRM_EQ'),
    0x20015005: ('i7', 1, 0, 0, 0, 'PATCH_CTRL_FSW_UP_PRM_SNDRTN'),
    0x20016000: ('i7', 1, 0, 0, 3, 'PATCH_CTRL_CTL1_FUNC_TYPE'),
    0x20016001: ('n', 2, 0, 0, 129, 'PATCH_CTRL_CTL1_PRM_MEMORY'),
    0x20016003: ('i7', 1, 0, 0, 1, 'PATCH_CTRL_CTL1_PRM_AMP_SOLO'),
    0x20016004: ('i7', 1, 0, 0, 2, 'PATCH_CTRL_CTL1_PRM_EQ'),
    0x20016005: ('i7', 1, 0, 0, 0, 'PATCH_CTRL_CTL1_PRM_SNDRTN'),
    0x20017000: ('i7', 1, 0, 0, 3, 'PATCH_CTRL_CTL2_FUNC_TYPE'),
    0x20017001: ('n', 2, 0, 0, 129, 'PATCH_CTRL_CTL2_PRM_MEMORY'),
    0x20017003: ('i7', 1, 0, 0, 1, 'PATCH_CTRL_CTL2_PRM_AMP_SOLO'),
    0x20017004: ('i7', 1, 0, 0, 2, 'PATCH_CTRL_CTL2_PRM_EQ'),
    0x20017005: ('i7', 1, 0, 0, 0, 'PATCH_CTRL_CTL2_PRM_SNDRTN'),
    0x20020000: ('i7', 1, 0, 0, 5, 'PATCH_CTRL_EXP_FUNC_TYPE'),
    0x20020001: ('i7', 1, 0, 0, 120, 'PATCH_CTRL_EXP_AMP_GAIN_MIN'),
    0x20020002: ('i7', 1, 0, 0, 120, 'PATCH_CTRL_EXP_AMP_GAIN_MAX'),
    0x20020003: ('i7', 1, 0, 0, 100, 'PATCH_CTRL_EXP_AMP_LEVEL_MIN'),
    0x20020004: ('i7', 1, 0, 0, 100, 'PATCH_CTRL_EXP_AMP_LEVEL_MAX'),
    0x20020005: ('i7', 1, 0, 0, 100, 'PATCH_CTRL_EXP_AMP_SOLO_MIN'),
    0x20020006: ('i7', 1, 0, 0, 100, 'PATCH_CTRL_EXP_AMP_SOLO_MAX'),
    0x20020007: ('n', 2, 0, 0, 200, 'PATCH_CTRL_EXP_SEND_LEVEL_MIN'),
    0x20020009: ('n', 2, 0, 0, 200, 'PATCH_CTRL_EXP_SEND_LEVEL_MAX'),
    0x2002000B: ('n', 2, 0, 0, 200, 'PATCH_CTRL_EXP_RTN_LEVEL_MIN'),
    0x2002000D: ('n', 2, 0, 0, 200, 'PATCH_CTRL_EXP_RTN_LEVEL_MAX'),
}


# ---- Roland SysEx helpers -------------------------------------------------
# Addresses travel as four 7-bit bytes; arithmetic is done on the packed
# 28-bit form and converted back, as IR Loader's nibble()/_7bitize() do.

def pack(addr):
    return (((addr >> 24) & 0x7F) << 21) | (((addr >> 16) & 0x7F) << 14) | \
           (((addr >> 8) & 0x7F) << 7) | (addr & 0x7F)


def unpack(lin):
    return (((lin >> 21) & 0x7F) << 24) | (((lin >> 14) & 0x7F) << 16) | \
           (((lin >> 7) & 0x7F) << 8) | (lin & 0x7F)


def addr_add(addr, n):
    return unpack(pack(addr) + n)


def be4(x):
    return [(x >> 24) & 0x7F, (x >> 16) & 0x7F, (x >> 8) & 0x7F, x & 0x7F]


def checksum(body):
    return (128 - sum(body) % 128) & 0x7F


class Roland:
    def __init__(self, out):
        self.out = out
        self.device = 0x10

    def _send(self, cmd, body):
        data = [ROLAND, self.device] + MODEL_ID + [cmd] + body + [checksum(body)]
        self.out.send(mido.Message("sysex", data=data))

    def rq1(self, addr, size):
        self._send(RQ1, be4(addr) + be4(unpack(size)))

    def dt1(self, addr, values):
        self._send(DT1, be4(addr) + list(values))

    def parse_dt1(self, d):
        """(address, data bytes) if d is a DT1 from this model, else None."""
        head = [ROLAND, self.device] + MODEL_ID + [DT1]
        alt = [ROLAND, 0x7F] + MODEL_ID + [DT1]
        n = len(head)
        if len(d) < n + 5 or (d[:n] != head and d[:n] != alt):
            return None
        addr = (d[n] << 24) | (d[n+1] << 16) | (d[n+2] << 8) | d[n+3]
        return addr, d[n+4:-1]


# ---- decoding values -------------------------------------------------------

def decode(kind, n, raw):
    if kind == "i7":
        return raw[0]
    if kind == "i14":
        return (raw[0] << 7) | raw[1]
    if kind == "n":
        v = 0
        for b in raw:
            v = (v << 4) | (b & 0x0F)
        return v
    return bytes(raw).decode("ascii", "replace").rstrip()


WORDS = {
    "AMP": "amp", "CAB": "cab", "AMB": "ambience", "NS": "noise suppressor",
    "SR": "send return", "EQ": "E Q", "PEQ": "", "GEQ": "", "SW": "",
    "A": "A", "B": "B", "LOMID": "low mid", "HIMID": "high mid",
    "FREQ": "frequency", "Q": "Q", "EXP": "expression", "FSW": "footswitch",
    "CTL1": "control 1", "CTL2": "control 2", "CTRL": "", "PRM": "",
    "FUNC": "function", "EFX": "effect", "DIR": "direct", "RTN": "return",
    "MIN": "minimum", "MAX": "maximum", "POS": "position", "PREF": "preference",
    "MEMEXT": "memory extent", "RX": "receive", "TX": "transmit",
    "CH": "channel", "PC": "program change", "CC": "C C", "SNDRTN": "send return",
    "COM": "", "OUT": "output", "TYPE": "type", "USB": "U S B",
}


def spoken_name(pid):
    if pid in NAMES:
        return NAMES[pid]
    tokens = pid.split("_")
    if tokens[0] in ("PATCH", "SYSTEM", "SETUP"):
        tokens = tokens[1:]
    if pid.startswith("SYSTEM_"):
        tokens = ["system"] + tokens
    # "CAB_A_CAB_TYPE" -> "cab A type"
    out = []
    for t in tokens:
        w = WORDS.get(t, t.replace("kHz", " kilohertz").replace("Hz", " hertz").lower())
        if w and w not in out:
            out.append(w)
    text = " ".join(out)
    return text[:1].upper() + text[1:]


def spoken_value(pid, kind, ofs, mn, mx, v):
    if kind == "text":
        return v or "blank"
    if pid in LABELS and v in LABELS[pid]:
        return LABELS[pid][v]
    if mn == 0 and mx == 1 and not pid.endswith("TYPE"):
        return "on" if v else "off"
    shown = v - ofs
    if pid == "SETUP_COM_PATCH_NUMBER":
        shown = v + 1
    return str(shown)


# ---- speech ----------------------------------------------------------------

def make_speaker():
    try:
        from accessible_output2.outputs.auto import Auto
        out = Auto()
        out.output("", interrupt=True)
        return lambda text: out.output(text, interrupt=True)
    except Exception:
        pass
    try:
        import win32com.client
        voice = win32com.client.Dispatch("SAPI.SpVoice")
        return lambda text: voice.Speak(text, 1 | 2)  # async, purge previous
    except Exception:
        return lambda text: None


# ---- the reader ------------------------------------------------------------

class Reader:
    def __init__(self, say, record):
        self.say_raw, self.record = say, record
        self.last = ""
        self.values = {}
        self.pending = []
        self.memory_since = None   # when a memory change began waiting for its name

    def say(self, text):
        self.last = text
        self.record(f"SAY {text}")
        self.say_raw(text)

    def handle_dt1(self, addr, data, quiet=False):
        changed = []
        for i in range(len(data)):
            a = addr_add(addr, i)
            p = PARAMS.get(a)
            if not p:
                continue
            kind, n, ofs, mn, mx, pid = p
            if i + n > len(data):
                continue
            v = decode(kind, n, data[i:i+n])
            old = self.values.get(pid)
            self.values[pid] = v
            if old != v:
                changed.append((pid, kind, ofs, mn, mx, v))
        known = any(PARAMS.get(addr_add(addr, i)) for i in range(len(data)))
        if not known:
            self.record(f"UNKNOWN DT1 {addr:08X} {' '.join(f'{b:02X}' for b in data)}")
        if quiet:
            return
        if any(c[0] == "SETUP_COM_PATCH_NUMBER" for c in changed):
            self.pending.append("memory")
            return
        if self.memory_since is not None and addr == ADDR_TEMP_NAME:
            self.memory_since = None
            self.say(self.memory_text())
            return
        if len(changed) > 3:
            # A whole block arrived, usually after a memory change.
            name = self.values.get("PATCH_COM_NAME")
            if name and any(c[0] == "PATCH_COM_NAME" for c in changed):
                self.say(name)
            return
        if changed:
            self.say(", ".join(f"{spoken_name(c[0])} {spoken_value(*c)}" for c in changed))

    def memory_text(self):
        mem = self.values.get("SETUP_COM_PATCH_NUMBER", 0) + 1
        name = self.values.get("PATCH_COM_NAME")
        return f"Memory {mem}, {name}" if name else f"Memory {mem}"

    def summary(self):
        mem = self.values.get("SETUP_COM_PATCH_NUMBER")
        parts = []
        if mem is not None:
            parts.append(f"Memory {mem + 1}")
        if self.values.get("PATCH_COM_NAME"):
            parts.append(self.values["PATCH_COM_NAME"])
        for pid in ("PATCH_AMP_TYPE", "PATCH_AMP_GAIN", "PATCH_AMP_BASS",
                    "PATCH_AMP_MIDDLE", "PATCH_AMP_TREBLE", "PATCH_AMP_LEVEL"):
            if pid in self.values:
                a = next(k for k, p in PARAMS.items() if p[5] == pid)
                kind, n, ofs, mn, mx, _ = PARAMS[a]
                parts.append(f"{spoken_name(pid)} {spoken_value(pid, kind, ofs, mn, mx, self.values[pid])}")
        return ", ".join(parts) or "Nothing read yet"


def wait_for(inp, test, timeout=2.0):
    end = time.monotonic() + timeout
    while time.monotonic() < end:
        msg = inp.poll()
        if msg is None:
            time.sleep(0.005)
            continue
        r = test(msg)
        if r is not None:
            return r
    return None


def pick_port(names, wanted="IR-200"):
    return next((n for n in names if wanted.lower() in n.lower()), None)


def main():
    say = make_speaker()
    in_name = pick_port(mido.get_input_names())
    out_name = pick_port(mido.get_output_names())
    if not in_name or not out_name:
        text = "IR-200 not found. Is it on USB, and is IR Loader closed?"
        print(text)
        say(text)
        sys.exit(1)

    log = LOG.open("a", encoding="utf-8")
    def record(line):
        stamp = datetime.now().strftime("%H:%M:%S.%f")[:-3]
        log.write(f"{stamp} {line}\n")
        log.flush()
        print(stamp, line)

    reader = Reader(say, record)
    record(f"--- start, in={in_name!r} out={out_name!r}")

    with mido.open_input(in_name) as inp, mido.open_output(out_name) as out:
        r = Roland(out)

        # 1. Identity request; take the device ID from the reply.
        out.send(mido.Message("sysex", data=[0x7E, 0x7F, 0x06, 0x01]))
        def identity(msg):
            d = list(msg.data) if msg.type == "sysex" else []
            if len(d) >= 4 and d[0] == 0x7E and d[2:4] == [0x06, 0x02]:
                return d
        reply = wait_for(inp, identity)
        if reply:
            r.device = reply[1]
            record(f"identity reply: {' '.join(f'{b:02X}' for b in reply)}")
        else:
            record("no identity reply; trying anyway")

        def dt1_at(addr):
            def test(msg):
                if msg.type != "sysex":
                    return None
                p = r.parse_dt1(list(msg.data))
                if p and p[0] == addr:
                    return p[1]
            return test

        # 2. Communication level, then 3. editor mode on.
        r.rq1(ADDR_COMM_LEVEL, 1)
        level = wait_for(inp, dt1_at(ADDR_COMM_LEVEL))
        record(f"communication level: {level}")
        r.dt1(ADDR_COMM_MODE, [1])
        record("editor mode on")

        try:
            # 4. Read the current memory number, name and amp block.
            for addr, size in ((ADDR_PATCH_NUMBER, 2), (ADDR_TEMP_NAME, 16), (ADDR_TEMP_AMP, 9)):
                r.rq1(addr, size)
                data = wait_for(inp, dt1_at(addr))
                if data is not None:
                    reader.handle_dt1(addr, data, quiet=True)
            reader.say("IR-200 ready. " + reader.summary())

            run(inp, r, reader, record)
        finally:
            r.dt1(ADDR_COMM_MODE, [0])
            time.sleep(0.2)
            record("editor mode off --- end")
            say("Stopped")


def run(inp, r, reader, record):
    try:
        import msvcrt
    except ImportError:
        msvcrt = None

    while True:
        if msvcrt and msvcrt.kbhit():
            key = msvcrt.getwch().lower()
            if key == "q":
                return
            if key == "r":
                reader.say(reader.summary())
            if key == "s":
                reader.say(reader.last)

        if "memory" in reader.pending:
            reader.pending.clear()
            reader.memory_since = time.monotonic()
            r.rq1(ADDR_TEMP_NAME, 16)
            r.rq1(ADDR_TEMP_AMP, 9)
        if reader.memory_since and time.monotonic() - reader.memory_since > 0.7:
            reader.memory_since = None   # the name never came; say the number
            reader.say(reader.memory_text())

        try:
            msg = inp.poll()
        except KeyboardInterrupt:
            return
        if msg is None:
            try:
                time.sleep(0.005)
            except KeyboardInterrupt:
                return
            continue
        if msg.type in ("clock", "active_sensing"):
            continue
        if msg.type == "sysex":
            d = list(msg.data)
            p = r.parse_dt1(d)
            if p:
                record(f"DT1 {p[0]:08X} {' '.join(f'{b:02X}' for b in p[1])}")
                reader.handle_dt1(*p)
                continue
            record(f"UNKNOWN SYSEX F0 {' '.join(f'{b:02X}' for b in d)} F7")
            continue
        if msg.type == "program_change":
            record(f"PC {msg.program}")
            if reader.values.get("SETUP_COM_PATCH_NUMBER") != msg.program:
                reader.values["SETUP_COM_PATCH_NUMBER"] = msg.program
                reader.pending.append("memory")
            continue
        record(f"OTHER {msg}")


if __name__ == "__main__":
    main()
