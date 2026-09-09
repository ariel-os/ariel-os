//! Board specific initialization.

/// Initialize the power management components necessary for some of the hardware to function properly.
///
/// - `i2c_bus` needs to be a mutable reference to an i2c controller with SCL connected to P0_08 and SDA connected to P0_09.
/// - Set `sensors` to true to power the sensors (VDD_SENS on the schematic).
/// - Set `npm6001` to power the nPM6001 PMIC used for the wifi chip (nPM60_ENABLE on the schematic)
#[cfg(all(feature = "i2c", context = "nordic-thingy-91-x-nrf9151"))]
pub async fn init_thingy91x_board(
    i2c_bus: &mut crate::i2c::controller::I2c,
    npm6001: bool,
    sensors: bool,
) -> Result<(), ariel_os_embassy_common::i2c::controller::Error> {
    use embedded_hal_async::i2c::I2c;
    const NPM1300_ADDR: u8 = 0x6b;

    // Register addresses are on 16 bits, i2c works with 8 bits words so we split them.

    // Load switch 1, set value to 0x01 to power the wifi chip.
    const TASKLDSW1SET: [u8; 2] = [0x08, 0x00];
    let taskldsw1set_value: u8 = if npm6001 { 0x01 } else { 0x00 };
    // Load switch 2, set value to 0x01 to give power to the sensors.
    const TASKLDSW2SET: [u8; 2] = [0x08, 0x02];
    let taskldsw2set_value: u8 = if sensors { 0x01 } else { 0x00 };

    // Buck 2 enabled by GPIO2.
    const BUCKENCTRL: [u8; 2] = [0x04, 0x0c];
    const BUCKENCTRL_VALUE: u8 = 0x18;

    // Battery spec: https://www.lipobatteries.net/wp-content/uploads/2021/12/LP803448-LPCWES.pdf

    // Set charging rate to 675mA (from battery spec) and enable charging. This is a 10 bit value but we can't set the last bit.
    const CHARGING_RATE: u16 = 675;
    const BCHGISETMSB: [u8; 2] = [0x03, 0x08];
    const BCHGISETMSB_VALUE: u8 = (CHARGING_RATE >> 2) as u8;
    const BCHGISETLSB: [u8; 2] = [0x03, 0x09];
    const BCHGISETLSB_VALUE: u8 = (CHARGING_RATE >> 1) as u8 & 0x01;
    // Charging must be enabled after setting the charge rate.
    const BCHGENABLESET: [u8; 2] = [0x03, 0x04];
    const BCHGENABLESET_ENABLE_VALUE: u8 = 0x01;
    const BCHGENABLESET_DISABLE_VALUE: u8 = 0x00;

    // 1A current discharge limit. Battery limit is 1350mA. IC can either have 200mA or 1A limit, we select 1A.
    const BCHGISETDISCHARGEMSB: [u8; 2] = [0x03, 0x0a];
    const BCHGISETDISCHARGEMSB_VALUE: u8 = 0xcf;
    const BCHGISETDISCHARGELSB: [u8; 2] = [0x03, 0x0b];
    const BCHGISETDISCHARGELSB_VALUE: u8 = 0x01;

    // 4.2V (Max. Charge Voltage in battery the spec) termination voltage for cool and nominal temperature region.
    const BCHGVTERM: [u8; 2] = [0x03, 0x0c];
    const BCHGVTERM_VALUE: u8 = 0x08;
    // 4.2V (Max. Charge Voltage in the battery spec) termination voltage for warm temperature region.
    const BCHGVTERMR: [u8; 2] = [0x03, 0x0d];
    const BCHGVTERMR_VALUE: u8 = 0x08;

    // Thermistor in the battery? (should be the same specs): https://amwei.com/ntc-10k-ohm-beta-3435k-axial-lead-glass-seal
    // NTC 10 kOhm
    const ADCNTCRSEL: [u8; 2] = [0x05, 0x0a];
    const ADCNTCRSEL_VALUE: u8 = 0x01;

    // Using the table in the NTC spec for resistance values.
    // The default temperature values of the IC should be fine except the COLD threshold, we need to raise it to 10C.
    const INTERNAL_BIAS_RESISTOR: u32 = 10000; // 10kOhms
    const RESISTANCE_10C: u32 = 18054; // 18.054kOhms
    // From section 6.2.4 of the nPM1300 datasheet.
    const NTCOLD_RAW_VALUE: u32 = 1024 * RESISTANCE_10C / (RESISTANCE_10C + INTERNAL_BIAS_RESISTOR);

    // Battery shouldn't charge under 10ºC, we change the cold threshold to this temperature. The value is 10 bit across 2 registers.
    const NTCCOLD: [u8; 2] = [0x03, 0x10];
    const NTCCOLD_VALUE: u8 = (NTCOLD_RAW_VALUE >> 2 & 0xFF) as u8;
    const NTCCOLDLSB: [u8; 2] = [0x03, 0x11];
    const NTCCOLDLSB_VALUE: u8 = (NTCOLD_RAW_VALUE & 0x03) as u8; // keep only the 2 least significant bits.

    // Input current limit for startup. Couldn't find exactly why this is needed but the nrf firmware sets it and if it's not set `i2c-scanner` says it finds a device for every address, that doesn's seem right.
    const VBUSINILIMSTARTUP: [u8; 2] = [0x02, 0x02];
    const VBUSINILIMSTARTUP_VALUE: u8 = 0x05;

    // Needs to configure the nPM1300 PMIC for the modem to work correctly.
    // This is based on reading the datasheets and what is set by the firmware "hello.nrfcloud.com" downloadable at https://www.nordicsemi.com/Products/Development-hardware/Nordic-Thingy-91-X/Download in the archive "Precompiled application and modem firmware".
    {
        ariel_os_log::debug!("VBUSINILIMSTARTUP");
        i2c_bus
            .write(
                NPM1300_ADDR,
                &[
                    VBUSINILIMSTARTUP[0],
                    VBUSINILIMSTARTUP[1],
                    VBUSINILIMSTARTUP_VALUE,
                ],
            )
            .await?;
        // Disable charging before modifying the configuration
        i2c_bus
            .write(
                NPM1300_ADDR,
                &[
                    BCHGENABLESET[0],
                    BCHGENABLESET[1],
                    BCHGENABLESET_DISABLE_VALUE,
                ],
            )
            .await?;

        i2c_bus
            .write(
                NPM1300_ADDR,
                &[
                    BCHGISETDISCHARGEMSB[0],
                    BCHGISETDISCHARGEMSB[1],
                    BCHGISETDISCHARGEMSB_VALUE,
                ],
            )
            .await?;
        i2c_bus
            .write(
                NPM1300_ADDR,
                &[BUCKENCTRL[0], BUCKENCTRL[1], BUCKENCTRL_VALUE],
            )
            .await?;
        i2c_bus
            .write(
                NPM1300_ADDR,
                &[
                    BCHGISETDISCHARGELSB[0],
                    BCHGISETDISCHARGELSB[1],
                    BCHGISETDISCHARGELSB_VALUE,
                ],
            )
            .await?;

        i2c_bus
            .write(
                NPM1300_ADDR,
                &[TASKLDSW1SET[0], TASKLDSW1SET[1], taskldsw1set_value],
            )
            .await?;
        i2c_bus
            .write(
                NPM1300_ADDR,
                &[TASKLDSW2SET[0], TASKLDSW2SET[1], taskldsw2set_value],
            )
            .await?;

        i2c_bus
            .write(
                NPM1300_ADDR,
                &[BCHGISETMSB[0], BCHGISETMSB[1], BCHGISETMSB_VALUE],
            )
            .await?;
        i2c_bus
            .write(
                NPM1300_ADDR,
                &[BCHGISETLSB[0], BCHGISETLSB[1], BCHGISETLSB_VALUE],
            )
            .await?;
        ariel_os_log::debug!("BCHGVTERM");

        i2c_bus
            .write(NPM1300_ADDR, &[BCHGVTERM[0], BCHGVTERM[1], BCHGVTERM_VALUE])
            .await?;
        i2c_bus
            .write(
                NPM1300_ADDR,
                &[BCHGVTERMR[0], BCHGVTERMR[1], BCHGVTERMR_VALUE],
            )
            .await?;
        i2c_bus
            .write(
                NPM1300_ADDR,
                &[ADCNTCRSEL[0], ADCNTCRSEL[1], ADCNTCRSEL_VALUE],
            )
            .await?;
        i2c_bus
            .write(NPM1300_ADDR, &[NTCCOLD[0], NTCCOLD[1], NTCCOLD_VALUE])
            .await?;
        ariel_os_log::debug!("NTCCOLDLSB");

        i2c_bus
            .write(
                NPM1300_ADDR,
                &[NTCCOLDLSB[0], NTCCOLDLSB[1], NTCCOLDLSB_VALUE],
            )
            .await?;
        // Enable charging once everything is configured.
        i2c_bus
            .write(
                NPM1300_ADDR,
                &[
                    BCHGENABLESET[0],
                    BCHGENABLESET[1],
                    BCHGENABLESET_ENABLE_VALUE,
                ],
            )
            .await?;
    }
    Ok(())
}
