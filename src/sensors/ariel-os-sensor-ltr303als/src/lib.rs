//! Driver for the [LTR-303ALS-01] digital ambiant light sensor.
//!
//! Compatible with [`ariel_os_sensors::Sensor`]
//!
//! [LTR-303ALS-01]: ... TODO
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

pub mod i2c;

use ariel_os_sensors::sensor::{Sample, SampleMetadata};

const PART_NUMBER: &str = "LTR-303ALS-01";

pub(crate) const INITIAL_STARTUP_TIME_MS: u64 = 100;
pub(crate) const WAKEUP_TIME_MS: u64 = 40;

pub enum Register {
    GainCtrl = 0x80,
    MeasureCtrl = 0x85,
    // Basically WhoAmI 1
    PartId = 0x86,
    // Basically WhoAmI 2
    ManufacturerId = 0x87,

    // Should be read in pair
    // and before channel 0 data
    Channel1Low = 0x88,
    Channel1High = 0x89,
    Channel0Low = 0x8A,
    Channel0High = 0x8B,

    StatusReg = 0x8C,

    InterruptReg = 0x8F,

    UpperThresholdLow = 0x97,
    UpperThresholdHigh = 0x98,
    LowerThresholdLow = 0x99,
    LowerThresholdHigh = 0x9A,

    InterruptPersistReg = 0x9E,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlsGain {
    /// 1 Lux -> 64k Lux
    #[default]
    _1 = 0b000 << 2,
    /// 0.5 Lux -> 32k Lux
    _2 = 0b001 << 2,
    /// 0.25 Lux -> 16k Lux
    _4 = 0b010 << 2,
    /// 0.125 Lux -> 8k Lux
    _8 = 0b011 << 2,
    /// 0.02 Lux -> 1.3k Lux
    _48 = 0b110 << 2,
    /// 0.01 Lux -> 600 Lux
    _96 = 0b111 << 2,
}

/// Set to 1 to initiate software reset procedure.
const SOFT_RESET: u8 = 1 << 1;
/// Set to 1 to activate the sensor
const ALS_MODE: u8 = 1 << 0;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlsIntegrationTimeMs {
    #[default]
    _100 = 0b000 << 3,
    _50 = 0b001 << 3,
    _200 = 0b010 << 3,
    _400 = 0b011 << 3,
    _150 = 0b100 << 3,
    _250 = 0b101 << 3,
    _300 = 0b110 << 3,
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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlsMeasurementRateMs {
    #[default]
    _500 = 0b011,
    _50 = 0b000,
    _100 = 0b001,
    _200 = 0b010,
    _1000 = 0b100,
    _2000 = 0b101,
}

const PART_ID: u8 = 0xA << 4;
const REV_ID: u8 = 0x0;
const MANU_ID: u8 = 0x05;

// This bit is set to 1 when the data is invalid
const ALS_DATA_VALIDITY: u8 = 1 << 7;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum MeasuredAlsDataGain {
    /// Data measured with x1 gain (default)
    _1 = 0b000 << 4,
    /// Data measured with x2 gain
    _2 = 0b001 << 4,
    /// Data measured with x4 gain
    _4 = 0b010 << 4,
    /// Data measured with x8 gain
    _8 = 0b011 << 4,
    /// Data measured with x48 gain
    _48 = 0b110 << 4,
    /// Data measured with x96 gain
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
/// This bit is set to 1 when an interrupt signal is active
const ALS_INTERRUPT_STATUS: u8 = 1 << 3;

/// This bit is set to 1 to indicate new data in the data registers
const ALS_DATA_STATUS: u8 = 1 << 2;

/// Describes and defines when the INT pin is considered active
/// - 0: when it's a logical 0
/// - 1: when it's a logical 1
const INTERRUPT_POLARITY: u8 = 1 << 2;

/// Set to 1 to allow ALS measurements to trigger interrupts
const INTERRUPT_MODE: u8 = 1 << 1;

/// Control the number of times that the ALS measurements must be outside of
/// the threshold ranges before triggering an interrupt on the INT pin
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum AlsPersist {
    /// Every ALS measurement outside of the threshold range
    /// can trigger an interrupt
    #[default]
    _1 = 0b0000,
    /// Two consecutive ALS measurements outside of the threshold range
    /// are needed.
    _2 = 0b0001,
    /// Sixteen consecutive ALS measurements outside of the threshold range
    /// are needed.
    _16 = 0b1111,
}

fn physical_lux_from_raw(
    channel1: u16,
    channel0: u16,
    als_gain: MeasuredAlsDataGain,
    als_integration_time: AlsIntegrationTimeMs,
) -> Sample {
    // TODO: find a way to remove floats I guess ?
    // According to appendix A
    // ratio =  ch1/(ch0 + ch1)
    let ch1 = f32::from(ch1);
    let ch0 = f32::from(ch0);
    let als_gain = match used_gain {
        MeasuredAlsDataGain::_1 => 1_f32,
        MeasuredAlsDataGain::_2 => 2_f32,
        MeasuredAlsDataGain::_4 => 4_f32,
        MeasuredAlsDataGain::_8 => 8_f32,
        MeasuredAlsDataGain::_48 => 48_f32,
        MeasuredAlsDataGain::_96 => 96_f32,
    };
    let als_int = match used_int_time {
        AlsIntegrationTimeMs::_50 => 0.5_f32,
        AlsIntegrationTimeMs::_100 => 1_f32,
        AlsIntegrationTimeMs::_150 => 1.5_f32,
        AlsIntegrationTimeMs::_200 => 2_f32,
        AlsIntegrationTimeMs::_250 => 2.5_f32,
        AlsIntegrationTimeMs::_300 => 3_f32,
        AlsIntegrationTimeMs::_350 => 3.5_f32,
        AlsIntegrationTimeMs::_400 => 4_f32,
    };

    let ratio = ch1 / (ch0 + ch1);
    let als_lux = if ratio < 0.45 {
        (1.7743 * ch0 + 1.1059 * ch1) / als_int / als_gain
    } else if ratio < 0.64 {
        (4.2785 * ch0 - 1.9548 * ch1) / als_int / als_gain
    } else if ratio < 0.85 {
        (0.5926 * ch0 + 0.1185 * ch1) / als_int / als_gain
    } else {
        0_f32
    };

    let lux_reading = (als_lux * 100.0) as i32;

    Sample::new(lux_reading, SampleMetadata::NoMeasurementError)
}
