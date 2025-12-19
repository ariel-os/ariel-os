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
    // Starting at 2 as the first one is not feature-gated and manually written.
    let samples_new_funcs = (2..=count).map(|i| {
        let variant = variant_name(i);
        let func_name = from_variant_func_name(i);
        let feature_name = feature_name(i);
        quote! {
            #[doc = concat!("Creates a new [`Samples`] containing ", #i, " samples.")]
            #[cfg(feature = #feature_name)]
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

    // Starting at 2 as the first one is not feature-gated and manually written.
    let reading_channels_from_impls = (2..=count).map(|i| {
        let variant = variant_name(i);
        let feature_name = feature_name(i);
        quote! {
            #[cfg(feature = #feature_name)]
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
        /// # For implementors
        ///
        /// Sensor driver crates must enable the appropriate `max-sample-min-count-*` Cargo feature
        /// on this crate.
        /// For instance, a 3-axis accelerometer driver crate must enable `max-sample-min-count-3`
        /// to be able to return 3 [`Sample`]s using [`Samples::from_3()`].
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
            /// Creates a new [`Samples`] containing 1 sample.
            pub fn from_1(sensor: &'static dyn Sensor, samples: [Sample; 1]) -> Self {
                Self {
                    samples: InnerSamples::V1(samples),
                    sensor,
                }
            }

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
        /// # For implementors
        ///
        /// [`ReadingChannels`] can be instantiated using [`From`] implementations.
        ///
        /// Sensor driver crates must enable the appropriate `max-sample-min-count-*` Cargo feature
        /// on this crate.
        /// For instance, a 3-axis accelerometer driver crate must enable `max-sample-min-count-3`
        /// to be able to expose 3 [`ReadingChannel`]s.
        #[derive(Debug, Copy, Clone)]
        pub struct ReadingChannels {
            channels: InnerReadingChannels,
        }

        impl From<[ReadingChannel; 1]> for ReadingChannels {
            fn from(value: [ReadingChannel; 1]) -> Self {
                Self { channels: InnerReadingChannels::V1(value) }
            }
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

    pub fn feature_name(index: usize) -> String {
        format!("max-sample-min-count-{index}")
    }

    pub fn from_variant_func_name(index: usize) -> syn::Ident {
        quote::format_ident!("from_{index}")
    }

    #[allow(unused_variables, reason = "overridden by feature selection")]
    pub fn get_allocation_size() -> usize {
        // The order of these feature-gated statements is important as these features are not meant to
        // be mutually exclusive.
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
        #[cfg(feature = "max-sample-min-count-10")]
        let count = 10;
        #[cfg(feature = "max-sample-min-count-11")]
        let count = 11;
        #[cfg(feature = "max-sample-min-count-12")]
        let count = 12;

        count
    }
}
