#![no_main]
#![no_std]

mod i2c_bus;
mod pins;
mod sensors;

use ariel_os::{
    log::{error, info},
    sensors::{
        Category, Label, MeasurementUnit, REGISTRY, Reading as _,
        sensor::{ReadingChannel, Sample},
    },
    time::Timer,
};

use stm32_lcd_driver::{Digit, Lcd};

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    i2c_bus::init(peripherals.i2c);
    sensors::init().await;

    info!("This will print the readings of the temperature sensor on the LCD");

    let mut lcd = Lcd::new(peripherals.lcd.lcd, peripherals.pins.into_pins());
    lcd.initialize().await;

    loop {
        let Some(sensor) = REGISTRY
            .sensors()
            .find(|s| s.categories().contains(&Category::Temperature))
        else {
            info!("There aren't any registered temperature sensors");
            break;
        };

        if let Err(err) = sensor.trigger_measurement() {
            error!("Error when triggering a measurement: {}", err);
            Timer::after_secs(2).await;
            continue;
        }
        let reading = sensor.wait_for_reading().await;

        match reading {
            Ok(samples) => {
                for (reading_channel, sample) in samples
                    .samples()
                    .filter(|(reading_channel, _)| reading_channel.label() == Label::Temperature)
                {
                    // Our code only supports Celsius right now
                    match reading_channel.unit() {
                        MeasurementUnit::Celsius => {
                            print_temp_to_lcd(&mut lcd, sample, reading_channel)
                        }
                        _ => {}
                    }
                }
            }
            Err(err) => {
                error!("Error when reading: {}", err);
            }
        }
        Timer::after_secs(2).await;
    }
}

fn print_temp_to_lcd(lcd: &mut Lcd, sample: Sample, reading_channel: ReadingChannel) {
    let value = match sample.value() {
        Ok(value) => value,
        Err(_) => {
            info!("Error when sampling");
            return;
        }
    };

    let channel_scaling = i32::from(reading_channel.scaling());
    let factor = 10_i32.pow(channel_scaling.unsigned_abs());
    let (integer_part, decimal_part) = if channel_scaling < 0 {
        // Fixed point arithmetic
        let int_part = value / factor;
        (
            int_part,
            value.unsigned_abs() - int_part.unsigned_abs() * factor.unsigned_abs(),
        )
    } else {
        // Just multiply
        (value * factor, 0)
    };

    if integer_part >= 1000 {
        unreachable!();
    } else if integer_part <= -100 {
        unreachable!();
    }

    lcd.clear();

    if integer_part <= 0 {
        lcd.write_digit(Digit::Minus, 0).unwrap();
    }

    let integer_part = integer_part.unsigned_abs();

    // hundreds digit
    let h = integer_part / 100;
    // tens digit
    let t = integer_part / 10 - 10 * h;
    // units digit
    let u = integer_part - 10 * t + 100 * h;
    // deci digit
    let d = if decimal_part == 0 {
        0
    } else {
        decimal_part / 10_u32.pow(decimal_part.ilog10())
    };

    if h != 0 {
        // "htu"
        lcd.write_digit(digit(h), 1).unwrap();
        lcd.write_digit(digit(t), 2).unwrap();
        lcd.write_digit(digit(u), 3).unwrap();
    } else {
        // "tu.d"
        if t != 0 {
            lcd.write_digit(digit(t), 1).unwrap();
        }
        lcd.write_digit(digit(u), 2).unwrap();
        // Decimal Point is written at the same digit as the units digit
        lcd.write_digit(Digit::Dp, 2).unwrap();
        lcd.write_digit(digit(d), 3).unwrap();
    };

    lcd.write_digit(Digit::Degree, 4).unwrap();
    lcd.write_digit(Digit::C, 5).unwrap();
    lcd.display();
}

fn digit(a: u32) -> Digit {
    match a {
        1 => Digit::_1,
        2 => Digit::_2,
        3 => Digit::_3,
        4 => Digit::_4,
        5 => Digit::_5,
        6 => Digit::_6,
        7 => Digit::_7,
        8 => Digit::_8,
        9 => Digit::_9,
        0 => Digit::_0,
        a => unreachable!("{}", a),
    }
}
