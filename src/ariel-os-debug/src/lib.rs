//! Provides debug interface facilities.

#![cfg_attr(not(any(test, feature = "std")), no_std)]
#![cfg_attr(test, no_main)]
#![deny(missing_docs)]

#[featurecomb::comb]
mod _featurecomb {}

mod exit;

pub use exit::*;

#[cfg(feature = "defmt-rtt")]
mod backend {
    use defmt_rtt as _;

    #[doc(hidden)]
    pub fn init() {}
}

#[cfg(feature = "rtt-target")]
mod backend {
    #[cfg(feature = "defmt")]
    pub use defmt::println as debug_output_println;

    #[cfg(not(feature = "defmt"))]
    pub use rtt_target::rprintln as debug_output_println;

    #[doc(hidden)]
    pub fn init() {
        #[cfg(not(feature = "defmt"))]
        {
            use rtt_target::ChannelMode::NoBlockTrim;

            rtt_target::rtt_init_print!(NoBlockTrim);
        }

        #[cfg(feature = "log")]
        crate::logger::init();

        #[cfg(feature = "defmt")]
        {
            use rtt_target::ChannelMode::NoBlockSkip;
            const DEFMT_BUFFER_SIZE: usize = 1024;
            let channels = rtt_target::rtt_init! {
                up: {
                    0: {
                        size: DEFMT_BUFFER_SIZE,
                        mode: NoBlockSkip,
                        // probe-run autodetects whether defmt is in use based on this channel name
                        name: "defmt"
                    }
                }
            };

            rtt_target::set_defmt_channel(channels.up.0);
        }
    }
}

#[cfg(feature = "std")]
mod backend {
    pub use std::println as debug_output_println;

    #[doc(hidden)]
    pub fn init() {
        #[cfg(feature = "log")]
        crate::logger::init();
    }
}

#[cfg(not(any(feature = "defmt-rtt", feature = "rtt-target", feature = "std")))]
mod backend {
    #[doc(hidden)]
    pub fn init() {}
}

pub use backend::*;
