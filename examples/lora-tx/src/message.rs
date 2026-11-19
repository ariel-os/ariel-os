//! Payload of the LoRa packets sent by this example.
//!
//! The layout is shared with the `lora-rx` example, multi-byte fields are
//! little-endian:
//!
//! | Offset | Size | Field                                              |
//! |--------|------|----------------------------------------------------|
//! | 0      | 2    | magic, `b"AR"`, to tell these packets from others  |
//! | 2      | 1    | version of the format, [`VERSION`]                 |
//! | 3      | 4    | node ID of the sender, see [`node_id()`]           |
//! | 7      | 4    | message index, incremented at each send            |
//! | 11     | 4    | uptime of the sender when sending, in milliseconds |
//! | 15     | 1    | TX power of the sender, in dBm                     |

/// Identifies the packets of these examples among other LoRa traffic.
const MAGIC: [u8; 2] = *b"AR";

/// Version of the payload format.
const VERSION: u8 = 1;

/// Length of the payload, in bytes.
pub const LEN: usize = 16;

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
    /// Encodes the message as a payload.
    pub fn to_bytes(&self) -> [u8; LEN] {
        let fields = MAGIC
            .into_iter()
            .chain([VERSION])
            .chain(self.node_id.to_le_bytes())
            .chain(self.index.to_le_bytes())
            .chain(self.uptime_ms.to_le_bytes())
            .chain(self.tx_power_dbm.to_le_bytes());

        let mut payload = [0; LEN];
        for (byte, field) in payload.iter_mut().zip(fields) {
            *byte = field;
        }
        payload
    }
}

/// Returns a 32-bit identifier of this device: the FNV-1a hash of its device
/// ID, or 0 when the device ID is not available.
pub fn node_id() -> u32 {
    let Ok(device_id) = ariel_os::identity::device_id_bytes() else {
        return 0;
    };
    device_id.as_ref().iter().fold(0x811c_9dc5, |hash, byte| {
        (hash ^ u32::from(*byte)).wrapping_mul(0x0100_0193)
    })
}
