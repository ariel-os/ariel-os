//! Provides utils useful for sensor drivers implementations.

#![no_std]
// #![deny(missing_docs)]

mod sensor_signaling;
mod state_atomic;

pub use sensor_signaling::SensorSignaling;
pub use state_atomic::StateAtomic;
