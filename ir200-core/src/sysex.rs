//! Roland SysEx framing for the IR-200.
//!
//! ```text
//! F0 41 [dev] 00 00 00 00 06 [cmd] [a a a a] [data …] [chk] F7
//!       │     └ model ID (5 bytes)  └ 11 = RQ1, 12 = DT1
//!       └ device ID: 10 by default; 7F = broadcast
//! ```
//!
//! Checksum: `(128 - (sum of address + data) % 128) & 0x7F`, the Roland
//! standard, over address and data only.
//!
//! Source: IR Loader's `product_setting.js` (model ID `0000000006`, device ID
//! `10`, family code `0604`) and `midi_controller.js` (framing, checksum).

use serde::{Deserialize, Serialize};
use thiserror::Error;

pub const SYSEX_START: u8 = 0xF0;
pub const SYSEX_END: u8 = 0xF7;
pub const ROLAND_ID: u8 = 0x41;

/// 5-byte Roland model ID of the IR-200.
pub const IR200_MODEL_ID: [u8; 5] = [0x00, 0x00, 0x00, 0x00, 0x06];

/// Identity Reply family code (`06 04`) and family number (`00 00`).
pub const IR200_FAMILY: [u8; 4] = [0x06, 0x04, 0x00, 0x00];

/// Data Set 1: write.
pub const CMD_DT1: u8 = 0x12;
/// Data Request 1: read.
pub const CMD_RQ1: u8 = 0x11;

/// The device ID a stock IR-200 uses.
pub const DEVICE_ID_DEFAULT: u8 = 0x10;
/// Broadcast device ID.
pub const DEVICE_ID_BROADCAST: u8 = 0x7F;

/// F0 41 dev [5 model] cmd [4 addr] chk F7.
const MIN_FRAME_LEN: usize = 15;
const CMD_INDEX: usize = 8;
const ADDR_INDEX: usize = 9;

#[derive(Debug, Clone, Error, PartialEq, Eq)]
pub enum SysExError {
    #[error("frame too short ({0} bytes, need at least {MIN_FRAME_LEN})")]
    TooShort(usize),
    #[error("missing F0 start")]
    MissingStart,
    #[error("missing F7 end")]
    MissingEnd,
    #[error("not a Roland frame (manufacturer id {0:#04x})")]
    NotRoland(u8),
    #[error("invalid device id {0:#04x}")]
    InvalidDeviceId(u8),
    #[error("wrong model id (expected IR-200 {IR200_MODEL_ID:02x?})")]
    WrongModel,
    #[error("unknown command byte {0:#04x}")]
    UnknownCommand(u8),
    #[error("checksum mismatch: expected {expected:#04x}, got {actual:#04x}")]
    ChecksumMismatch { expected: u8, actual: u8 },
    #[error("data byte out of range (>= 0x80) at index {0}")]
    DataByteOutOfRange(usize),
}

/// Roland checksum over address + data.
pub fn checksum(addr_and_data: &[u8]) -> u8 {
    midi_access_core::codec::checksum(addr_and_data)
}

/// A decoded Roland frame to or from the IR-200. For RQ1, `data` is the 4-byte
/// size field.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct Frame {
    pub device_id: u8,
    pub command: u8,
    pub address: [u8; 4],
    pub data: Vec<u8>,
}

impl Frame {
    /// A DT1 frame.
    ///
    /// ```
    /// use ir200_core::Frame;
    /// // Editor communication mode on: DT1 7F 00 00 01 = 01.
    /// let f = Frame::data_set(0x10, [0x7F, 0x00, 0x00, 0x01], vec![0x01]).encode();
    /// assert_eq!(
    ///     f,
    ///     vec![0xF0, 0x41, 0x10, 0x00, 0x00, 0x00, 0x00, 0x06, 0x12,
    ///          0x7F, 0x00, 0x00, 0x01, 0x01, 0x7F, 0xF7]
    /// );
    /// ```
    pub fn data_set(device_id: u8, address: [u8; 4], data: Vec<u8>) -> Self {
        Self {
            device_id,
            command: CMD_DT1,
            address,
            data,
        }
    }

    /// An RQ1 frame for `size` (4-byte, 7-bit) bytes at `address`.
    pub fn data_request(device_id: u8, address: [u8; 4], size: [u8; 4]) -> Self {
        Self {
            device_id,
            command: CMD_RQ1,
            address,
            data: size.to_vec(),
        }
    }

    pub fn encode(&self) -> Vec<u8> {
        let mut buf = Vec::with_capacity(MIN_FRAME_LEN + self.data.len());
        buf.push(SYSEX_START);
        buf.push(ROLAND_ID);
        buf.push(self.device_id);
        buf.extend_from_slice(&IR200_MODEL_ID);
        buf.push(self.command);
        buf.extend_from_slice(&self.address);
        buf.extend_from_slice(&self.data);
        let mut ck = self.address.to_vec();
        ck.extend_from_slice(&self.data);
        buf.push(checksum(&ck));
        buf.push(SYSEX_END);
        buf
    }

    pub fn decode(bytes: &[u8]) -> Result<Self, SysExError> {
        if bytes.len() < MIN_FRAME_LEN {
            return Err(SysExError::TooShort(bytes.len()));
        }
        if bytes[0] != SYSEX_START {
            return Err(SysExError::MissingStart);
        }
        if *bytes.last().unwrap() != SYSEX_END {
            return Err(SysExError::MissingEnd);
        }
        if bytes[1] != ROLAND_ID {
            return Err(SysExError::NotRoland(bytes[1]));
        }
        let device_id = bytes[2];
        if device_id > 0x1F && device_id != DEVICE_ID_BROADCAST {
            return Err(SysExError::InvalidDeviceId(device_id));
        }
        if bytes[3..8] != IR200_MODEL_ID {
            return Err(SysExError::WrongModel);
        }
        let command = bytes[CMD_INDEX];
        if command != CMD_DT1 && command != CMD_RQ1 {
            return Err(SysExError::UnknownCommand(command));
        }
        let address: [u8; 4] = bytes[ADDR_INDEX..ADDR_INDEX + 4].try_into().unwrap();
        let end = bytes.len() - 2;
        let data = bytes[ADDR_INDEX + 4..end].to_vec();
        if let Some(i) = data.iter().position(|&b| b >= 0x80) {
            return Err(SysExError::DataByteOutOfRange(i));
        }
        let mut ck = address.to_vec();
        ck.extend_from_slice(&data);
        let actual = checksum(&ck);
        if bytes[end] != actual {
            return Err(SysExError::ChecksumMismatch {
                expected: bytes[end],
                actual,
            });
        }
        Ok(Self {
            device_id,
            command,
            address,
            data,
        })
    }
}

/// Universal Identity Request `F0 7E <dev> 06 01 F7`.
pub fn identity_request(device_id: u8) -> Vec<u8> {
    vec![0xF0, 0x7E, device_id, 0x06, 0x01, 0xF7]
}

/// `Some(true)` for an IR-200 Identity Reply, `Some(false)` for another
/// device's, `None` if `bytes` is not an Identity Reply.
///
/// The reply is `F0 7E <dev> 06 02 41 06 04 00 00 <rev ×4> F7`.
pub fn identify(bytes: &[u8]) -> Option<bool> {
    let is_reply = bytes.len() >= 6
        && bytes[0] == 0xF0
        && bytes[1] == 0x7E
        && bytes[3] == 0x06
        && bytes[4] == 0x02;
    if !is_reply {
        return None;
    }
    Some(bytes.len() >= 10 && bytes[5] == ROLAND_ID && bytes[6..10] == IR200_FAMILY)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rq1_layout() {
        let f = Frame::data_request(0x10, [0x7F, 0, 0, 0], [0, 0, 0, 1]).encode();
        assert_eq!(f.len(), 19);
        assert_eq!(&f[..9], &[0xF0, 0x41, 0x10, 0, 0, 0, 0, 0x06, 0x11]);
        // sum 0x7F + 1 = 0x80 -> checksum 0.
        assert_eq!(f[17], 0x00);
        assert_eq!(Frame::decode(&f).unwrap().data, vec![0, 0, 0, 1]);
    }

    #[test]
    fn dt1_round_trip_and_rejections() {
        let f = Frame::data_set(0x7F, [0x20, 0x00, 0x10, 0x00], vec![1, 72, 50]);
        let mut b = f.encode();
        assert_eq!(Frame::decode(&b).unwrap(), f);
        let n = b.len();
        b[n - 2] ^= 1;
        assert!(matches!(
            Frame::decode(&b),
            Err(SysExError::ChecksumMismatch { .. })
        ));
        let mut w = f.encode();
        w[7] = 0x43;
        assert_eq!(Frame::decode(&w), Err(SysExError::WrongModel));
    }

    #[test]
    fn identity() {
        assert_eq!(
            identity_request(0x7F),
            vec![0xF0, 0x7E, 0x7F, 0x06, 0x01, 0xF7]
        );
        let reply = [
            0xF0, 0x7E, 0x10, 0x06, 0x02, 0x41, 0x06, 0x04, 0x00, 0x00, 0x00, 0x01, 0x01, 0x00,
            0xF7,
        ];
        assert_eq!(identify(&reply), Some(true));
        let md500 = [
            0xF0, 0x7E, 0x10, 0x06, 0x02, 0x41, 0x43, 0x03, 0x00, 0x00, 0xF7,
        ];
        assert_eq!(identify(&md500), Some(false));
        assert_eq!(identify(&[0xC0, 0x01]), None);
    }
}
