//! IR-200 CLI entry point.
//!
//! The whole command surface — ports / identity / dump / sync / show / lint /
//! diff / schema / catalog / resolve — is the generic engine in
//! `midi-access-cli`, dispatched through [`ir200_core::Ir200`]'s [`Device`] impl.
//!
//! [`Device`]: midi_access_core::Device

use std::process::ExitCode;

fn main() -> ExitCode {
    midi_access_cli::run::<ir200_core::Ir200>()
}
