use super::{InnerReadingChannels, ReadingChannel};

/// Metadata required to interpret samples returned by [`Sensor::wait_for_reading()`].
///
/// # Note
///
/// This type is automatically generated, the number of [`ReadingChannel`]s that can be
/// stored is automatically adjusted.
#[derive(Debug, Copy, Clone)]
pub struct ReadingChannels {
    pub(super) channels: InnerReadingChannels,
}

impl From<[ReadingChannel; 1]> for ReadingChannels {
    fn from(value: [ReadingChannel; 1]) -> Self {
        Self {
            channels: InnerReadingChannels::V1(value),
        }
    }
}
