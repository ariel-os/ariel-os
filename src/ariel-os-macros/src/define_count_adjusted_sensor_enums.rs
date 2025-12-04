/// Generates sensor-related enums whose number of variants needs to be adjusted based on Cargo
/// features, to accommodate the sensor driver returning the largest number of samples.
///
/// One single type must be defined so that it can be used in the Future returned by sensor
/// drivers, which must be the same for every sensor driver so it can be part of the `Sensor`
/// trait.
#[expect(clippy::too_many_lines)]
#[proc_macro]
pub fn define_count_adjusted_sensor_enums(_item: TokenStream) -> TokenStream {
    use quote::quote;

    #[allow(clippy::wildcard_imports)]
    use define_count_adjusted_enum::*;

    let count = get_allocation_size();

    let samples_variants = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { #variant([Sample; #i]) }
    });
    let samples_new_funcs = (1..=count).map(|i| {
        let variant = variant_name(i);
        let func_name = from_variant_func_name(i);
        quote! {
            #[doc = concat!("Creates a new [`Samples`] containing ", #i, " sample(s).")]
            pub fn #func_name(sensor: &'static dyn Sensor, samples: [Sample; #i]) -> Self {
                Self {
                    samples: InnerSamples::#variant(samples),
                    sensor,
                }
            }
        }
    });
    let samples_first_sample = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! {
            InnerSamples::#variant(samples) => {
                if let Some(sample) = samples.first() {
                    let reading_channel = self.sensor.reading_channels().first();
                    (reading_channel, *sample)
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
        }
    });

    let reading_channels_from_impls = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! {
            impl From<[ReadingChannel; #i]> for ReadingChannels {
                fn from(value: [ReadingChannel; #i]) -> Self {
                    Self { channels: InnerReadingChannels::#variant(value) }
                }
            }
        }
    });
    let reading_channels_variants = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { #variant([ReadingChannel; #i]) }
    });

    let samples_iter = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { InnerSamples::#variant(samples) => samples.iter().copied() }
    });
    let reading_channels_iter = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { InnerReadingChannels::#variant(ref channels) => channels.iter().copied() }
    });

    let expanded = quote! {
        /// Provides access to the sensor driver instance.
        /// For driver implementors only.
        pub trait SensorAccess: private::Sealed {
            /// Returns the sensor driver instance that produced these samples.
            /// For driver implementors only.
            fn sensor(&self) -> &'static dyn Sensor;
        }

        /// Avoid external implementations of [`SensorAccess`].
        mod private {
            use super::Samples;
            pub trait Sealed {}

            impl Sealed for Samples {}
        }

        /// Samples returned by a sensor driver.
        ///
        /// This type implements [`Reading`] to iterate over the samples.
        ///
        /// # Note
        ///
        /// This type is automatically generated, the number of [`Sample`]s that can be stored is
        /// automatically adjusted.
        #[derive(Copy, Clone)]
        pub struct Samples {
            samples: InnerSamples,
            sensor: &'static dyn Sensor,
        }

        impl core::fmt::Debug for Samples {
            fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
                f.debug_struct("Samples")
                 .field("samples", &self.samples)
                 .field("sensor", &"&dyn Sensor")
                 .finish()
            }
        }

        impl Samples {
            #(#samples_new_funcs)*
        }

        impl SensorAccess for Samples {
            fn sensor(&self) -> &'static dyn Sensor {
                self.sensor
            }
        }

        impl Reading for Samples {
            fn sample(&self) -> (ReadingChannel, Sample) {
                match self.samples {
                    #(#samples_first_sample),*
                }
            }

            fn samples(&self) -> impl ExactSizeIterator<Item = (ReadingChannel, Sample)> + core::iter::FusedIterator {
                let reading_channels = self.sensor.reading_channels();
                ChannelsSamplesZip::new(reading_channels, self.samples)
            }
        }

        #[derive(Debug, Copy, Clone)]
        enum InnerSamples {
            #(#samples_variants),*
        }

        impl InnerSamples {
            fn iter(&self) -> impl ExactSizeIterator<Item = Sample> + core::iter::FusedIterator + '_ {
                match self {
                    #(#samples_iter),*
                }
            }
        }

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

        #(#reading_channels_from_impls)*

        impl ReadingChannels {
            /// Returns an iterator over the underlying [`ReadingChannel`] items.
            ///
            /// For a given sensor driver, the number and order of items match the one of
            /// [`Samples`].
            pub fn iter(&self) -> impl ExactSizeIterator<Item = ReadingChannel> + core::iter::FusedIterator + '_ {
                match self.channels {
                    #(#reading_channels_iter),*
                }
            }

            /// Returns the first [`ReadingChannel`].
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
            #(#reading_channels_variants),*
        }

        // Introducing a custom iterator type is necessary for type erasure.
        struct ChannelsSamplesZip {
            reading_channels: ReadingChannels,
            samples: InnerSamples,
            i: usize,
        }

        impl ChannelsSamplesZip {
            fn new(reading_channels: ReadingChannels, samples: InnerSamples) -> Self {
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
                match (self.reading_channels.iter().nth(self.i), self.samples.iter().nth(self.i)) {
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
                self.reading_channels.iter().len().min(self.samples.iter().len())
            }
        }

        impl core::iter::FusedIterator for ChannelsSamplesZip {}
    };

    TokenStream::from(expanded)
}

mod define_count_adjusted_enum {
    pub fn variant_name(index: usize) -> syn::Ident {
        quote::format_ident!("V{index}")
    }

    pub fn from_variant_func_name(index: usize) -> syn::Ident {
        quote::format_ident!("from_{index}")
    }

    pub fn get_allocation_size() -> usize {
        // The order of these feature-gated statements is important as these features are not meant to
        // be mutually exclusive.
        #[allow(unused_variables, reason = "overridden by feature selection")]
        let count = 1;
        #[cfg(feature = "max-sample-min-count-2")]
        let count = 2;
        #[cfg(feature = "max-sample-min-count-3")]
        let count = 3;
        #[cfg(feature = "max-sample-min-count-4")]
        let count = 4;
        #[cfg(feature = "max-sample-min-count-5")]
        let count = 5;
        #[cfg(feature = "max-sample-min-count-6")]
        let count = 6;
        #[cfg(feature = "max-sample-min-count-7")]
        let count = 7;
        #[cfg(feature = "max-sample-min-count-8")]
        let count = 8;
        #[cfg(feature = "max-sample-min-count-9")]
        let count = 9;
        #[cfg(feature = "max-sample-min-count-12")]
        let count = 12;

        count
    }
}
