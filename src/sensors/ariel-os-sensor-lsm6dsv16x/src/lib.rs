//! Driver for the LSM6DSV16X 6-axis IMU.
//!
//! Compatible with [`ariel_os_sensors::Sensor`].

#![no_std]
#![deny(missing_docs)]

pub mod i2c;

use ariel_os_sensors::sensor::SampleMetadata;

const PART_NUMBER: &str = "LSM6DSV16X";

#[expect(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Register {
    WhoAmI = 0x0f,
    Ctrl1 = 0x10,
    Ctrl2 = 0x11,
    Ctrl3 = 0x12,
    Ctrl4 = 0x13,
    Ctrl5 = 0x14,
    Ctrl6 = 0x15,
    Ctrl7 = 0x16,
    Ctrl8 = 0x17,
    Ctrl9 = 0x18,
    Ctrl10 = 0x19,
    StatusReg = 0x1e,
    OutxLG = 0x22,
    OutxLA = 0x28,
}

#[expect(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum AccelMode {
    HighPerformance = 0x0 << 4,
    HighAccuaryOdr = 0x1 << 4,
    // 0x02 is reserved.
    OdrTriggered = 0x3 << 4,
    LowPower1 = 0x4 << 4,
    LowPower2 = 0x5 << 4,
    LowPower3 = 0x6 << 4,
    Normal = 0x7 << 4,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum AccelOdr {
    PowerDown = 0x0,
    _1_875Hz = 0x1,
    _7_5Hz = 0x2,
    _15Hz = 0x3,
    _30Hz = 0x4,
    _60Hz = 0x5,
    _120Hz = 0x6,
    _240Hz = 0x7,
    _480Hz = 0x8,
    _960Hz = 0x9,
    _1_92kHz = 0xa,
    _3_84kHz = 0xb,
    _7_68kHz = 0xc,
}

// Table 68 of the datasheet.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
enum AccelFullScale {
    #[default]
    _2g = 0x0,
    _4g = 0x1,
    _8g = 0x2,
    _16g = 0x3,
}

impl AccelFullScale {
    fn as_sensitivity_thousandths(self) -> i16 {
        // Table 3 of the datasheet.
        match self {
            Self::_2g => 61,
            Self::_4g => 122,
            Self::_8g => 244,
            Self::_16g => 488,
        }
    }
}

#[expect(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum GyroMode {
    HighPerformance = 0x0 << 4,
    HighAccuaryOdr = 0x1 << 4,
    // 0x02 is reserved.
    OdrTriggered = 0x3 << 4,
    Sleep = 0x4 << 4,
    LowPower = 0x5 << 4,
    Normal = 0x7 << 4,
}

#[derive(Copy, Clone, PartialEq, Eq)]
enum GyroOdr {
    PowerDown = 0x0,
    // 0x01 is skipped.
    _7_5Hz = 0x2,
    _15Hz = 0x3,
    _30Hz = 0x4,
    _60Hz = 0x5,
    _120Hz = 0x6,
    _240Hz = 0x7,
    _480Hz = 0x8,
    _960Hz = 0x9,
    _1_92kHz = 0xa,
    _3_84kHz = 0xb,
    _7_68kHz = 0xc,
}

// Table 63 of the datasheet.
#[derive(Copy, Clone, PartialEq, Eq, Default)]
enum GyroFullScale {
    #[default]
    _125dps = 0x0,
    _250dps = 0x1,
    _500dps = 0x2,
    _1000dps = 0x3,
    _2000dps = 0x4,
    // Some values are skipped.
    _4000dps = 0xc,
}

impl GyroFullScale {
    // Using `i16` as return type to make sure this cannot overflow when multiplying by an `i16`
    // inside a `i32`.
    fn as_sensitivity_hundredths(self) -> i16 {
        // Table 3 of the datasheet.
        match self {
            Self::_125dps => 438, // Rounded.
            Self::_250dps => 875,
            Self::_500dps => 1750,
            Self::_1000dps => 3500,
            Self::_2000dps => 7000,
            Self::_4000dps => 14_000,
        }
    }
}

// Gyroscope new data available.
const GDA_BITS: u8 = 1 << 1;
// Accelerometer new data available.
const XLDA_BITS: u8 = 1 << 0;

// Software reset.
const SW_RESET_BITS: u8 = 1 << 0;

#[expect(dead_code)]
const DEVICE_ID: u8 = 0x70;

fn accel_accuracy() -> SampleMetadata {
    // `LA_TyOff` from Table 3 of the datasheet.
    SampleMetadata::SymmetricalError {
        deviation: 12, // TODO: this could possibly be refined by taking into account `An` as well.
        bias: 0,
        scaling: -3,
    }
}

fn gyro_accuracy() -> SampleMetadata {
    // `G_TyOff` from Table 3 of the datasheet.
    SampleMetadata::SymmetricalError {
        deviation: 1, // TODO: this could possibly be refined by taking into account `Rn` as well.
        bias: 0,
        scaling: 0,
    }
}
