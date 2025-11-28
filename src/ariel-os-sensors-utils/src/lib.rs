//! Provides utils useful for sensor drivers implementations.

#![no_std]
// #![deny(missing_docs)]

mod atomic_state;
mod sensor_signaling;

pub use atomic_state::AtomicState;
pub use sensor_signaling::SensorSignaling;
