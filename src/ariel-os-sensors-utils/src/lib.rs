//! Provides utils useful for sensor drivers implementations.

#![no_std]
#![deny(missing_docs)]

mod atomic_state;

pub use atomic_state::AtomicState;
