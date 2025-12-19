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

#[cfg(feature = "max-sample-min-count-2")]
impl From<[ReadingChannel; 2]> for ReadingChannels {
    fn from(value: [ReadingChannel; 2]) -> Self {
        Self {
            channels: InnerReadingChannels::V2(value),
        }
    }
}

#[cfg(feature = "max-sample-min-count-3")]
impl From<[ReadingChannel; 3]> for ReadingChannels {
    fn from(value: [ReadingChannel; 3]) -> Self {
        Self {
            channels: InnerReadingChannels::V3(value),
        }
    }
}

#[cfg(feature = "max-sample-min-count-4")]
impl From<[ReadingChannel; 4]> for ReadingChannels {
    fn from(value: [ReadingChannel; 4]) -> Self {
        Self {
            channels: InnerReadingChannels::V4(value),
        }
    }
}

#[cfg(feature = "max-sample-min-count-5")]
impl From<[ReadingChannel; 5]> for ReadingChannels {
    fn from(value: [ReadingChannel; 5]) -> Self {
        Self {
            channels: InnerReadingChannels::V5(value),
        }
    }
}

#[cfg(feature = "max-sample-min-count-6")]
impl From<[ReadingChannel; 6]> for ReadingChannels {
    fn from(value: [ReadingChannel; 6]) -> Self {
        Self {
            channels: InnerReadingChannels::V6(value),
        }
    }
}

#[cfg(feature = "max-sample-min-count-7")]
impl From<[ReadingChannel; 7]> for ReadingChannels {
    fn from(value: [ReadingChannel; 7]) -> Self {
        Self {
            channels: InnerReadingChannels::V7(value),
        }
    }
}

#[cfg(feature = "max-sample-min-count-8")]
impl From<[ReadingChannel; 8]> for ReadingChannels {
    fn from(value: [ReadingChannel; 8]) -> Self {
        Self {
            channels: InnerReadingChannels::V8(value),
        }
    }
}

#[cfg(feature = "max-sample-min-count-9")]
impl From<[ReadingChannel; 9]> for ReadingChannels {
    fn from(value: [ReadingChannel; 9]) -> Self {
        Self {
            channels: InnerReadingChannels::V9(value),
        }
    }
}

#[cfg(feature = "max-sample-min-count-10")]
impl From<[ReadingChannel; 10]> for ReadingChannels {
    fn from(value: [ReadingChannel; 10]) -> Self {
        Self {
            channels: InnerReadingChannels::V10(value),
        }
    }
}

#[cfg(feature = "max-sample-min-count-11")]
impl From<[ReadingChannel; 11]> for ReadingChannels {
    fn from(value: [ReadingChannel; 11]) -> Self {
        Self {
            channels: InnerReadingChannels::V11(value),
        }
    }
}

#[cfg(feature = "max-sample-min-count-12")]
impl From<[ReadingChannel; 12]> for ReadingChannels {
    fn from(value: [ReadingChannel; 12]) -> Self {
        Self {
            channels: InnerReadingChannels::V12(value),
        }
    }
}
