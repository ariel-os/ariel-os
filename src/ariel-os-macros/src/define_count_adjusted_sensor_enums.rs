/// Generates sensor-related enums whose number of variants needs to be adjusted based on Cargo
/// features, to accommodate the sensor driver returning the largest number of samples.
///
/// One single type must be defined so that it can be used in the Future returned by sensor
/// drivers, which must be the same for every sensor driver so it can be part of the `Sensor`
/// trait.
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
    let samples_from_impls = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! {
            impl From<[Sample; #i]> for Samples {
                fn from(value: [Sample; #i]) -> Self {
                    Self { samples: InnerSamples::#variant(value) }
                }
            }
        }
    });
    let samples_first_sample = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! {
            InnerSamples::#variant(samples) => {
                if let Some(sample) = samples.first() {
                    *sample
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
        quote! { InnerSamples::#variant(ref samples) => samples.iter().copied() }
    });
    let reading_channels_iter = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { InnerReadingChannels::#variant(ref channels) => channels.iter().copied() }
    });

    let expanded = quote! {
        /// Samples returned by a sensor driver.
        ///
        /// This type implements [`Reading`] to iterate over the samples.
        ///
        /// # Note
        ///
        /// This type is automatically generated, the number of [`Sample`]s that can be stored is
        /// automatically adjusted.
        #[derive(Debug, Copy, Clone)]
        pub struct Samples {
            samples: InnerSamples,
        }

        #(#samples_from_impls)*

        impl Reading for Samples {
            fn sample(&self) -> Sample {
                match self.samples {
                    #(#samples_first_sample),*
                }
            }

            fn samples(&self) -> impl ExactSizeIterator<Item = Sample> + core::iter::FusedIterator {
                match self.samples {
                    #(#samples_iter),*
                }
            }
        }

        #[derive(Debug, Copy, Clone)]
        enum InnerSamples {
            #(#samples_variants),*
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
            /// [`Iterator::zip()`] can be useful to zip the returned iterator with the one
            /// obtained with [`Reading::samples()`].
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
    };

    TokenStream::from(expanded)
}

mod define_count_adjusted_enum {
    pub fn variant_name(index: usize) -> syn::Ident {
        quote::format_ident!("V{index}")
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
