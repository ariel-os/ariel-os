//! Driver for the ASAIR AHT20 temperature and humidity sensor.
//!
//! Compatible with [`ariel_os_sensors::Sensor`].
//!
//! [ASAIR AHT20]: https://files.seeedstudio.com/wiki/Grove-AHT20_I2C_Industrial_Grade_Temperature_and_Humidity_Sensor/AHT20-datasheet-2020-4-16.pdf
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

pub mod i2c;

use ariel_os_sensors::sensor::SampleMetadata;

use crc::{CRC_8_NRSC_5, Crc};

#[derive(Copy, Clone, PartialEq, Eq)]
/// Commands that can be sent to the sensor. Described in Table 9 and in section 5.4.
enum Command {
    /// Initialize the sensor
    /// In particular this set the calibration bit.
    Initialize = 0xBE,

    /// Soft reset the sensor.
    SoftReset = 0xBA,

    /// Trigger a measurement.
    /// This requires hardcoded arguments.
    TriggerMeasurement = 0xAC,

    /// Read the status byte.
    /// This command has unclear side effects on the CRC
    /// when used after a measurement has been triggered.
    /// Issuing a general Read command and reading a single byte
    /// has the same effect and thus should be preferred.
    ReadStatusReg = 0x71,
}

// STATUS register bits, see Table 10 of the
/// Set to 1 if the sensor is calibrated.
pub const CALIBRATION_STATUS: u8 = 1 << 3;

/// Set to 1 if the sensor is busy.
pub const BUSY_BIT: u8 = 1 << 7;

/// Magic argument to be sent with the [`Command::TriggerMeasurement`].
/// Described in Section 5.3.
pub const MEASUREMENT_ARG_0: u8 = 0x33;
/// Magic argument to be sent with the [`Command::TriggerMeasurement`].
/// Described in Section 5.3.
pub const MEASUREMENT_ARG_1: u8 = 0x00;

/// Magic argument to be sent with the [`Command::Initialize`].
/// Described in Section 5.4 of v1.0 of the Datasheet.
pub const INITIALIZE_ARG_0: u8 = 0x08;
/// Magic argument to be sent with the [`Command::Initialize`].
/// Described in Section 5.4 of v1.0 of the Datasheet.
pub const INITIALIZE_ARG_1: u8 = 0x00;

const PART_NUMBER: &str = "AHT20";

fn t_accuracy(temp: i32) -> SampleMetadata {
    // See Table 3 and Figure 3 of the datasheet.
    // Accuracy of 0.3 °C between around 20 °C and 60 °C.
    if 200 < temp && temp < 600 {
        SampleMetadata::SymmetricalError {
            deviation: 3,
            bias: 0,
            scaling: -1,
        }
    }
    // Accuracy of 1.5 °C between -40 °C and 20 °C.
    else if -400 < temp && temp < 200 {
        SampleMetadata::SymmetricalError {
            deviation: 15,
            bias: 0,
            scaling: -1,
        }
    }
    // Accuracy of 2 °C between 60 °C and 85 °C.
    else if 200 < temp && temp < 850 {
        SampleMetadata::SymmetricalError {
            deviation: 20,
            bias: 0,
            scaling: -1,
        }
    }
    // Otherwise we are outstide of the scope of work of the sensor
    else {
        unreachable!("We are outside of the working conditions of this sensor");
    }
}

fn h_accuracy(humi: i32) -> SampleMetadata {
    // See Table 1 and Figure 2 of the datasheet.
    // Accuracy of 2 %RH between 20 %RH and 80 %RH.
    if 20 < humi && humi < 80 {
        SampleMetadata::SymmetricalError {
            deviation: 2,
            bias: 0,
            scaling: 0,
        }
    }
    // Accuracy of 5 %RH otherwise.
    else {
        SampleMetadata::SymmetricalError {
            deviation: 5,
            bias: 0,
            scaling: 0,
        }
    }
}

/// Calculate the CRC of a 5 byte value.
/// The algorithm is described in section 5.4.4
/// The important information are:
/// - the polynomial: X^8 + X^5 + X^4 + 1,
/// - and the initial value: 0xFF,
///
/// Usual CRC notation doesn't take into account the most significant
/// coefficient and hence here it would be `0b0011_0001` = 0x31.
fn calculate_crc(data: &[u8]) -> u8 {
    let crc = Crc::<u8>::new(&CRC_8_NRSC_5);
    let mut digest = crc.digest();
    digest.update(data);
    digest.finalize()
}

/// Checks that the data leads to the expected crc.
fn check_crc(data: &[u8], expected_crc: u8) -> bool {
    calculate_crc(data) == expected_crc
}
