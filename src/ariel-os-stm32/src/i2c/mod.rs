//! Provides support for the I2C communication bus.

#[doc(alias = "master")]
pub mod controller;

#[doc(hidden)]
pub fn init(peripherals: &mut crate::OptionalPeripherals) {
    // This macro has to be defined in this function so that the `peripherals` variables exists.
    macro_rules! take_all_i2c_peripherals {
        ($( $peripheral:ident ),*) => {
            $(
                let _ = peripherals.$peripheral.take().unwrap();
            )*
        }
    }

    // Take all I2c peripherals and do nothing with them.
    cfg_select! {
        context = "stm32c031c6" => {
            take_all_i2c_peripherals!(I2C1);
        }
        context = "stm32f042k6" => {
            take_all_i2c_peripherals!(I2C1);
        }
        context = "stm32f303cb" => {
            take_all_i2c_peripherals!(I2C1, I2C2);
        }
        context = "stm32f303re" => {
            take_all_i2c_peripherals!(I2C1, I2C2, I2C3);
        }
        any(context = "stm32f401re", context = "stm32f411re") => {
            take_all_i2c_peripherals!(I2C1, I2C2, I2C3);
        }
        any(context = "stm32h755zi", context = "stm32h753zi") => {
            take_all_i2c_peripherals!(I2C1, I2C2, I2C3, I2C4);
        }
        context = "stm32l475vg" => {
            take_all_i2c_peripherals!(I2C1, I2C2, I2C3);
        }
        any(context = "stm32u073kc", context = "stm32u083mc") => {
            take_all_i2c_peripherals!(I2C1, I2C2, I2C3, I2C4);
        }
        context = "stm32u585ai" => {
            take_all_i2c_peripherals!(I2C1, I2C2, I2C3, I2C4);
        }
        context = "stm32wb55rg" => {
            take_all_i2c_peripherals!(I2C1, I2C3);
        }
        _ => {
            compile_error!("this STM32 chip is not supported");
        }
    }
}
