//! YAML string codecs, for the wasm layer (the CLI does its own file I/O).

use crate::codec::CodecError;
use crate::memory::Memory;
use crate::pc_map::ProgramChangeMap;
use crate::system::System;

macro_rules! yaml_codec {
    ($to:ident, $from:ident, $ty:ty) => {
        pub fn $to(v: &$ty) -> Result<String, CodecError> {
            serde_yaml::to_string(v).map_err(|e| CodecError::Yaml(e.to_string()))
        }
        pub fn $from(s: &str) -> Result<$ty, CodecError> {
            serde_yaml::from_str(s).map_err(|e| CodecError::Yaml(e.to_string()))
        }
    };
}

yaml_codec!(memory_to_yaml, memory_from_yaml, Memory);
yaml_codec!(system_to_yaml, system_from_yaml, System);
yaml_codec!(pc_map_to_yaml, pc_map_from_yaml, ProgramChangeMap);

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn memory_yaml_round_trip() {
        let m = Memory::init_memory();
        let y = memory_to_yaml(&m).unwrap();
        assert!(y.contains("name: INIT MEMORY"));
        assert!(y.contains("gain_switch: middle"));
        assert_eq!(memory_from_yaml(&y).unwrap(), m);
        assert!(matches!(
            memory_from_yaml("nope: 1"),
            Err(CodecError::Yaml(_))
        ));
    }
}
