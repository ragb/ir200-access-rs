//! Round trips against real IR-200 dumps (captured 2026-11-17 with
//! `tools/explore/ir200probe.py read`, firmware 1.10, factory memories 1-32).
//!
//! Every fixture must decode, and re-encoding must reproduce the pedal's bytes
//! exactly.

use std::collections::BTreeMap;

use ir200_core::address::{memory_base, TEMPORARY_BASE};
use ir200_core::device::{decode_memory_at, dt1_frames, encode_memory_at};
use ir200_core::types::AmbienceType;
use ir200_core::{Ir200, ProgramChangeMap, System};
use midi_access_core::Device;

fn fixture(name: &str) -> Vec<u8> {
    std::fs::read(format!(
        "{}/tests/fixtures/{name}",
        env!("CARGO_MANIFEST_DIR")
    ))
    .unwrap()
}

/// address -> data of every DT1 in a buffer.
fn by_address(bytes: &[u8]) -> BTreeMap<[u8; 4], Vec<u8>> {
    dt1_frames(bytes)
        .into_iter()
        .map(|f| (f.address, f.data))
        .collect()
}

#[test]
fn setup_is_zero_based() {
    let v = Ir200::decode("setup", &fixture("setup.syx")).unwrap();
    // The pedal showed memory 61, "NATURAL BASS".
    assert_eq!(v["memory_index"].as_u64(), Some(60));
}

#[test]
fn system_round_trips_byte_exact() {
    let raw = fixture("system.syx");
    let v = Ir200::decode("system", &raw).unwrap();
    let s: System = serde_yaml::from_value(v.clone()).unwrap();
    assert!(s.modules.amp);
    let again = Ir200::encode("system", &v, 0).unwrap();
    assert_eq!(by_address(&again), by_address(&raw));
}

#[test]
fn temporary_memory_round_trips_byte_exact() {
    let raw = fixture("temporary.syx");
    let v = Ir200::decode("edit", &raw).unwrap();
    assert_eq!(v["name"].as_str(), Some("NATURAL BASS"));
    let again = Ir200::encode("edit", &v, 0).unwrap();
    assert_eq!(by_address(&again), by_address(&raw));
}

#[test]
fn factory_memories_decode_and_round_trip() {
    // (memory, name, amp type, cab A) from the BOSS sound list.
    let expect = [
        (1, "NATURAL 1", 1, 9),
        (2, "JC-120 1", 2, 43),
        (3, "TWIN COMBO 1", 3, 23),
        (4, "TWEED COMBO 1", 5, 51),
        (5, "DIAMOND AMP 1", 4, 25),
        (8, "BGR UB METAL 1", 8, 154),
        (29, "NATURAL BASS", 9, 119),
        (31, "MUTED PICK BASS", 11, 134),
        (32, "X-DRIVE BASS", 10, 119),
    ];
    for n in (1..=32u8).chain([128]) {
        let raw = fixture(&format!("memory-{n:03}.syx"));
        let frames = dt1_frames(&raw);
        let base = memory_base(n).unwrap();
        let m = decode_memory_at(&frames, base).unwrap_or_else(|e| panic!("memory {n}: {e}"));
        let again = encode_memory_at(&m, base, 0x10).unwrap();
        assert_eq!(by_address(&again), by_address(&raw), "memory {n}");
        if let Some(&(_, name, amp, cab)) = expect.iter().find(|e| e.0 == n) {
            assert_eq!(m.name, name);
            assert_eq!(m.amp.amp_type, amp, "memory {n} amp");
            assert_eq!(m.cabinet_a.cab_type, cab, "memory {n} cab");
        }
        assert_eq!(m.ambience.ambience_type, AmbienceType::Hall, "memory {n}");
    }
    // A stored memory is not readable at the temporary address.
    assert!(decode_memory_at(&dt1_frames(&fixture("memory-001.syx")), TEMPORARY_BASE).is_err());
}

#[test]
fn pc_map_round_trips_byte_exact() {
    let raw = fixture("pc-map.syx");
    let v = Ir200::decode("pc-map", &raw).unwrap();
    let m: ProgramChangeMap = serde_yaml::from_value(v.clone()).unwrap();
    assert_eq!(m.memory.len(), 128);
    // Stock entries map PC#n to memory n.
    assert_eq!(m.memory[127], 128);
    assert_eq!(m.memory[40], 41);
    // The pedal answers a 256-byte read in its own frame size (241 + 15), so
    // compare the reassembled bytes, not the frames.
    let base = [0x50, 0, 0, 0];
    let pedal = ir200_core::device::collect(&dt1_frames(&raw), base, 256).unwrap();
    let ours = ir200_core::device::collect(
        &dt1_frames(&Ir200::encode("pc-map", &v, 0).unwrap()),
        base,
        256,
    )
    .unwrap();
    assert_eq!(ours, pedal);
}
