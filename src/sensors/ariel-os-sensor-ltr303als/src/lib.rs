//! Driver for the [LTR-303ALS-01] digital ambiant light sensor.
//!
//! Compatible with [`ariel_os_sensors::Sensor`].
//!
//! [LTR-303ALS-01]: https://optoelectronics.liteon.com/upload/download/DS86-2013-0004/LTR-303ALS-01_DS_V1.1.PDF
//! [Appendix-A]: https://github.com/latonita/datasheets-storage/blob/main/sensors/LTR-303%20329_Appendix%20A%20Ver_1.0_22%20Feb%202013.pdf
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

pub mod i2c;

use ariel_os_sensors::sensor::{Sample, SampleMetadata};

const PART_NUMBER: &str = "LTR-303ALS-01";

pub(crate) const INITIAL_STARTUP_TIME_MS: u64 = 100;
pub(crate) const WAKEUP_TIME_MS: u64 = 40;

/// Adresses of the registers of the sensor. See page 13.
#[expect(dead_code)]
enum Register {
    /// Controls the Gain setting, ALS operations modes
    /// and Software reset. See page 14.
    Ctrl = 0x80,
    /// Controls the integration time and timing of the periodic measurement
    /// of the ALS in active mode. See page 15.
    MeasureCtrl = 0x85,

    /// Register that should read [`PART_ID`]. See page 16.
    PartId = 0x86,
    /// Register should read [`MANUFAC_ID`]. See page 16.
    ManufacturerId = 0x87,

    /// Lower byte of the Channel 1 Data. See page 17.
    Channel1Low = 0x88,
    /// Upper byte of the Channel 1 Data. See page 17.
    Channel1High = 0x89,
    /// Lower byte of the Channel 0 Data. See page 18.
    Channel0Low = 0x8A,
    /// Upper byte of the Channel 0 Data. See page 18.
    Channel0High = 0x8B,

    /// Stores the status of the ALS data including:
    /// - The validity of the data
    /// - The gain used to measure that data
    /// - The interrupt status
    /// - And Whether the currently stored data has been read already or not
    ///
    /// See page 19.
    StatusReg = 0x8C,

    /// Controls the operation of the interrupt pin.
    /// This regsiter should be set before the sensor is switched to active mode.
    /// See page 20.
    InterruptReg = 0x8F,

    /// Lower byte of the upper threshold. See page 21.
    UpperThresholdLow = 0x97,
    /// Higher byte of the upper threshold. See page 21.
    UpperThresholdHigh = 0x98,
    /// Lower byte of the lower threshold. See page 21.
    LowerThresholdLow = 0x99,
    /// Upper byte of the lower threshold. See page 21.
    LowerThresholdHigh = 0x9A,

    /// Controls the number of times that the measurement
    /// needs to be out of the threshold ranges before
    /// starting an interrupt. See page 22.
    InterruptPersistReg = 0x9E,
}

/// Gain used by the ADC.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlsGain {
    /// 1 Lux -> 64k Lux.
    #[default]
    _1 = 0b000 << 2,
    /// 0.5 Lux -> 32k Lux.
    _2 = 0b001 << 2,
    /// 0.25 Lux -> 16k Lux.
    _4 = 0b010 << 2,
    /// 0.125 Lux -> 8k Lux.
    _8 = 0b011 << 2,
    /// 0.02 Lux -> 1.3k Lux.
    _48 = 0b110 << 2,
    /// 0.01 Lux -> 600 Lux.
    _96 = 0b111 << 2,
}

/// Set to 1 to initiate the software reset procedure.
const SOFT_RESET: u8 = 1 << 1;
/// Set to 1 to activate the sensor.
const ALS_MODE: u8 = 1 << 0;

/// Time taken by the ALS to make it measurements.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlsIntegrationTimeMs {
    /// 100 ms.
    #[default]
    _100 = 0b000 << 3,
    /// 50 ms.
    _50 = 0b001 << 3,
    /// 200 ms.
    _200 = 0b010 << 3,
    /// 400 ms.
    _400 = 0b011 << 3,
    /// 150 ms.
    _150 = 0b100 << 3,
    /// 250 ms.
    _250 = 0b101 << 3,
    /// 300 ms.
    _300 = 0b110 << 3,
    /// 350 ms.
    _350 = 0b111 << 3,
}

impl AlsIntegrationTimeMs {
    fn from_measure_ctrl_reg(reg: u8) -> Self {
        match reg & (0b111 << 3) {
            u if u == Self::_50 as u8 => Self::_50,
            u if u == Self::_100 as u8 => Self::_100,
            u if u == Self::_150 as u8 => Self::_150,
            u if u == Self::_200 as u8 => Self::_200,
            u if u == Self::_250 as u8 => Self::_250,
            u if u == Self::_300 as u8 => Self::_300,
            u if u == Self::_350 as u8 => Self::_350,
            u if u == Self::_400 as u8 => Self::_400,
            _ => unreachable!(),
        }
    }
}

/// Time interval before the update of the ALS data registers.
///
/// This has to been superior or equal the ALS integration time.
/// If not, the integrated circuit of the sensor will reset the integration
/// time to be equal to the measurement rate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlsMeasurementRateMs {
    /// 500 ms.
    #[default]
    _500 = 0b011,
    /// 50 ms.
    _50 = 0b000,
    /// 100 ms.
    _100 = 0b001,
    /// 200 ms.
    _200 = 0b010,
    /// 1s.
    _1000 = 0b100,
    /// 2 s.
    /// Setting the bits to `0b110` or `0b111`
    /// also leads to a 2s measurement rate.
    _2000 = 0b101,
}

/// Hardcoded value present in the `PartId` [`Register`] variant.
/// Represent the concatenation of the Part Id and the Revision ID.
/// See page 17 of the datasheet.
pub const PART_ID: u8 = 0xA0;
/// Hardcoded value present in the `ManufacturerId` [`Register`] variant.
/// See page 17 of the datasheet.
pub const MANUFACTURER_ID: u8 = 0x05;

/// This bit is set to 1 when the data is invalid
/// See page 19 of the datasheet.
const ALS_DATA_VALIDITY: u8 = 1 << 7;

/// The gain that was used in the currently stored measurement.
/// See page 19 of the datasheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MeasuredAlsDataGain {
    /// Data measured with x1 gain (default).
    _1 = 0b000 << 4,
    /// Data measured with x2 gain.
    _2 = 0b001 << 4,
    /// Data measured with x4 gain.
    _4 = 0b010 << 4,
    /// Data measured with x8 gain.
    _8 = 0b011 << 4,
    /// Data measured with x48 gain.
    _48 = 0b110 << 4,
    /// Data measured with x96 gain.
    _96 = 0b111 << 4,
}

impl MeasuredAlsDataGain {
    fn from_status_reg(status: u8) -> Self {
        match status & (0b111 << 4) {
            u if u == Self::_1 as u8 => Self::_1,
            u if u == Self::_2 as u8 => Self::_2,
            u if u == Self::_4 as u8 => Self::_4,
            u if u == Self::_8 as u8 => Self::_8,
            u if u == Self::_48 as u8 => Self::_48,
            u if u == Self::_96 as u8 => Self::_96,
            _ => unreachable!("Invalid gains"),
        }
    }
}

/// This bit is set to 1 when an interrupt is active.
/// See page 19 of the datasheet.
const ALS_INTERRUPT_STATUS: u8 = 1 << 3;

/// This bit is set to 1 to indicate that the data
/// present in the channel1/0 registers hasn't been read yet.
/// See page 19 of the datasheet.
const ALS_DATA_STATUS: u8 = 1 << 2;

/// Describes and defines when the INT pin is considered active:
/// - 0: when it's a logical 0
/// - 1: when it's a logical 1
///
/// See page 20 of the datasheet.
const INTERRUPT_POLARITY: u8 = 1 << 2;

/// Set to 1 to allow ALS measurements to trigger interrupts.
/// See page 20 of the datasheet.
const INTERRUPT_MODE: u8 = 1 << 1;

/// Controls the number of times that the ALS measurements must be outside of
/// the threshold ranges before triggering an interrupt on the INT pin.
/// See page 21 of the datasheet.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlsPersist {
    /// Every ALS measurement outside of the threshold range
    /// can trigger an interrupt.
    #[default]
    _1 = 0b0000,
    /// Two consecutive ALS measurements outside of the threshold range
    /// are needed.
    _2 = 0b0001,
    /// Sixteen consecutive ALS measurements outside of the threshold range
    /// are needed.
    _16 = 0b1111,
}

/// This function converts raw lux data into physical lux data following the procedure
/// described in the [Appendix-A].
fn physical_lux_from_raw(
    channel1: u16,
    channel0: u16,
    als_gain: MeasuredAlsDataGain,
    als_integration_time: AlsIntegrationTimeMs,
) -> Sample {
    // The fifteen extra bits (from u16 to i32) allows us to have
    // some oppurtunities to multiply to get integers.

    // Already integer values, don't scale them
    let ch1 = i32::from(channel1);
    let ch0 = i32::from(channel0);

    // Already integers, don't scale
    let als_gain = match als_gain {
        MeasuredAlsDataGain::_1 => 1,
        MeasuredAlsDataGain::_2 => 2,
        MeasuredAlsDataGain::_4 => 4,
        MeasuredAlsDataGain::_8 => 8,
        MeasuredAlsDataGain::_48 => 48,
        MeasuredAlsDataGain::_96 => 96,
    };

    // Compared to data sheet, multiply by 10 to get integers
    let als_int = match als_integration_time {
        AlsIntegrationTimeMs::_50 => 5 ,
        AlsIntegrationTimeMs::_100 => 10,
        AlsIntegrationTimeMs::_150 => 15,
        AlsIntegrationTimeMs::_200 => 20,
        AlsIntegrationTimeMs::_250 => 25,
        AlsIntegrationTimeMs::_300 => 30,
        AlsIntegrationTimeMs::_350 => 35,
        AlsIntegrationTimeMs::_400 => 40,
    };

    // Multiply the top by 1O ^ 5 to get an integer result
    // Set the ratio to 0 if (ch0 + ch1) == 0
    let ratio = (ch1 * 10_i32.pow(5)).checked_div(ch0 + ch1).unwrap_or_default();
    // Accordingly multiply by 10 ^5 the values used for comparisons
    // ratio  < 0.45
    let als_lux = if ratio < 45 * 10_i32.pow(3) {
        // (1.7743 * 10 ^ 4 * ch0 + 1.1059 * 10 ^ 4 * ch1) / (als_int * 10) / als_gain = als_lux * 10 ^ 3
        (17_743 * ch0 + 11_059 * ch1) / als_int / als_gain
    // 0.45 <= ratio < 0.64
    } else if ratio < 64 * 10_i32.pow(3) {
        // (4.2785 * 10 ^ 4 * ch0 - 1.9584 * 10 ^ 4 * ch1) / (als_int * 10) / als_gain = als_lux * 10 ^ 3
        (42_785 * ch0 - 19_548 * ch1) / als_int / als_gain
    // 0.64 <= ratio < 0.85
    } else if ratio < 85 * 10_i32.pow(3) {
        // (0.5926 * 10 ^ 4 * ch0 + 0.1185 * 10 ^ 4 * ch1) / (als_int * 10) / als_gain = als_lux * 10 ^ 3
        (5_926 * ch0 + 1_185 * ch1) / als_int / als_gain
    } else {
        0
    };

    // Divide the final result by 10 to keep only 2 significative digits
    let lux_reading = als_lux / 10;

    Sample::new(lux_reading, SampleMetadata::UnknownAccuracy)
}
