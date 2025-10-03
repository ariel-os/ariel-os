//! Driver for the STMicroelectronics [LPS22DF] temperature sensor.
//!
//! Compatible with [`ariel_os_sensors::Sensor`].
//!
//! [LPS22DF]: https://www.st.com/en/mems-and-sensors/lps22df.html

#![cfg_attr(not(test), no_std)]
#![deny(missing_docs)]

pub mod i2c;

use ariel_os_sensors::sensor::SampleMetadata;

const PART_NUMBER: &str = "LPS22DF";

#[expect(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Register {
    WhoAmI = 0x0f,
    CtrlReg2 = 0x11,
    RpdsL = 0x1a,
    Status = 0x27,
    PressOutXl = 0x28,
}

// `CTRL_REG2` register bits.
const ONESHOT_BITS: u8 = 1 << 0;
const SWRESET_BITS: u8 = 1 << 2;
const BDU_BITS: u8 = 1 << 3;

// `STATUS` register bits.
const P_DA_BITS: u8 = 1 << 0;
const T_DA_BITS: u8 = 1 << 1;

#[expect(dead_code)]
const DEVICE_ID: u8 = 0xb4;

// Table 2 of the datasheet.
const PRESSURE_SENSITIVITY: i32 = 4096;
const TEMP_SENSITIVITY: i32 = 100;

fn pressure_accuracy(_pressure: i32) -> SampleMetadata {
    // Takes into account `PAccT` + `P_drift` and a rough upper bound of the temperature offset
    // from Table 2 of the datasheet.
    // This could be refined if needed.
    SampleMetadata::SymmetricalError {
        deviation: 150, // Pa
        bias: 0,
        scaling: 0,
    }
}

fn temp_accuracy(_temp: i32) -> SampleMetadata {
    // `Tacc` from Table 2 of the datasheet.
    SampleMetadata::SymmetricalError {
        deviation: 150,
        bias: 0,
        scaling: -2,
    }
}
