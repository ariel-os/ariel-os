//! Driver for the STTS22H temperature sensor.
//!
//! Compatible with [`ariel_os_sensors::Sensor`].

#![no_std]
#![deny(missing_docs)]

pub mod i2c;

use ariel_os_sensors::sensor::SampleMetadata;

const PART_NUMBER: &str = "STTS22H";

#[expect(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Register {
    WhoAmIRegAddr = 0x01,
    TempHLimit = 0x02,
    TempLLimit = 0x03,
    CtrlRegAddr = 0x04,
    StatusRegAddr = 0x05,
    TempLOutRegAddr = 0x06,
    TempHOutRegAddr = 0x07,
}

// CTRL register bits.
const ONE_SHOT_BITS: u8 = 1 << 0;
const IF_ADD_INC_BITS: u8 = 1 << 3;
const BDU_BITS: u8 = 1 << 6;

// STATUS register bits.
const BUSY_BITS: u8 = 1 << 0;

#[expect(dead_code)]
const DEVICE_ID: u8 = 0xa0;

fn accuracy(temp: i32) -> SampleMetadata {
    // FIXME: Figure 2 of the datasheet is unclear
    // Accuracy of 0.5 °C between -5 °C and +55 °C
    if -500 < temp && temp < 5500 {
        return SampleMetadata::SymmetricalError {
            deviation: 50,
            bias: 0,
            scaling: -2,
        };
    }

    // Accuracy of 1.0 °C otherwise
    return SampleMetadata::SymmetricalError {
        deviation: 100,
        bias: 0,
        scaling: -2,
    };
}
