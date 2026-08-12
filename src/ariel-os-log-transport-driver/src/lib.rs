//! Provides logging transport adapters.

#![cfg_attr(not(test), no_std)]
#![cfg_attr(nightly, feature(doc_cfg))]
#![deny(missing_docs)]

cfg_select! {
    feature = "usb-cdc-acm-esp" => {
        mod usb_cdc_acm_esp;
        pub use usb_cdc_acm_esp::init;
    }
    _ => {
        mod dummy;
        pub use dummy::init;
    }
}
