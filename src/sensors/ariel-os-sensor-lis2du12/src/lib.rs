//! Driver for the STMicroelectronics [LIS2DU12] ultralow-power 3-axis accelerometer.
//!
//! Compatible with [`ariel_os_sensors::Sensor`].
//!
//! [LIS2DU12]: https://www.st.com/en/mems-and-sensors/lis2du12.html

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

pub mod i2c;

use ariel_os_sensors::sensor::SampleMetadata;
use ariel_os_sensors_utils::u8_register;

const PART_NUMBER: &str = "LIS2DU12";

#[expect(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Register {
    Ctrl1 = 0x10,
    Ctrl2,
    Ctrl3,
    Ctrl4,
    Ctrl5,
    Status = 0x25,
    OutXL = 0x28,
    WhoAmI = 0x43,
}

// Table 37 of the datasheet.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
enum AccelFullScale {
    #[default]
    _2g,
    _4g,
    _8g,
    _16g,
}

impl AccelFullScale {
    fn to_microg_from_lsb(self, lsb: i16) -> i32 {
        // Table 2 of the datasheet.
        let sensitivity = match self {
            Self::_2g => 976,
            Self::_4g => 1952,
            Self::_8g => 3904,
            Self::_16g => 7808,
        };

        i32::from(lsb >> 4) * sensitivity
    }
}

// Table 34 of the datasheet.
#[expect(unused)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Odr {
    PowerDown,
    _1_6HzUltraLpMode,
    _3HzUltraLpMode,
    _6HzUltraLpMode,
    _6HzNormalMode,
    _12_5HzNormalMode,
    _25HzNormalMode,
    _50HzNormalMode,
    _100HzNormalMode,
    _200HzNormalMode,
    _400HzNormalMode,
    _800HzNormalMode,
    // Some values are skipped.
    OneShotInt2Pin = 0xe,
    OneShotInterface,
}

u8_register!(Ctrl1 {
    SW_RESET << 5,
    IF_ADD_INC << 4,
});

u8_register!(Ctrl4 {
    BDU << 5,
    SOC << 1,
});

u8_register!(Ctrl5 {
    [Odr] << 4,
});

u8_register!(Status {
    DRDY << 0,
});

u8_register!(OutXL {});

#[expect(dead_code)]
const DEVICE_ID: u8 = 0b0100_0101;

fn accel_accuracy() -> SampleMetadata {
    // `TyOff` from Table 2 of the datasheet.
    SampleMetadata::SymmetricalError {
        deviation: 11, // TODO: this could possibly be refined by taking into account `An` as well.
        bias: 0,
        scaling: -3,
    }
}
