//! Provides logging transport adapters.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

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
