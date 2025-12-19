//! Provides a sensor abstraction layer.
//!
//! # Definitions
//!
//! In the context of this abstraction:
//!
//! - A *sensor device* is a device measuring one or multiple physical quantities and reporting
//!   them as one or more digital values—we call these values *samples*.
//! - Sensor devices measuring the same physical quantity are said to be part of the same *sensor
//!   category*.
//!   A sensor device may be part of multiple sensor categories.
//! - A *measurement* is the physical operation of measuring one or several physical quantities.
//! - A *reading* is the digital result returned by a sensor device after carrying out a
//!   measurement.
//!   Samples of different physical quantities can therefore be part of the same reading.
//! - A *sensor driver* refers to a sensor device as exposed by the sensor abstraction layer.
//! - A *sensor driver instance* is an instance of a sensor driver.
//!
//! # Goals
//!
//! This abstraction has two main goals:
//!
//! - Providing a unified way of accessing the readings from all registered sensor driver instances
//!   in a homogeneous way.
//! - Making it easy and as transparent as possible to substitute a specific sensor device by a
//!   similar one from the same category.
//!
//! # Accessing sensor driver instances
//!
//! Registered sensor driver instances can be accessed using
//! [`REGISTRY::sensors()`](registry::Registry::sensors).
//! Sensor drivers implement the [`Sensor`] trait, which allows to trigger measurements and obtain
//! the resulting readings.
//!
//! # Obtaining a sensor reading
//!
//! After triggering a measurement with [`Sensor::trigger_measurement()`], a reading can be
//! obtained using [`Sensor::wait_for_reading()`].
//! [`Sensor::wait_for_reading()`] returns a [`Samples`], a data "tuple" containing values returned
//! by the sensor driver along with their associated [`ReadingChannel`]s, which identify which
//! physical quantity the samples are about.
//! For instance, [`ReadingChannel`]s allow to disambiguate the samples provided by a temperature
//! & humidity sensor.
//!
//! To avoid handling floats, [`Sample`]s returned by [`Sensor::wait_for_reading()`]
//! are integers, and a fixed scaling value is [provided in
//! `ReadingChannel`][ReadingChannel::scaling] for each [`Sample`]
//! returned.
//!
//! Additionally, [`Sensor::reading_channels()`] returns a [`ReadingChannels`], which lists the
//! [`ReadingChannel`]s a sensor driver returns, in the same order as [`Samples`].
//! For instance, this can be used to pre-render a table of readings, without having to trigger
//! measurements.
//!
//! # For implementors
//!
//! Sensor drivers must implement the [`Sensor`] trait.
//!
//! [`Sample`]: ariel_os_sensors::sensor::Sample
//! [`Samples`]: ariel_os_sensors::sensor::Samples
//! [`ReadingChannel`]: ariel_os_sensors::sensor::ReadingChannel
//! [ReadingChannel::scaling]: ariel_os_sensors::sensor::ReadingChannel::scaling()
//! [`ReadingChannels`]: ariel_os_sensors::sensor::ReadingChannels

pub use ariel_os_sensors::*;
#[doc(inline)]
pub use ariel_os_sensors_registry as registry;
pub use ariel_os_sensors_registry::{REGISTRY, SENSOR_REFS};
