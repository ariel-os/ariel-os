//! riot-rs
//!
//! This is a meta-package, pulling in the sub-crates of RIOT-rs.
//!
//! # Cargo features
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]
#![no_std]
#![feature(doc_cfg)]

pub use riot_rs_buildinfo as buildinfo;
pub use riot_rs_debug as debug;
pub use riot_rs_embassy::{self as embassy, define_peripherals};
pub use riot_rs_rt as rt;

// Attribute macros
pub use riot_rs_macros::config;
#[cfg(any(feature = "threading", doc))]
#[doc(cfg(feature = "threading"))]
pub use riot_rs_macros::thread;

#[cfg(feature = "threading")]
pub use riot_rs_threads as thread;

// These are used by proc-macros we provide
pub use linkme;
pub use static_cell;

// ensure this gets linked
use riot_rs_boards as _;
