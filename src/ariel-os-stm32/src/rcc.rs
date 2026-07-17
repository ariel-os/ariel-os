pub fn config() -> embassy_stm32::rcc::Config {
    cfg_select! {
        feature = "rcc-config-override" => {
            unsafe extern "Rust" {
                fn __ariel_os_rcc_config() -> embassy_stm32::rcc::Config;
            }
            unsafe { __ariel_os_rcc_config() }
        }
        _ => default(),
    }
}

#[cfg_attr(feature = "rcc-config-override", expect(dead_code))]
fn default() -> embassy_stm32::rcc::Config {
    #[allow(unused_mut, reason = "conditional compilation")]
    let mut rcc = embassy_stm32::rcc::Config::default();

    #[cfg(context = "st-b-l475e-iot01a")]
    {
        use embassy_stm32::rcc::*;

        // This board has an LSE clock, we can use it to calibrate the MSI clock
        rcc.ls = LsConfig {
            rtc: RtcClockSource::LSE,
            lsi: false,
            lse: Some(LseConfig {
                frequency: embassy_stm32::time::Hertz(32768),
                mode: LseMode::Oscillator(LseDrive::MediumHigh),
            }),
        };
        rcc.hsi = false;
        // Setting the MSI range to 48 MHz crashes the system. If the source of the issue is found,
        // we can use MSI as the clock source for the usb peripheral directly and avoid using more PLLs.
        rcc.msi = Some(MSIRange::RANGE8M);
        rcc.pll = Some(Pll {
            source: PllSource::MSI,
            prediv: PllPreDiv::DIV1, // 8 Mhz
            mul: PllMul::MUL20,      // 160 MHz
            divp: None,
            divq: None,
            divr: Some(PllRDiv::DIV2), // sysclk 80Mhz (8 / 1  * 20 / 2)
        });
        rcc.sys = Sysclk::PLL1_R;
        rcc.pllsai1 = Some(Pll {
            source: PllSource::MSI,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL12, // 8 MHz MSI * 12 = 96 MHz
            divp: None,
            divq: Some(PllQDiv::DIV2), // USB 48 MHz (8 / 1 * 12 / 2)
            divr: None,
        });
        // With a 32.768 kHz LSE, the MSI clock will be calibrated and considered accurate enough.
        // Embassy automatically enables MSIPLLEN if the LSE is configured.
        rcc.mux.clk48sel = mux::Clk48sel::PLLSAI1_Q;
    }

    #[cfg(context = "st-nucleo-wb55")]
    {
        use embassy_stm32::rcc::*;

        rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        rcc.sys = Sysclk::PLL1_R;
        rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(32000000),
            mode: HseMode::Oscillator,
            prescaler: HsePrescaler::DIV1,
        });
        rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL10,
            divp: None,
            divq: None,
            divr: Some(PllRDiv::DIV2), // sysclk 80Mhz (32 / 2 * 10 / 2)
        });
        rcc.mux.clk48sel = mux::Clk48sel::HSI48;
    }

    #[cfg(any(context = "stm32f303cb", context = "stm32f303re"))]
    {
        use embassy_stm32::rcc::*;

        rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(8000000),
            mode: HseMode::Oscillator,
        });
        rcc.pll = Some(Pll {
            src: PllSource::HSE,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL9,
        });
        rcc.ahb_pre = AHBPrescaler::DIV1;
        rcc.apb1_pre = APBPrescaler::DIV4;
        rcc.apb2_pre = APBPrescaler::DIV2;
        rcc.sys = Sysclk::PLL1_P; // 72 MHz (8 / 1 * 9)
    }

    #[cfg(context = "st-nucleo-f401re")]
    {
        use embassy_stm32::rcc::*;
        rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(8000000),
            mode: HseMode::Bypass,
        });
        rcc.pll_src = PllSource::HSE;
        rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL168,
            divp: Some(PllPDiv::DIV4),
            divq: Some(PllQDiv::DIV7),
            divr: None,
        });
        rcc.ahb_pre = AHBPrescaler::DIV1;
        rcc.apb1_pre = APBPrescaler::DIV4;
        rcc.apb2_pre = APBPrescaler::DIV2;
        rcc.sys = Sysclk::PLL1_P;
    }

    #[cfg(context = "st-nucleo-f767zi")]
    {
        use embassy_stm32::rcc::*;
        rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(8000000),
            mode: HseMode::Bypass,
        });
        rcc.pll_src = PllSource::HSE;
        rcc.pll = Some(Pll {
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL216,
            divp: Some(PllPDiv::DIV2),
            divq: None,
            divr: None,
        });
        rcc.ahb_pre = AHBPrescaler::DIV1;
        rcc.apb1_pre = APBPrescaler::DIV4;
        rcc.apb2_pre = APBPrescaler::DIV2;
        rcc.sys = Sysclk::PLL1_P;
    }

    #[cfg(context = "stm32h755zi")]
    {
        use embassy_stm32::rcc::*;

        rcc.hsi = Some(HSIPrescaler::DIV1);
        rcc.csi = true;
        rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            // Required for SPI (configured by `spi123sel`)
            divq: Some(PllDiv::DIV16), // FIXME: adjust this divider
            divr: None,
        });
        rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        rcc.voltage_scale = VoltageScale::Scale1;
        // Set SMPS power config otherwise MCU will not powered after next power-off
        rcc.supply_config = SupplyConfig::DirectSMPS;
        rcc.mux.usbsel = mux::Usbsel::HSI48;
        // Select the clock signal used for SPI1, SPI2, and SPI3.
        // FIXME: what to do about SPI4, SPI5, and SPI6?
        rcc.mux.spi123sel = mux::Saisel::PLL1_Q; // Reset value
    }

    #[cfg(context = "stm32h753zi")]
    {
        use embassy_stm32::rcc::*;

        rcc.hsi = Some(HSIPrescaler::DIV1);
        rcc.csi = true;
        rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        rcc.pll1 = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL50,
            divp: Some(PllDiv::DIV2),
            divq: Some(PllDiv::DIV16), // 50 MHz
            divr: None,
        });
        rcc.sys = Sysclk::PLL1_P; // 400 Mhz
        rcc.ahb_pre = AHBPrescaler::DIV2; // 200 Mhz
        rcc.apb1_pre = APBPrescaler::DIV2; // 100 Mhz
        rcc.apb2_pre = APBPrescaler::DIV2; // 100 Mhz
        rcc.apb3_pre = APBPrescaler::DIV2; // 100 Mhz
        rcc.apb4_pre = APBPrescaler::DIV2; // 100 Mhz
        rcc.voltage_scale = VoltageScale::Scale1;
        rcc.mux.usbsel = mux::Usbsel::HSI48;
        // Select the clock signal used for SPI1, SPI2, and SPI3.
        // FIXME: what to do about SPI4, SPI5, and SPI6?
        rcc.mux.spi123sel = mux::Saisel::PLL1_Q; // Reset value
    }

    #[cfg(any(context = "stm32u073kc", context = "stm32u083mc"))]
    {
        use embassy_stm32::rcc::*;

        rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        // No HSE fitted on the stm32u083c-dk board
        rcc.hsi = true;
        rcc.sys = Sysclk::PLL1_R;
        rcc.pll = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL7,
            divp: None,
            divq: None,
            divr: Some(PllRDiv::DIV2), // sysclk 56Mhz
        });
        rcc.mux.clk48sel = mux::Clk48sel::HSI48;
    }

    #[cfg(context = "st-steval-mkboxpro")]
    {
        use embassy_stm32::rcc::*;

        rcc.ls = LsConfig {
            rtc: RtcClockSource::LSE,
            lsi: true,
            lse: Some(LseConfig {
                peripherals_clocked: true,
                frequency: embassy_stm32::time::Hertz(32768),
                mode: LseMode::Oscillator(LseDrive::MediumHigh),
            }),
        };
        rcc.hsi = true;
        rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        rcc.sys = Sysclk::PLL1_R;
        rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(16_000_000),
            mode: HseMode::Oscillator,
        });
        rcc.pll1 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV1,
            mul: PllMul::MUL10,
            divp: None,
            divq: None,
            divr: Some(PllDiv::DIV1), // sysclk 160Mhz (16 / 1 * 10 / 1)
        });
        rcc.sys = Sysclk::PLL1_R;
        rcc.mux.iclksel = mux::Iclksel::HSI48;
        rcc.voltage_range = VoltageScale::RANGE1;
    }

    #[cfg(context = "stm32f042k6")]
    {
        use embassy_stm32::rcc::*;

        rcc.hsi48 = Some(Hsi48Config {
            sync_from_usb: true,
        }); // needed for USB
        rcc.sys = Sysclk::HSI48;
        rcc.pll = Some(Pll {
            src: PllSource::HSI48,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL2,
        });
    }

    #[cfg(context = "seeedstudio-lora-e5-mini")]
    {
        use embassy_stm32::rcc::*;

        rcc.hse = Some(Hse {
            freq: embassy_stm32::time::Hertz(32_000_000),
            mode: HseMode::Bypass,
            prescaler: HsePrescaler::DIV1,
        });
        rcc.ls = LsConfig::default_lse();
        rcc.msi = None;
        rcc.pll = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,
            mul: PllMul::MUL6,
            divp: None,
            divq: Some(PllQDiv::DIV2), // PLL1_Q clock (32 / 2 * 6 / 2), used for RNG
            divr: Some(PllRDiv::DIV2), // sysclk 48Mhz clock (32 / 2 * 6 / 2)
        });

        rcc.sys = Sysclk::PLL1_R;
    }

    #[cfg(context = "st-nucleo-wba65ri")]
    {
        use embassy_stm32::rcc::*;

        rcc.hse = Some(Hse {
            prescaler: HsePrescaler::DIV1,
        });
        rcc.pll1 = Some(Pll {
            source: PllSource::HSE,
            prediv: PllPreDiv::DIV2,  // 32 / 2 = 16 MHz
            mul: PllMul::MUL12,       // 16 * 12 = 192 MHz
            divp: Some(PllDiv::DIV6), // 192 / 6 = 32 MHz (for SAI1)
            divq: None,
            divr: Some(PllDiv::DIV2), // 192 / 2 = 96 MHz (sysclk)
            frac: None,
        });
        rcc.sys = Sysclk::PLL1_R;
        rcc.voltage_scale = VoltageScale::RANGE1;
        rcc.mux.otghssel = Otghssel::HSE; // USB OTG HS ref clock from HSE (32 MHz)
    }

    #[cfg(context = "stm32g431rb")]
    {
        use embassy_stm32::rcc::*;

        // HSI 16 MHz / 4 = 4 MHz; * 85 = 340 MHz VCO; / 2 = 170 MHz SYSCLK.
        rcc.hsi = true;
        rcc.pll = Some(Pll {
            source: PllSource::HSI,
            prediv: PllPreDiv::DIV4,
            mul: PllMul::MUL85,
            divp: None,
            divq: None,
            divr: Some(PllRDiv::DIV2),
        });
        rcc.sys = Sysclk::PLL1_R;
        rcc.boost = true; // required for SYSCLK > 150 MHz
    }

    rcc
}
