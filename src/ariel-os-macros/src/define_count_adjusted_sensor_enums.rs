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

    let samples_variants = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { #variant([Sample; #i]) }
    });
    let samples_first_sample = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! {
            Self::#variant(samples) => {
                if let Some(sample) = samples.first() {
                    *sample
                } else {
                    // NOTE(no-panic): there is always at least one sample
                    unreachable!();
                }
            }
        }
    });

    let reading_channels_variants = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { #variant([ReadingChannel; #i]) }
    });

    let samples_iter = (1..=count)
        .map(|i| {
            let variant = variant_name(i);
            quote! { Self::#variant(samples) => samples.iter().copied() }
        })
        .collect::<Vec<_>>();

    let expanded = quote! {
        /// Samples returned by a sensor driver.
        ///
        /// This type implements [`Reading`] to iterate over the samples.
        ///
        /// # For implementors
        ///
        /// This enum is automatically generated and its number of variants is adjusted based
        /// on the set of `max-sample-min-count-*` Cargo features enabled in the build.
        /// When writing a sensor driver, its crate must enable the `max-sample-min-count-$c`
        /// feature, where `$c` is the number of channels the sensor driver returns.
        /// This makes sure the [`Samples`] enum contains `$v` different variants, with `$v` being
        /// higher than or equal to the largest `$c` enabled by the different sensor drivers in the
        /// build, to accommodate the sensor driver having the highest number of channels.
        /// Each variant is named `V$i` and has an array of `$i` [`Sample`]s as associated data.
        ///
        /// ## Example
        ///
        /// ```
        /// # use ariel_os_sensors::sensor::{Accuracy, Sample, Samples};
        /// # let temperature = 0;
        /// # let accuracy = Accuracy::Unknown;
        /// let sample = Sample::new(temperature, accuracy);
        /// # let _ =
        /// Samples::V1([sample])
        /// # ;
        /// ```
        #[derive(Debug, Copy, Clone)]
        pub enum Samples {
            #(
                #[doc(hidden)]
                #samples_variants
            ),*
        }

        impl Reading for Samples {
            fn sample(&self) -> Sample {
                match self {
                    #(#samples_first_sample),*
                }
            }

            fn samples(&self) -> impl ExactSizeIterator<Item = Sample> + core::iter::FusedIterator {
                match self {
                    #(#samples_iter),*
                }
            }
        }

        /// Metadata required to interpret samples returned by [`Sensor::wait_for_reading()`].
        ///
        /// # For implementors
        ///
        /// This enum is automatically generated and its number of variants is adjusted based
        /// on the set of `max-sample-min-count-*` Cargo features enabled in the build.
        /// When writing a sensor driver, its crate must enable the `max-sample-min-count-$c`
        /// feature, where `$c` is the number of channels the sensor driver returns.
        /// This makes sure the [`ReadingChannels`] enum contains `$v` different variants, with
        /// `$v` being higher than or equal to the largest `$c` enabled by the different sensor
        /// drivers in the build, to accommodate the sensor driver having the highest number of
        /// channels.
        /// Each variant is named `V$i` and has an array of `$i` [`ReadingChannel`]s as associated
        /// data.
        ///
        /// ## Example
        ///
        /// ```
        /// # use ariel_os_sensors::{Label, MeasurementUnit, sensor::{ReadingChannel, ReadingChannels}};
        /// let reading_channel = ReadingChannel::new(Label::Main, -1, MeasurementUnit::Celsius);
        /// # let _ =
        /// ReadingChannels::V1([reading_channel])
        /// # ;
        /// ```
        #[derive(Debug, Copy, Clone)]
        pub enum ReadingChannels {
            #(
                #[doc(hidden)]
                #reading_channels_variants
            ),*,
        }

        impl ReadingChannels {
            /// Returns an iterator over the underlying [`ReadingChannel`] items.
            ///
            /// For a given sensor driver, the number and order of items match the one of
            /// [`Samples`].
            /// [`Iterator::zip()`] can be useful to zip the returned iterator with the one
            /// obtained with [`Reading::samples()`].
            pub fn iter(&self) -> impl ExactSizeIterator<Item = ReadingChannel> + core::iter::FusedIterator + '_ {
                match self {
                    #(#samples_iter),*,
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
    };

    TokenStream::from(expanded)
}

mod define_count_adjusted_enum {
    pub fn variant_name(index: usize) -> syn::Ident {
        quote::format_ident!("V{index}")
    }
}
