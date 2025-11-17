use ariel_os::hal::{i2c, peripherals};

ariel_os::hal::group_peripherals!(Peripherals {
    lcd: LcdPeripherals,
    pins: Pins,
    i2c: I2CPins,
});

pub type SensorI2c = i2c::controller::I2C1;

ariel_os::hal::define_peripherals!(LcdPeripherals { lcd: LCD });
ariel_os::hal::define_peripherals!(Pins {
    pc4: PC4,
    pc5: PC5,
    pb1: PB1,
    pe7: PE7,
    pe8: PE8,
    pe9: PE9,
    pb11: PB11,
    pb14: PB14,
    pb15: PB15,
    pd8: PD8,
    pd9: PD9,
    pd12: PD12,
    pb9: PB9,
    pa10: PA10,
    pa9: PA9,
    pa8: PA8,
    pd13: PD13,
    pc6: PC6,
    pc8: PC8,
    pc9: PC9,
    pc10: PC10,
    pd0: PD0,
    pd1: PD1,
    pd3: PD3,
    pd4: PD4,
    pd5: PD5,
    pd6: PD6,
    pc11: PC11,
});

ariel_os::hal::define_peripherals!(I2CPins {
    i2c_sda: PB7,
    i2c_scl: PB8,
});

impl Pins {
    pub fn into_pins(self) -> stm32_lcd_driver::Pins {
        stm32_lcd_driver::Pins {
            pc4: self.pc4,
            pc5: self.pc5,
            pb1: self.pb1,
            pe7: self.pe7,
            pe8: self.pe8,
            pe9: self.pe9,
            pb11: self.pb11,
            pb14: self.pb14,
            pb15: self.pb15,
            pd8: self.pd8,
            pd9: self.pd9,
            pd12: self.pd12,
            pb9: self.pb9,
            pa10: self.pa10,
            pa9: self.pa9,
            pa8: self.pa8,
            pd13: self.pd13,
            pc6: self.pc6,
            pc8: self.pc8,
            pc9: self.pc9,
            pc10: self.pc10,
            pd0: self.pd0,
            pd1: self.pd1,
            pd3: self.pd3,
            pd4: self.pd4,
            pd5: self.pd5,
            pd6: self.pd6,
            pc11: self.pc11,
        }
    }
}
