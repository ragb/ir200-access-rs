//! [`Device`] implementation for the BOSS IR-200: plugs the typed codec into
//! the generic CLI engine (`midi-access-cli`) with `serde_yaml::Value` as the
//! document format.
//!
//! ## Areas
//!
//! - `setup`  — the current memory number, `00 00 00 00`.
//! - `system` — global settings, 19 blocks from `10 00 00 00`.
//! - `edit`   — the temporary memory (what the pedal plays), 17 blocks from
//!   `20 00 00 00`.
//! - `pc-map` — the MIDI Program Change map, `50 00 00 00`.
//!
//! Each leaf block is requested with its own RQ1, as IR Loader does. Stored
//! memories 1–128 are reached through `--store <n>` (copy the document to that
//! memory) and `load <n>` (select it), and from the wasm layer.

use serde_yaml::Value;

use midi_access_core::{split_sysex, Area, Catalogs, Device, DeviceError, Inbound, Params};

use crate::address::{
    from_u28, memory_base, size_field, to_u28, AddressSpace, SETUP_BASE, TEMPORARY_BASE,
};
use crate::codec::{Block, CodecError};
use crate::memory::Memory;
use crate::pc_map::ProgramChangeMap;
use crate::setup::Setup;
use crate::sysex::{identify, Frame, CMD_DT1, DEVICE_ID_DEFAULT};
use crate::system::System;

/// The BOSS IR-200.
pub struct Ir200;

/// Largest RQ1/DT1 payload, as IR Loader splits them (`SYSEX_MAXLEN`).
pub const MAX_CHUNK: usize = 128;

const AREAS: &[Area] = &[
    Area {
        name: "setup",
        label: "Setup",
        about: "The current memory number",
    },
    Area {
        name: "system",
        label: "System",
        about: "Global settings: outputs, global EQ, MIDI, USB, controls",
    },
    Area {
        name: "edit",
        label: "Memory",
        about: "The temporary memory: amp, cabinets, ambience, EQ, controls",
    },
    Area {
        name: "pc-map",
        label: "Program Change map",
        about: "Which memory each MIDI Program Change selects",
    },
];

enum Target {
    Setup,
    System,
    Edit,
    PcMap,
}

fn target(area: &str) -> Option<Target> {
    Some(match area {
        "setup" => Target::Setup,
        "system" => Target::System,
        "edit" => Target::Edit,
        "pc-map" => Target::PcMap,
        _ => return None,
    })
}

impl Target {
    fn blocks(&self) -> Vec<&'static Block> {
        match self {
            Target::Setup => vec![Setup::BLOCK],
            Target::System => System::blocks(),
            Target::Edit => Memory::blocks(),
            Target::PcMap => vec![&crate::map::midi_pcmap::BLOCK],
        }
    }
}

/// Wire device ID. The IR-200 has no device-ID setting and IR Loader addresses
/// it as `10`; the engine's `--device` channel is not used.
fn device_id(_ch: u8) -> u8 {
    DEVICE_ID_DEFAULT
}

fn dec(e: impl std::fmt::Display) -> DeviceError {
    DeviceError::Decode(e.to_string())
}
fn enc(e: impl std::fmt::Display) -> DeviceError {
    DeviceError::Encode(e.to_string())
}
fn unknown(area: &str) -> DeviceError {
    DeviceError::UnknownArea(area.to_string())
}

/// RQ1 frames reading `len` bytes at `address`, split into [`MAX_CHUNK`] pieces.
pub fn request_frames(id: u8, address: [u8; 4], len: usize) -> Vec<u8> {
    let mut out = Vec::new();
    let mut a = to_u28(address);
    let mut left = len;
    while left > 0 {
        let n = left.min(MAX_CHUNK);
        out.extend(Frame::data_request(id, from_u28(a), size_field(n)).encode());
        a += n as u32;
        left -= n;
    }
    out
}

/// DT1 frames writing `data` at `address`, split into [`MAX_CHUNK`] pieces.
pub fn data_set_frames(id: u8, address: [u8; 4], data: &[u8]) -> Vec<u8> {
    let mut out = Vec::new();
    let mut a = to_u28(address);
    for chunk in data.chunks(MAX_CHUNK) {
        out.extend(Frame::data_set(id, from_u28(a), chunk.to_vec()).encode());
        a += chunk.len() as u32;
    }
    out
}

/// Every DT1 in a collected dump.
pub fn dt1_frames(dump: &[u8]) -> Vec<Frame> {
    split_sysex(dump)
        .into_iter()
        .filter_map(|f| Frame::decode(&f).ok())
        .filter(|f| f.command == CMD_DT1)
        .collect()
}

/// Reassemble `len` bytes starting at `base` from DT1 frames whose addresses
/// chain contiguously from `base`.
pub fn collect(frames: &[Frame], base: [u8; 4], len: usize) -> Result<Vec<u8>, CodecError> {
    let mut out = Vec::with_capacity(len);
    let mut a = to_u28(base);
    while out.len() < len {
        match frames.iter().rev().find(|f| to_u28(f.address) == a) {
            Some(f) if !f.data.is_empty() => {
                out.extend_from_slice(&f.data);
                a += f.data.len() as u32;
            }
            _ => break,
        }
    }
    if out.len() < len {
        return Err(CodecError::WrongLength {
            expected: len,
            actual: out.len(),
        });
    }
    out.truncate(len);
    Ok(out)
}

/// Decode a memory from DT1 frames, with its blocks relocated to `base`
/// (`TEMPORARY_BASE` or a stored memory's base).
pub fn decode_memory_at(frames: &[Frame], base: [u8; 4]) -> Result<Memory, CodecError> {
    let blocks: Vec<(&Block, Vec<u8>)> = Memory::blocks()
        .into_iter()
        .map(|b| collect(frames, b.relocated(TEMPORARY_BASE, base), b.len).map(|d| (b, d)))
        .collect::<Result<_, _>>()?;
    Memory::from_blocks(|b| {
        blocks
            .iter()
            .find(|(x, _)| x.addr == b.addr)
            .map(|(_, d)| d.as_slice())
            .ok_or(CodecError::MissingBlock(b.name))
    })
}

/// DT1 frames writing `memory` with its blocks relocated to `base`.
pub fn encode_memory_at(memory: &Memory, base: [u8; 4], id: u8) -> Result<Vec<u8>, CodecError> {
    let mut out = Vec::new();
    for (b, data) in memory.to_blocks()? {
        out.extend(data_set_frames(
            id,
            b.relocated(TEMPORARY_BASE, base),
            &data,
        ));
    }
    Ok(out)
}

fn decode_system(frames: &[Frame]) -> Result<System, CodecError> {
    let blocks: Vec<(&Block, Vec<u8>)> = System::blocks()
        .into_iter()
        .map(|b| collect(frames, b.address(), b.len).map(|d| (b, d)))
        .collect::<Result<_, _>>()?;
    System::from_blocks(|b| {
        blocks
            .iter()
            .find(|(x, _)| x.addr == b.addr)
            .map(|(_, d)| d.as_slice())
            .ok_or(CodecError::MissingBlock(b.name))
    })
}

/// Parse a `--store` / `load` destination: a memory number 1–128.
fn memory_dest(dest: &str) -> Result<u8, DeviceError> {
    dest.trim()
        .parse::<u8>()
        .ok()
        .filter(|n| (1..=128).contains(n))
        .ok_or_else(|| DeviceError::Other(format!("memory must be 1-128, got {dest:?}")))
}

impl Device for Ir200 {
    const NAME: &'static str = "ir200";

    fn areas() -> &'static [Area] {
        AREAS
    }

    fn params() -> Params {
        crate::catalog::params()
    }

    fn catalogs() -> &'static dyn Catalogs {
        &crate::catalog::IR200_CATALOGS
    }

    fn defaults(area: &str) -> Option<Value> {
        match target(area)? {
            Target::Setup => serde_yaml::to_value(Setup { memory_index: 0 }).ok(),
            Target::System => serde_yaml::to_value(System::init()).ok(),
            Target::Edit => serde_yaml::to_value(Memory::init_memory()).ok(),
            Target::PcMap => serde_yaml::to_value(ProgramChangeMap::init()).ok(),
        }
    }

    fn schema(area: &str) -> Option<String> {
        #[cfg(feature = "schema")]
        {
            use midi_access_core::schema::schema_json;
            Some(match target(area)? {
                Target::Setup => schema_json::<Setup>(),
                Target::System => schema_json::<System>(),
                Target::Edit => schema_json::<Memory>(),
                Target::PcMap => schema_json::<ProgramChangeMap>(),
            })
        }
        #[cfg(not(feature = "schema"))]
        {
            let _ = area;
            None
        }
    }

    fn request(area: &str, ch: u8) -> Result<Vec<u8>, DeviceError> {
        let id = device_id(ch);
        let mut out = Vec::new();
        for b in target(area).ok_or_else(|| unknown(area))?.blocks() {
            out.extend(request_frames(id, b.address(), b.len));
        }
        Ok(out)
    }

    fn decode(area: &str, dump: &[u8]) -> Result<Value, DeviceError> {
        let frames = dt1_frames(dump);
        match target(area).ok_or_else(|| unknown(area))? {
            Target::Setup => {
                let d = collect(&frames, SETUP_BASE, Setup::BLOCK.len).map_err(dec)?;
                serde_yaml::to_value(Setup::from_bytes(&d).map_err(dec)?).map_err(dec)
            }
            Target::System => {
                serde_yaml::to_value(decode_system(&frames).map_err(dec)?).map_err(dec)
            }
            Target::Edit => {
                serde_yaml::to_value(decode_memory_at(&frames, TEMPORARY_BASE).map_err(dec)?)
                    .map_err(dec)
            }
            Target::PcMap => {
                let b = &crate::map::midi_pcmap::BLOCK;
                let d = collect(&frames, b.address(), b.len).map_err(dec)?;
                serde_yaml::to_value(ProgramChangeMap::from_bytes(&d).map_err(dec)?).map_err(dec)
            }
        }
    }

    fn encode(area: &str, doc: &Value, ch: u8) -> Result<Vec<u8>, DeviceError> {
        let id = device_id(ch);
        match target(area).ok_or_else(|| unknown(area))? {
            Target::Setup => {
                let s: Setup = serde_yaml::from_value(doc.clone()).map_err(enc)?;
                Ok(data_set_frames(id, SETUP_BASE, &s.to_bytes().map_err(enc)?))
            }
            Target::System => {
                let s: System = serde_yaml::from_value(doc.clone()).map_err(enc)?;
                let mut out = Vec::new();
                for (b, data) in s.to_blocks().map_err(enc)? {
                    out.extend(data_set_frames(id, b.address(), &data));
                }
                Ok(out)
            }
            Target::Edit => {
                let m: Memory = serde_yaml::from_value(doc.clone()).map_err(enc)?;
                encode_memory_at(&m, TEMPORARY_BASE, id).map_err(enc)
            }
            Target::PcMap => {
                let m: ProgramChangeMap = serde_yaml::from_value(doc.clone()).map_err(enc)?;
                let b = &crate::map::midi_pcmap::BLOCK;
                Ok(data_set_frames(
                    id,
                    b.address(),
                    &m.to_bytes().map_err(enc)?,
                ))
            }
        }
    }

    /// Copy-to-slot store: write the memory document to stored memory `dest`
    /// (1–128). The IR-200's own write command (`7F 00 01 04`) is not used.
    fn store(area: &str, doc: &Value, dest: &str, ch: u8) -> Option<Result<Vec<u8>, DeviceError>> {
        if area != "edit" {
            return None;
        }
        Some((|| {
            let n = memory_dest(dest)?;
            let m: Memory = serde_yaml::from_value(doc.clone()).map_err(enc)?;
            encode_memory_at(&m, memory_base(n).unwrap(), device_id(ch)).map_err(enc)
        })())
    }

    /// Select memory `dest` (1–128) by writing the setup memory number.
    fn recall(dest: &str, _channel: u8) -> Option<Result<Vec<Vec<u8>>, DeviceError>> {
        Some(memory_dest(dest).map(|n| {
            let s = Setup::for_memory(n).unwrap();
            vec![Frame::data_set(DEVICE_ID_DEFAULT, SETUP_BASE, s.to_bytes().unwrap()).encode()]
        }))
    }

    fn classify_inbound(bytes: &[u8]) -> Inbound {
        if let Some(is_ir200) = identify(bytes) {
            return Inbound::Identity {
                bytes: bytes.to_vec(),
                model: is_ir200.then(|| "IR-200".to_string()),
            };
        }
        use crate::inbound::{classify_inbound as core, InboundMessage};
        match core(bytes) {
            InboundMessage::DataSet {
                space,
                address,
                data,
                ..
            } => Inbound::Dump {
                area: match space {
                    AddressSpace::Setup => Some("setup".into()),
                    AddressSpace::System => Some("system".into()),
                    AddressSpace::Temporary => Some("edit".into()),
                    AddressSpace::PcMap => Some("pc-map".into()),
                    _ => None,
                },
                address: address.to_vec(),
                data,
            },
            InboundMessage::DataRequest { address, .. } => Inbound::Request {
                address: address.to_vec(),
            },
            InboundMessage::UnparseableSysEx { bytes, .. } | InboundMessage::NonSysEx(bytes) => {
                Inbound::Other(bytes)
            }
        }
    }

    fn accepts(area: &str, doc: &Value) -> bool {
        let v = doc.clone();
        match target(area) {
            Some(Target::Setup) => serde_yaml::from_value::<Setup>(v).is_ok(),
            Some(Target::System) => serde_yaml::from_value::<System>(v).is_ok(),
            Some(Target::Edit) => serde_yaml::from_value::<Memory>(v).is_ok(),
            Some(Target::PcMap) => serde_yaml::from_value::<ProgramChangeMap>(v).is_ok(),
            None => false,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn reply_for(area: &str, doc: &Value) -> Vec<u8> {
        // What the pedal would send back: the same DT1s the encoder writes.
        Ir200::encode(area, doc, 0).unwrap()
    }

    #[test]
    fn every_area_round_trips_through_its_defaults() {
        for a in AREAS {
            let doc = Ir200::defaults(a.name).unwrap();
            let got = Ir200::decode(a.name, &reply_for(a.name, &doc)).unwrap();
            assert_eq!(got, doc, "{}", a.name);
            assert!(Ir200::accepts(a.name, &doc));
        }
    }

    #[test]
    fn requests_one_rq1_per_block() {
        assert_eq!(split_sysex(&Ir200::request("edit", 0).unwrap()).len(), 17);
        assert_eq!(split_sysex(&Ir200::request("system", 0).unwrap()).len(), 19);
        // 256 bytes -> two 128-byte requests.
        let pc = split_sysex(&Ir200::request("pc-map", 0).unwrap());
        assert_eq!(pc.len(), 2);
        assert_eq!(
            Frame::decode(&pc[1]).unwrap().address,
            [0x50, 0x00, 0x01, 0x00]
        );
    }

    #[test]
    fn decode_survives_split_replies() {
        // The pedal may answer one RQ1 with several DT1s; chain them.
        let doc = Ir200::defaults("pc-map").unwrap();
        let m: ProgramChangeMap = serde_yaml::from_value(doc.clone()).unwrap();
        let bytes = m.to_bytes().unwrap();
        let mut dump = Vec::new();
        let mut a = to_u28([0x50, 0, 0, 0]);
        for chunk in bytes.chunks(100) {
            dump.extend(Frame::data_set(0x10, from_u28(a), chunk.to_vec()).encode());
            a += chunk.len() as u32;
        }
        assert_eq!(Ir200::decode("pc-map", &dump).unwrap(), doc);
    }

    #[test]
    fn store_relocates_to_memory() {
        let doc = Ir200::defaults("edit").unwrap();
        let frames = split_sysex(&Ir200::store("edit", &doc, "8", 0).unwrap().unwrap());
        assert_eq!(frames.len(), 17);
        assert_eq!(
            Frame::decode(&frames[0]).unwrap().address,
            [0x21, 0x00, 0x00, 0x00]
        );
        assert_eq!(
            Frame::decode(&frames[1]).unwrap().address,
            [0x21, 0x00, 0x10, 0x00]
        );
        assert!(Ir200::store("edit", &doc, "129", 0).unwrap().is_err());
        assert!(Ir200::store("system", &doc, "1", 0).is_none());
    }

    #[test]
    fn recall_writes_setup() {
        let f = &Ir200::recall("100", 1).unwrap().unwrap()[0];
        let fr = Frame::decode(f).unwrap();
        assert_eq!(fr.address, SETUP_BASE);
        assert_eq!(fr.data, vec![0x06, 0x03]);
    }

    #[test]
    fn identity_and_dump_classification() {
        let reply = [
            0xF0, 0x7E, 0x10, 0x06, 0x02, 0x41, 0x06, 0x04, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00,
            0xF7,
        ];
        assert!(matches!(
            Ir200::classify_inbound(&reply),
            Inbound::Identity { model: Some(m), .. } if m == "IR-200"
        ));
        let dt1 = Frame::data_set(0x10, [0x20, 0x00, 0x10, 0x01], vec![72]).encode();
        assert!(matches!(
            Ir200::classify_inbound(&dt1),
            Inbound::Dump { area: Some(a), .. } if a == "edit"
        ));
    }
}
