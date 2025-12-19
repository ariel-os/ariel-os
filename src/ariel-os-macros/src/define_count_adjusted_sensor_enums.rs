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

    let reading_channels_variants = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { #variant([ReadingChannel; #i]) }
    });

    let reading_channels_iter = (1..=count).map(|i| {
        let variant = variant_name(i);
        quote! { InnerReadingChannels::#variant(ref channels) => channels.iter().copied() }
    });

    let expanded = quote! {
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
    };

    TokenStream::from(expanded)
}

mod define_count_adjusted_enum {
    pub fn variant_name(index: usize) -> syn::Ident {
        quote::format_ident!("V{index}")
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
