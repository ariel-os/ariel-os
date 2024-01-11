//! riot-rs
//!
//! This is a meta-package, pulling in the sub-crates of RIOT-rs.

#![no_std]

pub use riot_rs_buildinfo as buildinfo;
pub use riot_rs_embassy::{self as embassy, define_peripherals};
pub use riot_rs_rt as rt;

#[cfg(feature = "threading")]
pub use riot_rs_threads as thread;

// ensure this gets linked
use riot_rs_boards as _;
