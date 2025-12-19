use super::{InnerSamples, ReadingChannel, ReadingChannels, Sample};

// Introducing a custom iterator type is necessary for type erasure.
pub struct ChannelsSamplesZip {
    reading_channels: ReadingChannels,
    samples: InnerSamples,
    i: usize,
}

impl ChannelsSamplesZip {
    pub fn new(reading_channels: ReadingChannels, samples: InnerSamples) -> Self {
        Self {
            reading_channels,
            samples,
            i: 0,
        }
    }
}

impl Iterator for ChannelsSamplesZip {
    type Item = (ReadingChannel, Sample);

    fn next(&mut self) -> Option<Self::Item> {
        // This is functionally zipping samples with channels.
        // TODO: it might be possible to write this more efficiently.
        match (
            self.reading_channels.iter().nth(self.i),
            self.samples.iter().nth(self.i),
        ) {
            (Some(reading_channel), Some(sample)) => {
                self.i += 1;
                Some((reading_channel, sample))
            }
            _ => None,
        }
    }
}

impl ExactSizeIterator for ChannelsSamplesZip {
    fn len(&self) -> usize {
        self.reading_channels
            .iter()
            .len()
            .min(self.samples.iter().len())
    }
}

impl core::iter::FusedIterator for ChannelsSamplesZip {}
