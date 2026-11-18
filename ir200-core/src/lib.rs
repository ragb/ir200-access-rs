//! Pure codec for the BOSS IR-200 Amp & IR Cabinet pedal's SysEx protocol.
//!
//! There is no published MIDI implementation for the IR-200; the parameter map
//! is IR Loader's own (`address_map.js`), turned into [`map`] by
//! `tools/gen/gen_map.py`. Value names come from the owner's manual and the
//! BOSS sound list.
//!
//! No I/O, no threads, no MIDI backend: compiles for `wasm32-unknown-unknown`.

#![forbid(unsafe_code)]

pub mod address;
pub mod catalog;
pub mod codec;
pub mod device;
pub mod inbound;
pub mod irconv;
pub mod live;
pub mod map;
pub mod memory;
pub mod pc_map;
pub mod setup;
pub mod sysex;
pub mod system;
pub mod types;
pub mod yaml;

pub use catalog::{Ir200Catalogs, IR200_CATALOGS};
pub use codec::CodecError;
pub use device::Ir200;
pub use inbound::{classify_inbound, InboundMessage};
pub use live::{describe, Change};
pub use memory::Memory;
pub use pc_map::ProgramChangeMap;
pub use setup::Setup;
pub use sysex::{identify, identity_request, Frame, SysExError, CMD_DT1, CMD_RQ1, IR200_MODEL_ID};
pub use system::System;
