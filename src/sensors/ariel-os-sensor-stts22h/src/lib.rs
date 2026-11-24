//! Driver for the STMicroelectronics [STTS22H] temperature sensor.
//!
//! Compatible with [`ariel_os_sensors::Sensor`].
//!
//! [STTS22H]: https://www.st.com/en/mems-and-sensors/stts22h.html

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

pub mod i2c;

use ariel_os_sensors::sensor::SampleMetadata;
use ariel_os_sensors_utils::u8_register;

const PART_NUMBER: &str = "STTS22H";

#[expect(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Register {
    Whoami = 0x01,
    TempHLimit,
    TempLLimit,
    Ctrl,
    Status,
    TempLOut,
    TempHOut,
}

u8_register!(TempHLimit {});

u8_register!(Ctrl {
    BDU << 6,
    IF_ADD_INC << 3,
    ONE_SHOT << 0,
});

u8_register!(Status {
    BUSY << 0,
});

u8_register!(TempLOut {});

#[expect(dead_code)]
const DEVICE_ID: u8 = 0xa0;

fn accuracy(temp: i32) -> SampleMetadata {
    // See Table 3 and Figure 2 of the datasheet.
    // Accuracy of 0.5 °C between -10 °C and +60 °C.
    if -1000 < temp && temp < 6000 {
        return SampleMetadata::SymmetricalError {
            deviation: 50,
            bias: 0,
            scaling: -2,
        };
    }

    // Accuracy of 1.0 °C otherwise.
    SampleMetadata::SymmetricalError {
        deviation: 100,
        bias: 0,
        scaling: -2,
    }
}
