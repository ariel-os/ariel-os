//! Common types for GNSS functionality across different HALs.
use core::fmt::Debug;
use derive_builder::Builder;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex,
    watch::{Receiver, Sender, Watch},
};

/// Maximum number of concurrent receivers for GNSS data.
pub const MAX_WATCH_RECEIVERS: usize = 4;
/// Embassy Watch to hold the latest GNSS data and notify receivers.
pub type GnssDataWatch = Watch<CriticalSectionRawMutex, GnssData, MAX_WATCH_RECEIVERS>;
/// Type alias for the receiver of GNSS data (receiver of `GnssDataWatch`).
pub type GnssDataReceiver<'a> =
    Receiver<'a, CriticalSectionRawMutex, GnssData, MAX_WATCH_RECEIVERS>;
/// Type alias for the sender of GNSS data (sender of `GnssDataWatch`).
#[doc(hidden)]
pub type GnssDataSender<'a> = Sender<'a, CriticalSectionRawMutex, GnssData, MAX_WATCH_RECEIVERS>;

/// Operation modes for the GNSS module.
#[derive(Debug, Copy, Clone, Eq, PartialEq)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub enum GnssOperationMode {
    /// Always keep the GNSS module active.
    Continuous,
    /// Update the GNSS fix periodically. Period is defined in seconds.
    Periodic(u16),
    /// Try to get a GNSS fix only when requested, you won't be able to get updates using `get_receiver`. Timeout is defined in seconds.
    SingleShot(u16),
}

impl core::fmt::Display for GnssOperationMode {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// Configuration for the GNSS.
///
/// You can customize it using the `gnss-config-override` feature.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct Config {
    /// The mode of GNSS to use.
    pub operation_mode: GnssOperationMode,
}

impl Config {
    /// Creates a new `Config` with the specified operation mode.
    #[must_use]
    pub const fn new(operation_mode: GnssOperationMode) -> Self {
        Self { operation_mode }
    }
}

impl Default for Config {
    fn default() -> Self {
        Config::new(GnssOperationMode::Continuous)
    }
}

impl core::fmt::Display for Config {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// Trait for fixed-point numbers stored in 32-bit.
///
/// The underlying representation is expected to be u32 and to use the wrapping behavior when converting to signed types.
///
/// Conversion example:
///
///     let scale = 1 << TypeName::FRAC_BITS;
///     if TypeName::SIGNED {
///         let scaled = (value * scale as f32).round() as i32;
///         TypeName::from_bits(scaled as u32)
///     } else {
///         let scaled = (value * scale as f32).round() as u32;
///         TypeName::from_bits(scaled)
///     }
pub trait FixedPoint32 {
    /// Number of fractional bits.
    /// Max 31 for signed, max 32 for unsigned.
    const FRAC_BITS: u8;
    /// Whether the fixed-point number is signed.
    const SIGNED: bool;

    /// Create the fixed-point number from raw bits.
    fn from_bits(bits: u32) -> Self;

    // fn from_f32(value: f32) -> Self
    // where
    //     Self: Sized,
    // {
    //     let scale = 1 << Self::FRAC_BITS;
    //     if Self::SIGNED {
    //         let scaled = (value * scale as f32).round() as i32;
    //         Self::from_bits(scaled as u32)
    //     } else {
    //         let scaled = (value * scale as f32).round() as u32;
    //         Self::from_bits(scaled)
    //     }
    // }

    /// Get the raw bits of the fixed-point number.
    // Should be fine use `as u32` on signed types as well, as we only use the raw bits
    fn as_bits(&self) -> u32;

    /// Get the big-endian byte representation of the fixed-point number.
    fn as_be_bytes(&self) -> [u8; 4] {
        self.as_bits().to_be_bytes()
    }

    /// Lossy conversion to f32
    #[allow(clippy::cast_precision_loss)]
    // we actually want the wrapping behavior here
    #[allow(clippy::cast_possible_wrap)]
    fn as_f32(&self) -> f32 {
        let scale = 1 << Self::FRAC_BITS;
        if Self::SIGNED {
            (self.as_bits() as i32) as f32 / scale as f32
        } else {
            self.as_bits() as f32 / scale as f32
        }
    }

    /// Display the fixed-point number in decimal format.
    /// This is to be used in the `Display` implementation.
    ///
    /// # Errors
    ///
    /// An error is returned if writing to the formatter fails.
    #[allow(clippy::cast_possible_wrap)]
    fn display_format(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let integer_part = self.as_bits() >> Self::FRAC_BITS;
        let mut fractional_part = self.as_bits() & ((1 << Self::FRAC_BITS) - 1);
        if Self::SIGNED {
            write!(f, "{}", integer_part as i32)?;
        } else {
            write!(f, "{integer_part}")?;
        }

        if fractional_part == 0 {
            return Ok(());
        }
        write!(f, ".")?;

        while fractional_part > 0 {
            // Multiply by 10 to get the next decimal digit
            fractional_part *= 10;
            let digit = fractional_part >> Self::FRAC_BITS;
            write!(f, "{digit}")?;
            // Remove the integer part
            fractional_part &= (1 << Self::FRAC_BITS) - 1;
        }
        Ok(())
    }
}

/// Trait for fixed-point numbers stored in 16-bit.
/// The underlying representation is expected to be u32 and to use the wrapping behavior when converting to signed types.
///
/// Conversion example:
///
///     let scale = 1 << TypeName::FRAC_BITS;
///     if TypeName::SIGNED {
///         let scaled = (value * scale as f32).round() as i16;
///         TypeName::from_bits(scaled as u16)
///     } else {
///         let scaled = (value * scale as f32).round() as u16;
///         TypeName::from_bits(scaled)
///     }
pub trait FixedPoint16 {
    /// Number of fractional bits.
    /// Max 15 for signed, max 16 for unsigned.
    const FRAC_BITS: u8;
    /// Whether the fixed-point number is signed.
    const SIGNED: bool;

    /// Create the fixed-point number from raw bits.
    fn from_bits(bits: u16) -> Self;

    /// Get the raw bits of the fixed-point number.
    // Should be fine use `as u16` on signed types as well, as we only use the raw bits
    fn as_bits(&self) -> u16;

    /// Get the big-endian byte representation of the fixed-point number.
    fn as_be_bytes(&self) -> [u8; 2] {
        self.as_bits().to_be_bytes()
    }

    /// Lossless Conversion to f32
    // we actually want the wrapping behavior here
    #[allow(clippy::cast_possible_wrap)]
    fn as_f32(&self) -> f32 {
        let scale: u16 = 1 << Self::FRAC_BITS;
        if Self::SIGNED {
            f32::from(self.as_bits() as i16) / f32::from(scale)
        } else {
            f32::from(self.as_bits()) / f32::from(scale)
        }
    }

    /// Display the fixed-point number in decimal format.
    /// This is to be used in the `Display` implementation.
    ///
    /// # Errors
    ///
    /// An error is returned if writing to the formatter fails.
    #[allow(clippy::cast_possible_wrap)]
    fn display_format(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let integer_part = self.as_bits() >> Self::FRAC_BITS;
        let mut fractional_part = self.as_bits() & ((1 << Self::FRAC_BITS) - 1);
        if Self::SIGNED {
            write!(f, "{}", integer_part as i16)?;
        } else {
            write!(f, "{integer_part}")?;
        }

        if fractional_part == 0 {
            return Ok(());
        }
        write!(f, ".")?;

        while fractional_part > 0 {
            // Multiply by 10 to get the next decimal digit
            fractional_part *= 10;
            let digit = fractional_part >> Self::FRAC_BITS;
            write!(f, "{digit}")?;
            // Remove the integer part
            fractional_part &= (1 << Self::FRAC_BITS) - 1;
        }
        Ok(())
    }
}

/// Longitude in decimal degrees.
/// Stored in fixed-point format.
///
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 23 fractional bits, be signed and stored in an i32.
/// 1 sign bit, 8 integer bits, 23 fractional bits, this makes the difference between two consecutive values at the equator of 40,075,016.7/(360*2^23) ~= 0.013 meters
/// Range from -180 to 180 degrees.
#[derive(Copy, Clone)]
pub struct Longitude {
    inner: u32,
}
impl FixedPoint32 for Longitude {
    const FRAC_BITS: u8 = 23;
    const SIGNED: bool = true;

    fn from_bits(bits: u32) -> Self {
        Self { inner: bits }
    }
    fn as_bits(&self) -> u32 {
        self.inner
    }
}

impl core::fmt::Display for Longitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display_format(f)
    }
}

impl Debug for Longitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Longitude({self})")
    }
}

#[cfg(feature = "defmt")]
impl defmt::Format for Longitude {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{}", self);
    }
}

/// Latitude in decimal degrees
/// Stored in fixed-point format.
///
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 23 fractional bits, be signed and stored in an i32.
/// 1 sign bit, 8 integer bits, 23 fractional bits, this makes the difference between two consecutive values at the equator of 40,075,016.7/(360*2^23) ~= 0.013 meters
/// Range from -90 to 90 degrees.
#[derive(Copy, Clone)]
pub struct Latitude {
    inner: u32,
}

impl FixedPoint32 for Latitude {
    const FRAC_BITS: u8 = 23;
    const SIGNED: bool = true;

    fn as_bits(&self) -> u32 {
        self.inner
    }
    fn from_bits(bits: u32) -> Self {
        Self { inner: bits }
    }
}
impl core::fmt::Display for Latitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display_format(f)
    }
}
impl Debug for Latitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Latitude({self})")
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Latitude {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{}", self);
    }
}

/// Altitude in meters above sea level.
/// Stored in fixed-point format.
///
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 8 fractional bits, be signed and stored in an i32.
/// 1 sign bit, 23 integer bits, 8 fractional bits, this gives a maximum altitude of ~8388 km. The difference between two consecutive values is 1/2^8 ~= 0.004 meters
#[derive(Copy, Clone)]
pub struct Altitude {
    inner: u32,
}
impl FixedPoint32 for Altitude {
    const FRAC_BITS: u8 = 8;
    const SIGNED: bool = true;

    #[allow(clippy::cast_sign_loss)]
    fn as_bits(&self) -> u32 {
        self.inner
    }
    // we actually want the wrapping behavior here
    #[allow(clippy::cast_possible_wrap)]
    fn from_bits(bits: u32) -> Self {
        Self { inner: bits }
    }
}
impl core::fmt::Display for Altitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display_format(f)
    }
}
impl Debug for Altitude {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Altitude({self})")
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Altitude {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{}", self);
    }
}

/// Accuracy in meters.
/// Stored in fixed-point format.
///
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 8 fractional bits, be unsigned and stored in an u16.
/// 8 integer bits, 8 fractional bits, range from around 0 to 255 meters, with a difference between two consecutive values of 1/2^8 ~= 0.004 meters
#[derive(Copy, Clone)]
pub struct Accuracy {
    inner: u16,
}
impl FixedPoint16 for Accuracy {
    const FRAC_BITS: u8 = 8;
    const SIGNED: bool = false;
    fn as_bits(&self) -> u16 {
        self.inner
    }
    fn from_bits(bits: u16) -> Self {
        Self { inner: bits }
    }
}
impl core::fmt::Display for Accuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display_format(f)
    }
}
impl Debug for Accuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Accuracy({self})")
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Accuracy {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{}", self);
    }
}

/// Ground speed in m/s.
/// Stored in fixed-point format.
///
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 6 fractional bits, be unsigned and stored in an u16.
/// 10 integer bits, 6 fractional bits. Makes max speed of around 1023 m/s. Difference between two consecutive values is 1/2^6 ~= 0.0016 m/s or 0.057 km/h.
#[derive(Copy, Clone)]
pub struct GroundSpeed {
    inner: u16,
}
impl FixedPoint16 for GroundSpeed {
    const FRAC_BITS: u8 = 6;
    const SIGNED: bool = false;
    fn as_bits(&self) -> u16 {
        self.inner
    }
    fn from_bits(bits: u16) -> Self {
        Self { inner: bits }
    }
}

impl core::fmt::Display for GroundSpeed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display_format(f)
    }
}
impl Debug for GroundSpeed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "GroundSpeed({self})")
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for GroundSpeed {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{}", self);
    }
}

/// Speed accuracy in m/s.
/// Stored in fixed-point format.
///
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 6 fractional bits, be unsigned and stored in an u16.
/// 10 integer bits, 6 fractional bits. Makes max speed accuracy of around 1023 m/s. Difference between two consecutive values is 1/2^6 ~= 0.0016 m/s or 0.057 km/h.
#[derive(Copy, Clone)]
pub struct SpeedAccuracy {
    inner: u16,
}
impl FixedPoint16 for SpeedAccuracy {
    const FRAC_BITS: u8 = 6;
    const SIGNED: bool = false;
    fn as_bits(&self) -> u16 {
        self.inner
    }
    fn from_bits(bits: u16) -> Self {
        Self { inner: bits }
    }
}

impl core::fmt::Display for SpeedAccuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display_format(f)
    }
}

impl Debug for SpeedAccuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "SpeedAccuracy({self})")
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for SpeedAccuracy {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{}", self);
    }
}

/// Vertical speed in m/s.
/// Stored in fixed-point format.
///
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 6 fractional bits, be signed and stored in an i16.
/// 1 sign bit, 9 integer bits, 6 fractional bits. Makes max vertical speed of around 511 m/s. Difference between two consecutive values is 1/2^6 ~= 0.0016 m/s or 0.057 km/h.
#[derive(Copy, Clone)]
pub struct VerticalSpeed {
    inner: u16,
}
impl FixedPoint16 for VerticalSpeed {
    const FRAC_BITS: u8 = 6;
    const SIGNED: bool = true;
    fn as_bits(&self) -> u16 {
        self.inner
    }

    // we actually want the wrapping behavior here
    #[allow(clippy::cast_possible_wrap)]
    fn from_bits(bits: u16) -> Self {
        Self { inner: bits }
    }
}

impl core::fmt::Display for VerticalSpeed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display_format(f)
    }
}
impl Debug for VerticalSpeed {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "VerticalSpeed({self})")
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for VerticalSpeed {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{}", self);
    }
}

/// Heading in degrees (0–360).
/// Stored in fixed-point format.
///
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 7 fractional bits, be unsigned and stored in an u16.
/// 9 integer bits, 7 fractional bits. Difference between two consecutive values is 1/2^7 ~= 0.008 degrees
#[derive(Copy, Clone)]
pub struct Heading {
    inner: u16,
}
impl FixedPoint16 for Heading {
    const FRAC_BITS: u8 = 7;
    const SIGNED: bool = false;
    fn as_bits(&self) -> u16 {
        self.inner
    }
    fn from_bits(bits: u16) -> Self {
        Self { inner: bits }
    }
}
impl core::fmt::Display for Heading {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display_format(f)
    }
}
impl Debug for Heading {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "Heading({self})")
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for Heading {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{}", self);
    }
}
/// Heading precision in degrees, maximum value is 360.
/// Stored in fixed-point format.
///
/// To initialize this struct, you need to provide a fixed-point representation of the value.
/// This fixed-point representation should have 7 fractional bits, be unsigned and stored in an u16.
/// 9 integer bits, 7 fractional bits. Difference between two consecutive values is 1/2^7 ~= 0.008 degrees
#[derive(Copy, Clone)]
pub struct HeadingAccuracy {
    inner: u16,
}
impl FixedPoint16 for HeadingAccuracy {
    const FRAC_BITS: u8 = 7;
    const SIGNED: bool = false;
    fn as_bits(&self) -> u16 {
        self.inner
    }
    fn from_bits(bits: u16) -> Self {
        Self { inner: bits }
    }
}
impl core::fmt::Display for HeadingAccuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.display_format(f)
    }
}
impl Debug for HeadingAccuracy {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "HeadingAccuracy({self})")
    }
}
#[cfg(feature = "defmt")]
impl defmt::Format for HeadingAccuracy {
    fn format(&self, fmt: defmt::Formatter<'_>) {
        defmt::write!(fmt, "{}", self);
    }
}
/// Represents position data from GNSS.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssPosition {
    /// Longitude in decimal degrees
    /// Positive values indicate east, negative values indicate west.
    /// From -180 to 180
    pub longitude: Longitude,
    /// Latitude in decimal degrees.
    /// Positive values indicate north, negative values indicate south.
    /// From -90 to 90
    pub latitude: Latitude,
    /// Altitude in meters above sea level
    pub altitude: Altitude,
    /// Horizontal accuracy in meters, the calculation of this value depends on the GNSS module.
    /// An estimation from the HDOP can be done by multiplying the HDOP by the announced horizontal accuracy of the GNSS module.
    /// Value range between 0 and 255 meters.
    pub accuracy: Accuracy,
    /// Altitude accuracy in meters, the calculation of this value depends on the GNSS module.
    /// An estimation from the VDOP can be done by multiplying the VDOP by the announced vertical accuracy of the GNSS module.
    /// Value range between 0 and 255 meters.
    pub altitude_accuracy: Accuracy,
}

impl core::fmt::Display for GnssPosition {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "GnssPosition {{ longitude: {}, latitude: {}, altitude: {}, accuracy: {}, altitude_accuracy: {} }}",
            self.longitude, self.latitude, self.altitude, self.accuracy, self.altitude_accuracy
        )
    }
}

/// Represents velocity data from GNSS.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssVelocity {
    /// Speed in m/s
    pub speed: GroundSpeed,
    /// Speed accuracy in m/s
    pub speed_accuracy: SpeedAccuracy,
    /// Vertical speed in m/s
    pub vertical_speed: VerticalSpeed, // Vertical speed can be negative
    /// Vertical speed accuracy in m/s
    pub vertical_speed_accuracy: SpeedAccuracy,
    /// Heading in degrees (0–360)
    pub heading: Heading,
    /// Heading accuracy in degrees
    pub heading_accuracy: HeadingAccuracy,
}

// #[cfg(feature = "defmt")]
// impl defmt::Format for GnssVelocity {
//     fn format(&self, fmt: defmt::Formatter<'_>) {
//         defmt::write!(
//             fmt,
//             "GnssPosition {{ speed: {}, speed_accuracy: {}, vertical_speed: {}, vertical_speed_accuracy: {}, heading: {}, heading_accuracy: {} }}",
//             self.speed.to_num::<f32>(),
//             self.speed_accuracy.to_num::<f32>(),
//             self.vertical_speed.to_num::<f32>(),
//             self.vertical_speed_accuracy.to_num::<f32>(),
//             self.heading.to_num::<f32>(),
//             self.heading_accuracy.to_num::<f32>()
//         );
//     }
// }

impl core::fmt::Display for GnssVelocity {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(
            f,
            "GnssVelocity {{ speed: {}, speed_accuracy: {}, vertical_speed: {}, vertical_speed_accuracy: {}, heading: {}, heading_accuracy: {} }}",
            self.speed,
            self.speed_accuracy,
            self.vertical_speed,
            self.vertical_speed_accuracy,
            self.heading,
            self.heading_accuracy
        )
    }
}

/// Represents date and time information from GNSS.
#[derive(Debug, Copy, Clone)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
pub struct GnssDateTime {
    /// Seconds since Ariel epoch (2024-01-01T00:00:00Z)
    pub seconds: u32,
    /// Milliseconds (0-999)
    pub milliseconds: u16,
}

impl core::fmt::Display for GnssDateTime {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}

/// Represents GNSS data that can be received from the GNSS module.
///
/// A field can be `None` if the GNSS module did not provide that information.
#[derive(Debug, Copy, Clone, Builder)]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[builder(no_std, build_fn(error(validation_error = false)))]
pub struct GnssData {
    /// The position data, if available.
    #[builder(setter(into, strip_option), default)]
    position: Option<GnssPosition>,
    /// The velocity data, if available.
    #[builder(setter(into, strip_option), default)]
    velocity: Option<GnssVelocity>,
    /// The date and time information, if available.
    #[builder(setter(into, strip_option), default)]
    datetime: Option<GnssDateTime>,
}

impl GnssData {
    /// Get the position data, if available.
    #[must_use]
    pub fn position(&self) -> Option<GnssPosition> {
        self.position
    }
    /// Get the velocity data, if available.
    #[must_use]
    pub fn velocity(&self) -> Option<GnssVelocity> {
        self.velocity
    }
    /// Get the date and time information, if available.
    #[must_use]
    pub fn datetime(&self) -> Option<GnssDateTime> {
        self.datetime
    }
}

impl core::fmt::Display for GnssData {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(self, f)
    }
}
