//! Driver for the STMicroelectronics [LIS2MDL] ultralow-power, high-performance 3-axis
//! magnetometer.
//!
//! Compatible with [`ariel_os_sensors::Sensor`].
//!
//! [LIS2MDL]: https://www.st.com/en/mems-and-sensors/Lis2mdl.html

#![no_std]
#![deny(missing_docs)]

pub mod i2c;

use ariel_os_sensors::sensor::SampleMetadata;

const PART_NUMBER: &str = "LIS2MDL";

#[expect(dead_code)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Register {
    WhoAmI = 0x4f,
    CfgRegA = 0x60,
    CfgRegC = 0x62,
    StatusReg = 0x67,
    OutxLReg = 0x68,
}

// This device has only one I2C address.
const TARGET_I2C_ADDR: u8 = 0b001_1110;

// `CFG_REG_A` register bits.
const SOFT_RST_BITS: u8 = 1 << 5;
const COMP_TEMP_EN_BITS: u8 = 1 << 7;

// Table 25 of the datasheet, includes bit shift for `CFG_REG_A`.
#[expect(clippy::enum_variant_names, reason = "matches the datasheet")]
#[expect(unused)]
#[derive(Copy, Clone, PartialEq, Eq)]
enum Md {
    ContinuousMode = 0b00,
    SingleMode = 0b01,
    // There is an intermediate value, but it is unclear how it is different from `IdleMode`.
    IdleMode = 0b11,
}

// `CFG_REG_C` register bits.
const BDU_BITS: u8 = 1 << 4;

// `STATUS_REG` register bits.
const ZYXDA_BITS: u8 = 1 << 3;

#[expect(dead_code)]
const DEVICE_ID: u8 = 0x40;

fn accuracy() -> SampleMetadata {
    // TODO: could be refined by taking `TCSo` and `TCOff` into account.

    // Table 2 of the datasheet.
    let ty_off = 60;
    // Table 9 of AN5069 (taking highest value to not overestimate accuracy).
    let rms_noise = 9;

    SampleMetadata::SymmetricalError {
        deviation: ty_off + rms_noise,
        bias: 0,
        scaling: -7, // milligauss
    }
}
