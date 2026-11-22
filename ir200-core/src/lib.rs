//! Pure codec for the BOSS IR-200 Amp & IR Cabinet pedal's SysEx protocol.
//!
//! There is no published MIDI implementation for the IR-200; the parameter map
//! is IR Loader's own (`address_map.js`), turned into [`map`] by
//! `tools/gen/gen_map.py`. Value names come from the owner's manual and the
//! BOSS sound list.
//!
//! No I/O, no threads, no MIDI backend: compiles for `wasm32-unknown-unknown`.

#![forbid(unsafe_code)]
// tsify 0.5.8 deprecates `#[tsify(into_wasm_abi, from_wasm_abi)]`; the warning
// lands in every module holding such a derive (including `byte_enum!`/`block!`
// expansions). Migrating to `tsify::Ts<T>` would change the wasm ABI the web
// editor relies on, so keep the attributes and silence the deprecation.
#![cfg_attr(feature = "tsify", allow(deprecated))]

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

pub use catalog::{IR200_CATALOGS, Ir200Catalogs};
pub use codec::CodecError;
pub use device::Ir200;
pub use inbound::{InboundMessage, classify_inbound};
pub use live::{Change, describe};
pub use memory::Memory;
pub use pc_map::ProgramChangeMap;
pub use setup::Setup;
pub use sysex::{CMD_DT1, CMD_RQ1, Frame, IR200_MODEL_ID, SysExError, identify, identity_request};
pub use system::System;
