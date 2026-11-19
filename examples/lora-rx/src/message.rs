//! Payload of the LoRa packets received by this example.
//!
//! The layout is shared with the `lora-tx` example, multi-byte fields are
//! little-endian:
//!
//! | Offset | Size | Field                                              |
//! |--------|------|----------------------------------------------------|
//! | 0      | 2    | magic, `b"AR"`, to tell these packets from others  |
//! | 2      | 1    | version of the format, [`VERSION`]                 |
//! | 3      | 4    | node ID of the sender                              |
//! | 7      | 4    | message index, incremented at each send            |
//! | 11     | 4    | uptime of the sender when sending, in milliseconds |
//! | 15     | 1    | TX power of the sender, in dBm                     |

/// Identifies the packets of these examples among other LoRa traffic.
const MAGIC: [u8; 2] = *b"AR";

/// Version of the payload format.
const VERSION: u8 = 1;

/// Content of a packet.
pub struct Message {
    /// Node ID of the sender.
    pub node_id: u32,
    /// Index of the message, incremented at each send.
    pub index: u32,
    /// Uptime of the sender when sending, in milliseconds.
    pub uptime_ms: u32,
    /// TX power of the sender, in dBm.
    pub tx_power_dbm: i8,
}

impl Message {
    /// Decodes a payload, returns `None` if it is not a message of these
    /// examples (other LoRa traffic, unknown version or wrong length).
    pub fn from_bytes(payload: &[u8]) -> Option<Self> {
        let (magic, rest) = payload.split_first_chunk::<2>()?;
        let (&[version], rest) = rest.split_first_chunk::<1>()?;
        if *magic != MAGIC || version != VERSION {
            return None;
        }
        let (node_id, rest) = rest.split_first_chunk::<4>()?;
        let (index, rest) = rest.split_first_chunk::<4>()?;
        let (uptime_ms, rest) = rest.split_first_chunk::<4>()?;
        let (tx_power_dbm, rest) = rest.split_first_chunk::<1>()?;
        if !rest.is_empty() {
            return None;
        }

        Some(Self {
            node_id: u32::from_le_bytes(*node_id),
            index: u32::from_le_bytes(*index),
            uptime_ms: u32::from_le_bytes(*uptime_ms),
            tx_power_dbm: i8::from_le_bytes(*tx_power_dbm),
        })
    }
}
