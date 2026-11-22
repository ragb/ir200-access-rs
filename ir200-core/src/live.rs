//! Describe inbound parameter changes.
//!
//! In editor communication mode the pedal pushes a DT1 for every panel action
//! (verified, see `docs/sysex-notes.md`). [`describe`] turns one DT1 into the
//! parameters it touches, each with a document path, a label and a spoken
//! value, so a host can say "Gain 61" as the knob turns.

use midi_access_core::Catalogs;
use midi_access_core::meta::Kind;
use serde::Serialize;

use crate::address::{AddressSpace, TEMPORARY_BASE, to_u28};
use crate::codec::{Enc, P};
use crate::memory::Memory;
use crate::setup::Setup;
use crate::system::System;
use crate::{catalog, map};

/// One parameter a DT1 changed.
#[cfg_attr(feature = "tsify", derive(tsify::Tsify))]
#[cfg_attr(feature = "tsify", tsify(into_wasm_abi))]
#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct Change {
    /// IR Loader's parameter id, e.g. `PATCH_AMP_GAIN`.
    pub id: String,
    /// `setup`, `system`, `memory` or `pc-map`.
    pub area: String,
    /// Stored memory 1–128 when the DT1 addressed a stored memory; `None` for
    /// the temporary memory and other areas.
    pub memory: Option<u8>,
    /// Serde path inside the area's document, e.g. `amp.gain`, `eq_a.on`.
    pub path: String,
    /// Spoken label, e.g. `Gain`, `EQ A Low gain`.
    pub label: String,
    /// Display value (`None` for text).
    pub value: Option<i64>,
    /// Spoken value: a catalog name, `on`/`off`, an option, or the number
    /// with its unit. For text, the text.
    pub display: String,
}

/// Every parameter [`describe`] knows.
fn all() -> impl Iterator<Item = &'static P> {
    map::ALL
        .iter()
        .chain(std::iter::once(&map::patch_com::NAME))
}

/// Qualifier and catalog prefix for a repeated sub-structure.
fn shared(first: &str) -> Option<(&'static str, &'static str)> {
    Some(match first {
        "eq_a" => ("EQ A", "eq"),
        "eq_b" => ("EQ B", "eq"),
        "cabinet_a" => ("Cabinet A", "cabinet"),
        "cabinet_b" => ("Cabinet B", "cabinet"),
        "fsw_down" => ("Down switch", "switch"),
        "fsw_up" => ("Up switch", "switch"),
        "ctl1" => ("CTL 1", "switch"),
        "ctl2" => ("CTL 2", "switch"),
        "exp" => ("Expression", "exp"),
        _ => return None,
    })
}

/// Label and spoken value for `path` in `area`, from the catalog metadata.
fn label_and_value(area: &str, path: &str, v: i64, fallback: String) -> (String, String) {
    let (first, rest) = path.split_once('.').unwrap_or((path, ""));
    let (qualifier, meta_path) = match shared(first) {
        Some((q, prefix)) if !rest.is_empty() => (Some(q), format!("{prefix}.{rest}")),
        _ => (None, format!("{}.{path}", area.replace('-', "_"))),
    };
    let meta = catalog::params().get(&meta_path);
    let label = match (qualifier, meta) {
        (Some(q), Some(m)) => format!("{q} {}", m.label),
        (None, Some(m)) => m.label.to_string(),
        (_, None) => path.replace(['_', '.'], " "),
    };
    let value = match meta {
        Some(m) => match (m.catalog, m.kind) {
            (Some(c), _) => catalog::IR200_CATALOGS.label(c, v).unwrap_or(fallback),
            (None, Some(Kind::Range { unit: Some(u), .. })) => format!("{v} {u}"),
            _ => fallback,
        },
        None => fallback,
    };
    (label, value)
}

/// The parameters a DT1 of `data` at `address` touches. Writes to a stored
/// memory are mapped onto the memory layout (with [`Change::memory`] set).
pub fn describe(address: [u8; 4], data: &[u8]) -> Vec<Change> {
    let (lookup, memory) = match AddressSpace::classify(address) {
        AddressSpace::Memory(n) => {
            let base = crate::address::memory_base(n).unwrap();
            let rel = to_u28(address) - to_u28(base);
            (to_u28(TEMPORARY_BASE) + rel, Some(n))
        }
        _ => (to_u28(address), None),
    };
    let end = lookup + data.len() as u32;
    let mut out = Vec::new();
    for p in all() {
        let a = to_u28(p.address());
        let w = p.enc.width() as u32;
        if a < lookup || a + w > end {
            continue;
        }
        let slice = &data[(a - lookup) as usize..(a - lookup + w) as usize];
        let area = match p.block.addr >> 24 {
            0x00 => "setup",
            0x10 => "system",
            0x20 => "memory",
            _ => "pc-map",
        };
        if let Enc::Text(_) = p.enc {
            let text = String::from_utf8_lossy(slice).trim_end().to_string();
            out.push(Change {
                id: p.id.into(),
                area: area.into(),
                memory,
                path: "name".into(),
                label: "Name".into(),
                value: None,
                display: text,
            });
            continue;
        }
        let Some(v) = p.display_from_data(slice) else {
            continue;
        };
        let described = match area {
            "setup" => Setup::describe_param(p, v).map(|(f, s)| (f.to_string(), s)),
            "system" => System::describe_param(p, v),
            "memory" => Memory::describe_param(p, v),
            _ => Some((format!("memory.{}", p.off / 2), v.to_string())),
        };
        let (path, shown) = described.unwrap_or_else(|| (p.id.to_lowercase(), v.to_string()));
        let (label, display) = match area {
            "setup" => ("Memory".to_string(), (v + 1).to_string()),
            "pc-map" => (format!("Program change {}", p.off / 2 + 1), shown),
            _ => label_and_value(area, &path, v, shown),
        };
        out.push(Change {
            id: p.id.into(),
            area: area.into(),
            memory,
            path,
            label,
            value: Some(v),
            display,
        });
    }
    out
}

#[cfg(test)]
mod tests {
    use super::*;

    fn one(address: [u8; 4], data: &[u8]) -> Change {
        let c = describe(address, data);
        assert_eq!(c.len(), 1, "{c:?}");
        c.into_iter().next().unwrap()
    }

    #[test]
    fn knobs_as_the_pedal_sends_them() {
        let c = one([0x20, 0x00, 0x10, 0x01], &[0x3D]);
        assert_eq!(
            (c.path.as_str(), c.label.as_str(), c.display.as_str()),
            ("amp.gain", "Gain", "61")
        );
        assert_eq!(c.memory, None);

        let c = one([0x20, 0x00, 0x10, 0x00], &[0x08]);
        assert_eq!(
            (c.label.as_str(), c.display.as_str()),
            ("Amp type", "BGR UB METAL")
        );

        let c = one([0x20, 0x00, 0x50, 0x00], &[0, 0, 7, 8]);
        assert_eq!(c.label, "Cabinet A Cab type");
        assert_eq!(c.display, "B215 WIDE CND87-L");

        let c = one([0, 0, 0, 0], &[0x03, 0x0D]);
        assert_eq!((c.label.as_str(), c.display.as_str()), ("Memory", "62"));
    }

    #[test]
    fn enums_switches_units_and_system() {
        let c = one([0x20, 0x00, 0x10, 0x06], &[2]);
        assert_eq!(
            (c.label.as_str(), c.display.as_str()),
            ("Gain switch", "high")
        );
        let c = one([0x20, 0x00, 0x10, 0x07], &[1]);
        assert_eq!(c.display, "on");
        let c = one([0x10, 0x00, 0x30, 0x02], &[0]);
        assert_eq!(
            (c.label.as_str(), c.display.as_str()),
            ("EQ A Low gain", "-20 dB")
        );
        let c = one([0x10, 0x01, 0x20, 0x02], &[0]);
        assert_eq!(
            (c.label.as_str(), c.display.as_str()),
            ("Receive channel", "1")
        );
        let c = one([0x20, 0x00, 0x70, 0x02], &[26]);
        assert_eq!(c.display, "2.6 s");
    }

    #[test]
    fn whole_block_and_stored_memory() {
        let c = describe([0x21, 0x00, 0x10, 0x00], &[1, 50, 50, 50, 50, 50, 1, 0, 50]);
        assert_eq!(c.len(), 9);
        assert!(c.iter().all(|x| x.memory == Some(8)));
        let name = one([0x20, 0, 0, 0], b"CLEAN           ");
        assert_eq!(name.display, "CLEAN");
    }
}
