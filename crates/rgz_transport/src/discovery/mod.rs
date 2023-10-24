mod discovery;
mod types;
mod store;

pub(crate) use discovery::Discovery;
pub(crate) use types::*;
pub(crate) use store::DiscoveryStore;

use anyhow::{bail, Result};
use std::env;

/// Longest string to receive.
const MAX_RCV_STR: usize = u16::MAX as usize;

/// Wire protocol version.
const WIRE_VERSION: u32 = 10;

/// Default activity interval value (ms.).
const DEF_ACTIVITY_INTERVAL: u64 = 100;

/// Default heartbeat interval value (ms.).
const DEF_HEARTBEAT_INTERVAL: u64 = 1000;

/// Default silence interval value (ms.).
const DEF_SILENCE_INTERVAL: u64 = 3000;

/// Timeout used for receiving messages (ms.).
const TIMEOUT: i32 = 250;

/// Get the discovery protocol version.
fn version() -> u32 {
    let gz_stats = env::var("GZ_TRANSPORT_TOPIC_STATISTICS")
        .ok()
        .filter(|s| !s.is_empty())
        .map(|s| s == "1")
        .unwrap_or(false);
    WIRE_VERSION + (gz_stats as u32 * 100)
}

fn discovery_msg_decode(rcv_str: &mut Vec<u8>, received: usize) -> Result<DiscoveryMsg> {
    use prost::Message;

    let mut buf = [0u8; 2];
    buf.copy_from_slice(&rcv_str[..2]);
    let len = u16::from_le_bytes(buf);
    if len + 2 == received as u16 {
        if let Ok(msg) = DiscoveryMsg::decode(&rcv_str[2..(len + 2) as usize]) {
            return Ok(msg);
        } else {
            bail!("Failed to decode Discovery message.");
        }
    }
    bail!("Failed to decode Discovery message.");
}

fn discovery_msg_encode(msg: &DiscoveryMsg) -> Result<(Vec<u8>, usize)> {
    use prost::Message;

    // let msg_size_full = std::mem::size_of_val(msg);
    let msg_size_full = msg.encoded_len();
    if msg_size_full + std::mem::size_of::<u16>() > MAX_RCV_STR {
        bail!("Discovery message too large to send. Discovery won't work. This shouldn't happen.");
    }

    let msg_size: u16 = msg_size_full.try_into().unwrap();
    let total_size = std::mem::size_of::<u16>() + msg_size as usize;
    let mut buffer = vec![0u8; total_size];
    buffer[0..2].copy_from_slice(&msg_size.to_le_bytes());

    let mut buf = Vec::new();
    buf.reserve(msg_size_full);
    if msg.encode(&mut buf).is_ok() {
        buffer[2..total_size].copy_from_slice(&buf);
        return Ok((buffer, total_size));
    }

    bail!("Failed to encode Discovery message.");
}
