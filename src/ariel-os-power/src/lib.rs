//! Provides power management functionality.

#![deny(missing_docs)]
#![cfg_attr(not(context = "native"), no_std)]

mod exit;
mod reset;

pub use exit::*;
pub use reset::*;

fn hooks() {
    #[cfg(feature = "log-finish-hook")]
    ariel_os_log::transport::finish_transport();
}
