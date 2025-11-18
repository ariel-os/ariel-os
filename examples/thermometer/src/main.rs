#![no_main]
#![no_std]

mod i2c_bus;
mod pins;
mod sensors;

use ariel_os::{
    debug::log::{error, info},
    sensors::{
        Category, MeasurementUnit, REGISTRY, Reading as _,
        sensor::{ReadingChannel, Sample},
    },
    time::Timer,
};

use stm32_lcd_driver::Lcd;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    let pins::Peripherals {
        lcd: lcd_peris,
        pins: pin_peris,
        i2c: i2c_peris,
    } = peripherals;
    i2c_bus::init(i2c_peris);
    sensors::init().await;

    info!("Will print the readings of temperature sensor on the LCD Screen");

    let mut lcd = Lcd::new(lcd_peris.lcd, pin_peris.into_pins());
    lcd.initialize().await;

    loop {
        // Trigger measurements for each sensor driver in parallel.
        for sensor in REGISTRY
            .sensors()
            .filter(|s| s.categories().contains(&Category::Temperature))
        {
            if let Err(err) = sensor.trigger_measurement() {
                error!("Error when triggering a measurement: {}", err);
            }
        }

        // Then, collect and display the readings one at a time.
        for sensor in REGISTRY
            .sensors()
            .filter(|s| s.categories().contains(&Category::Temperature))
        {
            let reading = sensor.wait_for_reading().await;

            match reading {
                Ok(samples) => {
                    for (reading_channel, sample) in samples.samples() {
                        // Even though sensors that aren't temperature sensors are filtered out,
                        // A single sensor could provide multiple readings including ones that aren't temperature
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

    let channel_scaling = reading_channel.scaling();

    let integer_part: i32 = value as i32 / 10_i32.pow(channel_scaling.abs() as u32);
    let decimal_part: u32 = value.unsigned_abs() - integer_part.unsigned_abs() * 10_u32.pow(channel_scaling.abs() as u32);

    if integer_part >= 1000 {
        unreachable!();
    } else if integer_part <= -100 {
        unreachable!();
    }

    // 6 "Digits" available on the LCD display but
    // - '.' takes no space on the LCD display
    // - '°' takes up 2 bytes
    // so the buffer has to hold 8 bytes;
    let mut lcd_bytes = [0u8; 8];

    lcd_bytes[5..8].copy_from_slice("°C".as_bytes());
    if integer_part <= 0 {
        lcd_bytes[0..1].copy_from_slice("-".as_bytes());
    } else {
        lcd_bytes[0..1].copy_from_slice(" ".as_bytes());
    }
    let decimal_part = decimal_part as u32;
    let integer_part = integer_part.abs() as u32;

    // hundreds digit
    let h = integer_part / 100;
    // tens digit
    let t = integer_part / 10 - 10 * h;
    // units digit
    let u = integer_part - 10 * t + 100 * h;
    // deci digit
    let d = decimal_part / 10_u32.pow(decimal_part.ilog10());

    let start = if h != 0 {
        // the buffer will hold ["  XXX°C"]
        lcd_bytes[1..2].copy_from_slice(" ".as_bytes());
        lcd_bytes[2..3].copy_from_slice(digit(h).as_bytes());
        lcd_bytes[3..4].copy_from_slice(digit(t).as_bytes());
        lcd_bytes[4..5].copy_from_slice(digit(u).as_bytes());
        2
    } else {
        // the buffer will hold ["sXX.X°C"] where s is either " " or "-"
        lcd_bytes[1..2].copy_from_slice(digit(t).as_bytes());
        lcd_bytes[2..3].copy_from_slice(digit(u).as_bytes());
        lcd_bytes[3..4].copy_from_slice(".".as_bytes());
        lcd_bytes[4..5].copy_from_slice(digit(d).as_bytes());
        0
    };

    lcd.clear();
    lcd.write_string(str::from_utf8(&lcd_bytes).unwrap(), start)
        .unwrap();
    lcd.display();
}

fn digit(a: u32) -> &'static str {
    match a {
        1 => "1",
        2 => "2",
        3 => "3",
        4 => "4",
        5 => "5",
        6 => "6",
        7 => "7",
        8 => "8",
        9 => "9",
        0 => "0",
        a => unreachable!("{}", a),
    }
}
