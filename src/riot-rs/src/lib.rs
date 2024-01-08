//! riot-rs
//!
//! This is a meta-package, pulling in the sub-crates of RIOT-rs.

#![no_std]

pub use riot_rs_buildinfo as buildinfo;
pub use riot_rs_core::thread;
pub use riot_rs_embassy::{self as embassy, assign_resources};
pub use riot_rs_rt as rt;

// ensure this gets linked
use riot_rs_boards as _;
