//! Ariel OS is an operating system for secure, memory-safe, low-power Internet of Things (IoT).
//! Supported hardware includes various 32-bit microcontrollers.
//!
//! This is the API documentation for Ariel OS.
//! Orher resources available are:
//! - 📔 Extensive documentation Ariel OS can be found in the
//!   [book](https://ariel-os.github.io/ariel-os/dev/docs/book/).
//! - ⚙️  The git repository is available on
//!   [GitHub](https://github.com/ariel-os/ariel-os).
//! - ✨ [Examples](https://github.com/ariel-os/ariel-os/tree/main/examples)
//!   demonstrating various features of Ariel OS.
//! - 🧪 A set of [test cases](https://github.com/ariel-os/ariel-os/tree/main/tests)
//!   to further verify the capabilities of Ariel OS.
//!
//! # Structure
//!
//! Ariel OS is highly modular with a significant number of feature flags
//! to configure the operating system.
//!
//! ## Feature flags
//!
//! The overview of feature flags is shown
//! [below](https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/#cargo-features).
//!
//! While more feature flags are used in Ariel OS,
//! the list below is pruned to only contain the feature flags relevant for developers
//!
//! # Cargo features
#![doc = document_features::document_features!(feature_label = r#"<span class="stab portability"><code>{feature}</code></span>"#)]
#![no_std]
#![feature(doc_auto_cfg)]
#![deny(missing_docs)]

#[cfg(feature = "bench")]
#[doc(inline)]
pub use ariel_os_bench as bench;
#[doc(inline)]
pub use ariel_os_buildinfo as buildinfo;
#[cfg(feature = "coap")]
#[doc(inline)]
pub use ariel_os_coap as coap;
#[doc(inline)]
pub use ariel_os_debug as debug;
#[doc(inline)]
pub use ariel_os_identity as identity;
#[cfg(feature = "random")]
#[doc(inline)]
pub use ariel_os_random as random;
#[cfg(feature = "storage")]
#[doc(inline)]
pub use ariel_os_storage as storage;
#[cfg(feature = "threading")]
#[doc(inline)]
pub use ariel_os_threads as thread;

// Attribute macros
pub use ariel_os_macros::config;
pub use ariel_os_macros::spawner;
pub use ariel_os_macros::task;
#[cfg(any(feature = "threading", doc))]
pub use ariel_os_macros::thread;

// ensure this gets linked
use ariel_os_boards as _;

pub use ariel_os_embassy::api::*;

pub mod config {
    //! Provides configuration to the system and the application.

    pub use ariel_os_utils::{
        ipv4_addr_from_env, ipv4_addr_from_env_or, ipv6_addr_from_env, ipv6_addr_from_env_or,
        str_from_env, str_from_env_or,
    };
}

/// This module contains all third party crates as used by Ariel OS.
///
/// TODO: The version of this crate (`ariel-os`) will correspond to changes in
/// these dependencies (keeping semver guarantees).
pub mod reexports {
    pub use ariel_os_embassy::reexports::*;
    // These are used by proc-macros we provide
    pub use linkme;
    pub use static_cell;
}
