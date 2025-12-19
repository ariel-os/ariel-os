use super::ReadingChannel;

/// Metadata required to interpret samples returned by [`Sensor::wait_for_reading()`].
///
/// # Note
///
/// This type is automatically generated, the number of [`ReadingChannel`]s that can be
/// stored is automatically adjusted.
#[derive(Debug, Copy, Clone)]
pub struct ReadingChannels {
    channels: InnerReadingChannels,
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

impl ReadingChannels {
    /// Returns an iterator over the underlying [`ReadingChannel`] items.
    ///
    /// For a given sensor driver, the number and order of items match the one of
    /// [`Samples`].
    #[must_use]
    pub fn iter(
        &self,
    ) -> impl ExactSizeIterator<Item = ReadingChannel> + core::iter::FusedIterator + '_ {
        match self.channels {
            InnerReadingChannels::V1(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-2")]
            InnerReadingChannels::V2(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-3")]
            InnerReadingChannels::V3(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-4")]
            InnerReadingChannels::V4(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-5")]
            InnerReadingChannels::V5(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-6")]
            InnerReadingChannels::V6(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-7")]
            InnerReadingChannels::V7(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-8")]
            InnerReadingChannels::V8(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-9")]
            InnerReadingChannels::V9(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-10")]
            InnerReadingChannels::V10(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-11")]
            InnerReadingChannels::V11(ref channels) => channels.iter().copied(),
            #[cfg(feature = "max-sample-min-count-12")]
            InnerReadingChannels::V12(ref channels) => channels.iter().copied(),
        }
    }

    /// Returns the first [`ReadingChannel`].
    #[must_use]
    pub fn first(&self) -> ReadingChannel {
        if let Some(sample) = self.iter().next() {
            sample
        } else {
            // NOTE(no-panic): there is always at least one sample.
            unreachable!();
        }
    }
}

#[derive(Debug, Copy, Clone)]
enum InnerReadingChannels {
    V1([ReadingChannel; 1]),
    #[cfg(feature = "max-sample-min-count-2")]
    V2([ReadingChannel; 2]),
    #[cfg(feature = "max-sample-min-count-3")]
    V3([ReadingChannel; 3]),
    #[cfg(feature = "max-sample-min-count-4")]
    V4([ReadingChannel; 4]),
    #[cfg(feature = "max-sample-min-count-5")]
    V5([ReadingChannel; 5]),
    #[cfg(feature = "max-sample-min-count-6")]
    V6([ReadingChannel; 6]),
    #[cfg(feature = "max-sample-min-count-7")]
    V7([ReadingChannel; 7]),
    #[cfg(feature = "max-sample-min-count-8")]
    V8([ReadingChannel; 8]),
    #[cfg(feature = "max-sample-min-count-9")]
    V9([ReadingChannel; 9]),
    #[cfg(feature = "max-sample-min-count-10")]
    V10([ReadingChannel; 10]),
    #[cfg(feature = "max-sample-min-count-11")]
    V11([ReadingChannel; 11]),
    #[cfg(feature = "max-sample-min-count-12")]
    V12([ReadingChannel; 12]),
}
