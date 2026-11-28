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

    fn remaining(&self) -> usize {
        self.reading_channels
            .iter()
            .len()
            .min(self.samples.iter().len())
            .saturating_sub(self.i)
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

    fn size_hint(&self) -> (usize, Option<usize>) {
        let remaining = self.remaining();
        (remaining, Some(remaining))
    }
}

impl ExactSizeIterator for ChannelsSamplesZip {
    fn len(&self) -> usize {
        self.remaining()
    }
}

impl core::iter::FusedIterator for ChannelsSamplesZip {}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::{Label, MeasurementUnit, sensor::SampleMetadata};

    #[test]
    fn exact_size_iterator() {
        let channels = ReadingChannels::from([ReadingChannel::new(
            Label::Temperature,
            0,
            MeasurementUnit::Celsius,
        )]);
        let samples = InnerSamples::V1([Sample::new(42, SampleMetadata::UnknownAccuracy)]);

        let mut iter = ChannelsSamplesZip::new(channels, samples);
        assert_eq!(iter.len(), 1);
        assert_eq!(iter.size_hint(), (1, Some(1)));

        let skipped = ChannelsSamplesZip::new(channels, samples).skip(1);
        assert_eq!(skipped.len(), 0);

        assert!(iter.next().is_some());
        assert_eq!(iter.len(), 0);
        assert_eq!(iter.size_hint(), (0, Some(0)));
        assert!(iter.next().is_none());
    }
}
