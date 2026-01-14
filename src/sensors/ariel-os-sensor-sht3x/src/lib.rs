//! Driver for the Sensiron SHT3X series of humidity and temperature sensors.
//!
//! Compatible with [`ariel_os_sensors::Sensor`].
//!
//! [Datasheet]: https://sensirion.com/media/documents/213E6A3B/63A5A569/Datasheet_SHT3x_DIS.pdf
//! [Alert Datasheet]: https://sensirion.com/media/documents/40D749F7/65D61534/HT_AN_AlertMode.pdf

// In the absence of clear link to the datasheet, timing informations are sourced from the Tables 4 and 5
#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

pub mod i2c;

use ariel_os_sensors::sensor::SampleMetadata;

use crc::{Crc, CRC_8_NRSC_5};

#[expect(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
/// Commands that can be sent to the sensor
enum Command {
    /// Soft Reset the sensor, 1.5ms delay max before entering idle state
    SoftReset = 0x30A2,

    /// Read the status register
    ReadStatusReg = 0xF32D,
    /// Clears the status registers bits
    /// 15 (Alert pending),
    /// 11 (RH alert),
    /// 10 (TH alert)
    /// and 4 (system reset)
    ClearStatusReg = 0x3041,
    /// Enables the onboard heater
    HeaterEnable = 0x306D,
    /// Disables the onboard heater
    HeaterDisable = 0x3066,
    /// In periodic mode, this is used to fetch the latest data.
    /// Completing this transaction clears the measurement data
    /// and subsequent [`FetchData`] will need to wait for the next
    /// measurements. See Tables 4 and 5 of the datasheet for the apprioriate wait times.
    FetchData = 0xE000,
    /// The ART (Accelerated Response Time) will enable
    /// acquisition with a frequency of 4Hz. The rest of the
    /// acquisition procedure is the same as regular periodic execution
    PeriodicMeasurementWithART = 0x2B32,
    /// Break / Stop periodic acquisition mode
    /// This should be used for stopping periodic data acquisition
    /// before issuing any command other than [`FetchData`]
    Break = 0x3093,

    /// Single Shot acquisition With clock strectching and High Repeatability
    SingleShotEnabledHigh = 0x2C06,
    /// Single Shot acquisition With clock strectching and Medium Repeatability
    SingleShotEnabledMedium = 0x2C0D,
    /// Single Shot acquisition With clock strectching and Low Repeatability
    SingleShotEnabledLow = 0x2C10,

    /// Single Shot acquisition Without clock strectching and High Repeatability
    SingleShotDisabledHigh = 0x2400,
    /// Single Shot acquisition Without clock strectching and Medium Repeatability
    SingleShotDisabledMedium = 0x240B,
    /// Single Shot acquisition Without clock strectching and Low Repeatability
    SingleShotDisabledLow = 0x2416,

    /// Periodic Acquisition with 0.5 MPS and High Repeatability
    Periodic1MP2SHigh = 0x2032,
    /// Periodic Acquisition with 0.5 MPS and Medium Repeatability
    Periodic1MP2SMedium = 0x2024,

    /// Periodic Acquisition with 0.5 MPS and Low Repeatability
    Periodic1MP2SLow = 0x202F,

    /// Periodic Acquisition with 1 MPS and High Repeatability
    Periodic1MPSHigh = 0x2136,
    /// Periodic Acquisition with 1 MPS and Medium Repeatability
    Periodic1MPSMedium = 0x2126,
    /// Periodic Acquisition with 1 MPS and Low Repeatability
    Periodic1MPSLow = 0x212D,

    /// Periodic Acquisition with 2 MPS and High Repeatability
    Periodic2MPSHigh = 0x2236,
    /// Periodic Acquisition with 2 MPS and Medium Repeatability
    Periodic2MPSMedium = 0x2220,
    /// Periodic Acquisition with 2 MPS and Low Repeatability
    Periodic2MPSHLow = 0x222B,

    /// Periodic Acquisition with 4 MPS and High Repeatability
    Periodic4MPSHigh = 0x2334,
    /// Periodic Acquisition with 4 MPS and Medium Repeatability
    Periodic4MPSMedium = 0x2322,
    /// Periodic Acquisition with 4 MPS and Low Repeatability
    Periodic4MPSLow = 0x2329,

    /// Periodic Acquisition with 10 MPS and High Repeatability
    Periodic10MPSHigh = 0x2737,
    /// Periodic Acquisition with 10 MPS and Medium Repeatability
    Periodic10MPSMedium = 0x2721,
    /// Periodic Acquisition with 10 MPS and Low Repeatability
    Periodic10MPSLow = 0x272A,

    /// Reads the High Alert Set Limit
    ReadHighAlertSet = 0xE11F,
    /// Reads the High Alert Clear Limit
    ReadHighAlertClear = 0xE114,
    /// Reads the High Alert Set Limit
    ReadLowAlertSet = 0xE109,
    /// Reads the High Alert Clear Limit
    ReadLowAlertClear = 0xE102,

    /// Writes the High Alert Set Limit
    WriteHighAlertSet = 0x611D,
    /// Writes the High Alert Clear Limit
    WriteHighAlertClear = 0x6116,
    /// Writes the Low Alert Set Limit
    WriteLowAlertSet = 0x610B,
    /// Writes the Low Alert Clear Limit
    WriteLowAlertClear = 0x6100,
}


// STATUS register bits
/// Set to 1 if the checksum of the last write failed
pub const CHECKSUM_STATUS: u16 = 1 << 0;

/// Set to 1 if the last command was not processed
pub const COMMAND_STATUS: u16 = 1 << 1;

/// Set to 1 if a reset was detected since the last Status register clear
pub const RESET_DETECTED: u16 = 1 << 4;

/// Set to 1 in the event of a Temperature tracking alert
pub const TEMP_TRACKING_ALERT: u16 = 1 << 10;

/// Set to 1 in the event of a Relative Humidity tracking alert
pub const RH_TRACKING_ALERT: u16 = 1 << 11;

/// Set to 1 if the heater is on
pub const HEATER_STATUS: u16 = 1 << 13;

/// Set to 1 if an alert is pending
pub const ALERT_PENDING: u16 = 1 << 15;


// Maximal wait times before acquisition when powered with 2.4 < V < 5.5
// See Tabl 4 of the Datasheet
const HIGH_REPEAT_DELAY: u64 = 15;
const MEDIUM_REPEAT_DELAY: u64 = 6;
const LOW_REPEAT_DELAY: u64 = 4;



// FIXME: Take into account exact sensor model
// This assumes a SHT31
fn t_accuracy(temp: i32) -> SampleMetadata {
    // See Table 2 and Figures 8 to 10  of the datasheet.
    // Accuracy of 0.4 °C between -40 °C and +90 °C.
    if -4000 < temp && temp < 9000 {
        return SampleMetadata::SymmetricalError {
            deviation: 40,
            bias: 0,
            scaling: -2,
        };
    }

    // Accuracy of 0.75 °C otherwise.
    SampleMetadata::SymmetricalError {
        deviation: 75,
        bias: 0,
        scaling: -2,
    }
}


// This assumes a SHT31
fn h_accuracy(humi: i32) -> SampleMetadata {
    // See Table 1 and Figures 2 to 7  of the datasheet.
    // Accuracy of 2.5 %RH between 0 %RH and 90 %RH.
    if humi < 9000 {
        return SampleMetadata::SymmetricalError {
            deviation: 250,
            bias: 0,
            scaling: -2,
        };
    }

    // Accuracy of 3.5 %RH otherwise.
    SampleMetadata::SymmetricalError {
        deviation: 35,
        bias: 0,
        scaling: -1,
    }
}

/// Calculate the CRC of a 2 byte value
/// The algorithm is described in table 20 of the datasheet
fn calculate_crc(data: &[u8; 2]) -> u8 {
    let crc = Crc::<u8>::new(&CRC_8_NRSC_5);
    let mut digest = crc.digest();
    digest.update(data);
    digest.finalize()
}

/// Checks that the data leads to the expected crc
fn check_crc(data: &[u8; 2], expected_crc: u8) -> bool {
    calculate_crc(data) == expected_crc
}