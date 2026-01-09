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

fn i32_from_i24_be_bytes(bytes: [u8; 3]) -> i32 {
    // Branch-less sign extension from <https://stackoverflow.com/a/42536138>.
    const M: i32 = 1 << (24 - 1);
    (i32::from_be_bytes([0, bytes[0], bytes[1], bytes[2]]) ^ M).wrapping_sub(M)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_i32_from_i24_be_bytes() {
        assert_eq!(i32_from_i24_be_bytes([0x00, 0x00, 0x01]), 1);
        assert_eq!(i32_from_i24_be_bytes([0x7f, 0xff, 0xff]), 0x7f_ff_ff);
        assert_eq!(i32_from_i24_be_bytes([0x80, 0x00, 0x00]), !0x80_00_00 + 1);
        assert_eq!(i32_from_i24_be_bytes([0xff, 0xff, 0xff]), -1);
    }
}
