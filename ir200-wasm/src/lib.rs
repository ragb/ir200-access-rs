//! WASM bindings: exposes `ir200-core` to the able-midi editor.
//!
//! Frame builders return complete SysEx (one or more `F0 … F7` frames
//! concatenated); decoders take the concatenated DT1 frames the pedal sent.
//! Typed documents (`Memory`, `System`, `ProgramChangeMap`) render as
//! TypeScript types through the `tsify` derives in the core crate.

use ir200_core::address::{
    add_offset, from_u28, ir_slot_base, memory_base, size_field, to_u28, AddressSpace,
    CMD_COMM_MODE, IR_NAME_LEN, SETUP_BASE, TEMPORARY_BASE,
};
use ir200_core::codec::Block;
use ir200_core::device::{
    collect, data_set_frames, decode_memory_at, dt1_frames, encode_memory_at, request_frames,
};
use ir200_core::sysex::DEVICE_ID_DEFAULT;
use ir200_core::{
    classify_inbound as core_classify, identify, identity_request, Change, Frame, InboundMessage,
    Memory, ProgramChangeMap, Setup, System,
};
use midi_access_core::Catalogs;
use serde::Serialize;
use tsify_next::Tsify;
use wasm_bindgen::prelude::*;

fn js_err(e: impl std::fmt::Display) -> JsError {
    JsError::new(&e.to_string())
}

fn addr4(address: &[u8]) -> Result<[u8; 4], JsError> {
    address
        .try_into()
        .map_err(|_| JsError::new("address must be 4 bytes"))
}

/// The device ID the IR-200 uses (`0x10`).
#[wasm_bindgen(js_name = deviceId)]
pub fn device_id() -> u8 {
    DEVICE_ID_DEFAULT
}

// === Identity and editor mode ===

/// Universal Identity Request, broadcast.
#[wasm_bindgen(js_name = identityRequest)]
pub fn identity_request_wasm() -> Vec<u8> {
    identity_request(0x7F)
}

/// `true`/`false` for an IR-200 / other Identity Reply, `undefined` otherwise.
#[wasm_bindgen(js_name = identifyIr200)]
pub fn identify_ir200(bytes: &[u8]) -> Option<bool> {
    identify(bytes)
}

/// DT1 switching editor communication mode on or off. With it on, the pedal
/// pushes a DT1 for every knob, switch and memory change. Always switch it
/// off again when disconnecting.
#[wasm_bindgen(js_name = editorModeFrame)]
pub fn editor_mode_frame(on: bool) -> Vec<u8> {
    Frame::data_set(DEVICE_ID_DEFAULT, CMD_COMM_MODE, vec![on as u8]).encode()
}

// === Raw frames ===

/// RQ1 frames for `len` bytes at `address` (split at 128 bytes).
#[wasm_bindgen(js_name = dataRequest)]
pub fn data_request(address: &[u8], len: usize) -> Result<Vec<u8>, JsError> {
    Ok(request_frames(DEVICE_ID_DEFAULT, addr4(address)?, len))
}

/// DT1 frames writing `data` at `address` (split at 128 bytes).
#[wasm_bindgen(js_name = dataSet)]
pub fn data_set(address: &[u8], data: &[u8]) -> Result<Vec<u8>, JsError> {
    Ok(data_set_frames(DEVICE_ID_DEFAULT, addr4(address)?, data))
}

// === Setup: current memory ===

/// RQ1 for the current memory number.
#[wasm_bindgen(js_name = setupRequest)]
pub fn setup_request() -> Vec<u8> {
    request_frames(DEVICE_ID_DEFAULT, SETUP_BASE, Setup::BLOCK.len)
}

/// Memory number 1–128 from the setup block's data (2 bytes).
#[wasm_bindgen(js_name = decodeCurrentMemory)]
pub fn decode_current_memory(data: &[u8]) -> Result<u8, JsError> {
    Ok(Setup::from_bytes(data).map_err(js_err)?.memory())
}

/// DT1 selecting memory `n` (1–128). The pedal loads it into the temporary
/// memory; read that afterwards with `memoryRequest(0)`.
#[wasm_bindgen(js_name = selectMemoryFrame)]
pub fn select_memory_frame(n: u8) -> Result<Vec<u8>, JsError> {
    let s = Setup::for_memory(n).ok_or_else(|| JsError::new("memory must be 1-128"))?;
    Ok(data_set_frames(
        DEVICE_ID_DEFAULT,
        SETUP_BASE,
        &s.to_bytes().map_err(js_err)?,
    ))
}

// === Memories ===

fn memory_base_or_temp(n: u8) -> Result<[u8; 4], JsError> {
    if n == 0 {
        Ok(TEMPORARY_BASE)
    } else {
        memory_base(n).ok_or_else(|| JsError::new("memory must be 0 (temporary) or 1-128"))
    }
}

/// Base address of memory `n`: 0 = temporary memory, 1–128 = stored.
#[wasm_bindgen(js_name = memoryBase)]
pub fn memory_base_wasm(n: u8) -> Result<Vec<u8>, JsError> {
    Ok(memory_base_or_temp(n)?.to_vec())
}

/// One RQ1 for a whole memory. The pedal answers with one DT1 per block
/// (`memoryBlockCount()` of them).
#[wasm_bindgen(js_name = memoryRequest)]
pub fn memory_request(n: u8) -> Result<Vec<u8>, JsError> {
    let base = memory_base_or_temp(n)?;
    let last = Memory::blocks().into_iter().last().unwrap();
    let span = to_u28(last.address()) - to_u28(TEMPORARY_BASE) + last.len as u32;
    Ok(Frame::data_request(DEVICE_ID_DEFAULT, base, from_u28(span)).encode())
}

/// Number of DT1 blocks in a memory (17).
#[wasm_bindgen(js_name = memoryBlockCount)]
pub fn memory_block_count() -> usize {
    Memory::blocks().len()
}

/// Decode memory `n` (0 = temporary) from the concatenated DT1 frames.
#[wasm_bindgen(js_name = decodeMemory)]
pub fn decode_memory(frames: &[u8], n: u8) -> Result<Memory, JsError> {
    decode_memory_at(&dt1_frames(frames), memory_base_or_temp(n)?).map_err(js_err)
}

/// DT1 frames writing `memory` to memory `n` (0 = temporary, 1–128 = stored;
/// writing a stored memory saves it).
#[wasm_bindgen(js_name = encodeMemory)]
pub fn encode_memory(memory: Memory, n: u8) -> Result<Vec<u8>, JsError> {
    encode_memory_at(&memory, memory_base_or_temp(n)?, DEVICE_ID_DEFAULT).map_err(js_err)
}

/// IR Loader's INIT MEMORY.
#[wasm_bindgen(js_name = newMemory)]
pub fn new_memory() -> Memory {
    Memory::init_memory()
}

/// Which memory a DT1 address falls in: 0 = temporary, 1–128 = stored,
/// `undefined` outside memory space.
#[wasm_bindgen(js_name = memoryOfAddress)]
pub fn memory_of_address(address: &[u8]) -> Option<u8> {
    match AddressSpace::classify(addr4(address).ok()?) {
        AddressSpace::Temporary => Some(0),
        AddressSpace::Memory(n) => Some(n),
        _ => None,
    }
}

// === System ===

/// RQ1 frames for every system block.
#[wasm_bindgen(js_name = systemRequest)]
pub fn system_request() -> Vec<u8> {
    System::blocks()
        .into_iter()
        .flat_map(|b| request_frames(DEVICE_ID_DEFAULT, b.address(), b.len))
        .collect()
}

/// Number of DT1 blocks in the system area (19).
#[wasm_bindgen(js_name = systemBlockCount)]
pub fn system_block_count() -> usize {
    System::blocks().len()
}

/// Decode the system area from the concatenated DT1 frames.
#[wasm_bindgen(js_name = decodeSystem)]
pub fn decode_system(frames: &[u8]) -> Result<System, JsError> {
    let f = dt1_frames(frames);
    let data: Vec<(&Block, Vec<u8>)> = System::blocks()
        .into_iter()
        .map(|b| collect(&f, b.address(), b.len).map(|d| (b, d)))
        .collect::<Result<_, _>>()
        .map_err(js_err)?;
    System::from_blocks(|b| {
        data.iter()
            .find(|(x, _)| x.addr == b.addr)
            .map(|(_, d)| d.as_slice())
            .ok_or(ir200_core::CodecError::MissingBlock(b.name))
    })
    .map_err(js_err)
}

/// DT1 frames writing the system area.
#[wasm_bindgen(js_name = encodeSystem)]
pub fn encode_system(system: System) -> Result<Vec<u8>, JsError> {
    let mut out = Vec::new();
    for (b, data) in system.to_blocks().map_err(js_err)? {
        out.extend(data_set_frames(DEVICE_ID_DEFAULT, b.address(), &data));
    }
    Ok(out)
}

/// IR Loader's initial system settings (for offline mode).
#[wasm_bindgen(js_name = newSystem)]
pub fn new_system() -> System {
    System::init()
}

// === Program Change map ===

const PC_MAP: &Block = &ir200_core::map::midi_pcmap::BLOCK;

#[wasm_bindgen(js_name = pcMapRequest)]
pub fn pc_map_request() -> Vec<u8> {
    request_frames(DEVICE_ID_DEFAULT, PC_MAP.address(), PC_MAP.len)
}

#[wasm_bindgen(js_name = pcMapLength)]
pub fn pc_map_length() -> usize {
    PC_MAP.len
}

#[wasm_bindgen(js_name = decodePcMap)]
pub fn decode_pc_map(frames: &[u8]) -> Result<ProgramChangeMap, JsError> {
    let d = collect(&dt1_frames(frames), PC_MAP.address(), PC_MAP.len).map_err(js_err)?;
    ProgramChangeMap::from_bytes(&d).map_err(js_err)
}

#[wasm_bindgen(js_name = encodePcMap)]
pub fn encode_pc_map(map: ProgramChangeMap) -> Result<Vec<u8>, JsError> {
    Ok(data_set_frames(
        DEVICE_ID_DEFAULT,
        PC_MAP.address(),
        &map.to_bytes().map_err(js_err)?,
    ))
}

#[wasm_bindgen(js_name = newPcMap)]
pub fn new_pc_map() -> ProgramChangeMap {
    ProgramChangeMap::init()
}

// === User IR slots ===

/// A user IR slot's name and whether it holds an IR.
#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
pub struct IrSlot {
    pub slot: u8,
    pub name: String,
    /// `false` when the slot is empty (no file loaded).
    pub used: bool,
}

fn slot_base(slot: u8) -> Result<[u8; 4], JsError> {
    ir_slot_base(slot).ok_or_else(|| JsError::new("IR slot must be 1-128"))
}

/// RQ1 for user IR slot `slot` (1–128): its name and file-name length, 36
/// bytes (the pedal answers with one DT1).
#[wasm_bindgen(js_name = irSlotRequest)]
pub fn ir_slot_request(slot: u8) -> Result<Vec<u8>, JsError> {
    let base = slot_base(slot)?;
    Ok(Frame::data_request(DEVICE_ID_DEFAULT, base, size_field(IR_NAME_LEN + 4)).encode())
}

/// Decode a slot's name and use from the concatenated DT1 replies.
#[wasm_bindgen(js_name = decodeIrSlot)]
pub fn decode_ir_slot(frames: &[u8], slot: u8) -> Result<IrSlot, JsError> {
    let base = slot_base(slot)?;
    let f = dt1_frames(frames);
    // One read covers the name and the file-name length right after it; the
    // pedal answers in a single DT1, so collect it as one run from the base.
    let bytes = collect(&f, base, IR_NAME_LEN + 4).map_err(js_err)?;
    let (name, len) = bytes.split_at(IR_NAME_LEN);
    let n = len.iter().fold(0u32, |a, &x| (a << 4) | (x & 0x0F) as u32);
    let name: String = name
        .iter()
        .map(|&c| {
            if (0x20..0x7F).contains(&c) {
                c as char
            } else {
                ' '
            }
        })
        .collect();
    Ok(IrSlot {
        slot,
        name: name.split_whitespace().collect::<Vec<_>>().join(" "),
        used: n > 0,
    })
}

/// DT1 renaming user IR slot `slot` (up to 32 ASCII characters).
#[wasm_bindgen(js_name = irRenameFrame)]
pub fn ir_rename_frame(slot: u8, name: &str) -> Result<Vec<u8>, JsError> {
    if name.len() > IR_NAME_LEN || !name.bytes().all(|c| (0x20..0x7F).contains(&c)) {
        return Err(JsError::new("IR name: up to 32 printable ASCII characters"));
    }
    let mut b = name.as_bytes().to_vec();
    b.resize(IR_NAME_LEN, b' ');
    Ok(Frame::data_set(DEVICE_ID_DEFAULT, slot_base(slot)?, b).encode())
}

/// What a WAV file holds, for the load dialog.
#[derive(Serialize, Tsify)]
#[serde(rename_all = "camelCase")]
#[tsify(into_wasm_abi)]
pub struct WavInfo {
    pub channels: u16,
    pub sample_rate: u32,
    pub bits: u16,
    /// Length in seconds.
    pub seconds: f64,
}

/// Read a WAV file's format. Errors with a plain-language reason when the
/// pedal can't take it.
#[wasm_bindgen(js_name = wavInfo)]
pub fn wav_info(data: &[u8]) -> Result<WavInfo, JsError> {
    let w = ir200_core::irconv::read_wav(data).map_err(js_err)?;
    Ok(WavInfo {
        channels: w.channels,
        sample_rate: w.rate,
        bits: w.bits,
        seconds: w.frames() as f64 / w.rate as f64,
    })
}

/// Offset of the coefficient block in a user IR slot.
const IR_COEF_OFFSET: [u8; 4] = [0x00, 0x00, 0x02, 0x24];
/// Offset of the file name (UTF-8, two nibbles per byte) in a slot.
const IR_FILE_NAME_OFFSET: [u8; 4] = [0x00, 0x00, 0x00, 0x24];

/// Convert `wav` and build every DT1 that loads it into user IR `slot`, in
/// IR Loader's order: name (32 bytes), file name (120 bytes per message),
/// coefficients (30 values per message). Send them a few ms apart. The pedal
/// sets the file-name length itself, which marks the slot used.
#[wasm_bindgen(js_name = irLoadFrames)]
pub fn ir_load_frames(
    slot: u8,
    wav: &[u8],
    channel: u8,
    name: &str,
    file_name: &str,
) -> Result<Vec<u8>, JsError> {
    let base = slot_base(slot)?;
    let coefs = ir200_core::irconv::convert_wav(wav, channel as usize).map_err(js_err)?;
    let mut out = ir_rename_frame(slot, name)?;
    // File name: UTF-8, under 128 bytes, cut on a character boundary.
    let mut end = file_name.len().min(127);
    while !file_name.is_char_boundary(end) {
        end -= 1;
    }
    let nib: Vec<u8> = file_name.as_bytes()[..end]
        .iter()
        .flat_map(|b| [b >> 4, b & 0x0F])
        .collect();
    let mut a = to_u28(add_offset(base, IR_FILE_NAME_OFFSET));
    for chunk in nib.chunks(240) {
        out.extend(Frame::data_set(DEVICE_ID_DEFAULT, from_u28(a), chunk.to_vec()).encode());
        a += chunk.len() as u32;
    }
    let data = ir200_core::irconv::to_nibbles(&coefs);
    let mut a = to_u28(add_offset(base, IR_COEF_OFFSET));
    for chunk in data.chunks(240) {
        out.extend(Frame::data_set(DEVICE_ID_DEFAULT, from_u28(a), chunk.to_vec()).encode());
        a += chunk.len() as u32;
    }
    Ok(out)
}

/// First cab type that is a user IR (user IR n = this + n − 1).
#[wasm_bindgen(js_name = userIrFirstCabType)]
pub fn user_ir_first_cab_type() -> u16 {
    ir200_core::catalog::USER_IR_FIRST_CAB_TYPE
}

// === Live changes ===

/// Describe one inbound DT1 as the parameters it changed, with labels and
/// spoken values ("Gain", "61").
#[wasm_bindgen(js_name = describeDataSet)]
pub fn describe_data_set(address: &[u8], data: &[u8]) -> Result<Vec<Change>, JsError> {
    Ok(ir200_core::describe(addr4(address)?, data))
}

// === Catalog / metadata ===

/// The catalog bundle `{ device, params, catalogs, defaults }`.
#[wasm_bindgen(js_name = catalog)]
pub fn catalog() -> Result<JsValue, JsError> {
    serde_wasm_bindgen::to_value(&ir200_core::catalog::bundle()).map_err(js_err)
}

/// The parameter table: `[{ path, label, group, help, level?, kind?, catalog? }]`.
#[wasm_bindgen(js_name = paramCatalog)]
pub fn param_catalog() -> Result<JsValue, JsError> {
    serde_wasm_bindgen::to_value(ir200_core::catalog::params().as_slice()).map_err(js_err)
}

/// Help text for a field path, e.g. `"memory.amp.gain"`.
#[wasm_bindgen(js_name = fieldHelp)]
pub fn field_help(path: &str) -> Option<String> {
    ir200_core::catalog::params().help(path).map(String::from)
}

/// The names of a catalog, indexed by stored value (e.g. `"cab_type"`).
#[wasm_bindgen(js_name = catalogList)]
pub fn catalog_list(name: &str) -> Vec<String> {
    let c = &ir200_core::IR200_CATALOGS;
    (0..).map_while(|i| c.label(name, i)).collect()
}

// === YAML ===

#[wasm_bindgen(js_name = memoryToYaml)]
pub fn memory_to_yaml(memory: Memory) -> Result<String, JsError> {
    ir200_core::yaml::memory_to_yaml(&memory).map_err(js_err)
}

/// Parse a memory from YAML; value names (`cab_type: 412 BRIT STK DYN57-S`)
/// are resolved to numbers first.
#[wasm_bindgen(js_name = memoryFromYaml)]
pub fn memory_from_yaml(text: &str) -> Result<Memory, JsError> {
    let resolved = resolve(text)?;
    ir200_core::yaml::memory_from_yaml(&resolved).map_err(js_err)
}

#[wasm_bindgen(js_name = systemToYaml)]
pub fn system_to_yaml(system: System) -> Result<String, JsError> {
    ir200_core::yaml::system_to_yaml(&system).map_err(js_err)
}

#[wasm_bindgen(js_name = systemFromYaml)]
pub fn system_from_yaml(text: &str) -> Result<System, JsError> {
    let resolved = resolve(text)?;
    ir200_core::yaml::system_from_yaml(&resolved).map_err(js_err)
}

fn resolve(text: &str) -> Result<String, JsError> {
    midi_access_core::resolve_names_str(
        text,
        ir200_core::catalog::params(),
        &ir200_core::IR200_CATALOGS,
    )
    .map_err(js_err)
}

// === Inbound classification ===

#[derive(Serialize, Tsify)]
#[tsify(into_wasm_abi)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum WasmInbound {
    DataSet {
        #[tsify(type = "Uint8Array")]
        address: Vec<u8>,
        #[tsify(type = "Uint8Array")]
        data: Vec<u8>,
    },
    DataRequest {
        #[tsify(type = "Uint8Array")]
        address: Vec<u8>,
    },
    Other,
}

/// Classify a complete inbound MIDI message.
#[wasm_bindgen(js_name = classifyInbound)]
pub fn classify_inbound(bytes: &[u8]) -> WasmInbound {
    match core_classify(bytes) {
        InboundMessage::DataSet { address, data, .. } => WasmInbound::DataSet {
            address: address.to_vec(),
            data,
        },
        InboundMessage::DataRequest { address, .. } => WasmInbound::DataRequest {
            address: address.to_vec(),
        },
        _ => WasmInbound::Other,
    }
}

#[cfg(test)]
mod tests {
    use wasm_bindgen_test::wasm_bindgen_test;

    #[wasm_bindgen_test]
    fn memory_request_spans_all_blocks() {
        let f = super::memory_request(0).unwrap();
        // RQ1 20 00 00 00, size 00 02 00 0F (the span verified on hardware).
        assert_eq!(&f[9..17], &[0x20, 0, 0, 0, 0x00, 0x02, 0x00, 0x0F]);
        assert_eq!(super::memory_block_count(), 17);
    }

    #[wasm_bindgen_test]
    fn catalog_lists() {
        assert_eq!(super::catalog_list("cab_type").len(), 337);
        assert_eq!(super::catalog_list("amp_type")[1], "NATURAL");
    }
}

#[cfg(test)]
mod ir_slot_tests {
    use super::*;

    #[test]
    fn decodes_the_pedals_single_dt1_reply() {
        // Real reply to RQ1 68 02 00 00 size 36 (user IR 2).
        let hex = "F0 41 10 00 00 00 00 06 12 68 02 00 00 54 46 20 36 35 20 54 57 49 4E 20 32                    58 31 32 20 20 20 20 20 20 20 20 20 20 20 20 20 20 20 20 20 00 00 03 04 5B F7";
        let reply: Vec<u8> = hex
            .split_whitespace()
            .map(|b| u8::from_str_radix(b, 16).unwrap())
            .collect();
        let Ok(s) = decode_ir_slot(&reply, 2) else {
            panic!("the reply should decode");
        };
        assert_eq!(s.name, "TF 65 TWIN 2X12");
        assert!(s.used);
    }
}
