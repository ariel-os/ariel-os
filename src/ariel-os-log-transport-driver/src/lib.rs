//! Provides logging transport drivers.
//!
//! Only one driver can be selected at a time.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

#[featurecomb::comb]
mod _featurecomb {}

cfg_select! {
    feature = "uart" => {
        mod uart;
        pub use uart::init;
    }
    _ => {
        mod dummy;
        pub use dummy::init;
    }
}
