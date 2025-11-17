#![no_main]
#![no_std]

mod i2c_bus;
mod pins;
mod sensors;

use ariel_os::{
    debug::log::{error, info},
    sensors::{
        REGISTRY, Reading as _,
        sensor::{ReadingChannel, Sample, SampleMetadata},
    },
    time::Timer,
};
use ariel_os_sensors::MeasurementUnit;
use stm32_lcd_driver::Lcd;

#[ariel_os::task(autostart, peripherals)]
async fn main(peripherals: pins::Peripherals) {
    let pins::Peripherals {
        lcd: lcd_peris,
        pins: pin_peris,
        i2c: i2c_peris
    } = peripherals;
    i2c_bus::init(i2c_peris);
    sensors::init().await;

    info!("Will print the readings of temperature sensor on the LCD Screen");

    let mut lcd = Lcd::new(lcd_peris.lcd, pin_peris.into_pins());
    lcd.initialize().await;
    loop {
        // Trigger measurements for each sensor driver in parallel.
        for sensor in REGISTRY.sensors() {
            if let Err(err) = sensor.trigger_measurement() {
                error!("Error when triggering a measurement: {}", err);
            }
        }

        // Then, collect and display the readings one at a time.
        for sensor in REGISTRY.sensors() {
            let reading = sensor.wait_for_reading().await;

            match reading {
                Ok(samples) => {
                    for (reading_channel, sample) in samples.samples() {
                        match reading_channel.unit() {
                            MeasurementUnit::Celsius => print_temp_to_lcd(&mut lcd, sample, reading_channel),
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
    let value = if channel_scaling < 0 {
        value as f32 / 10i32.pow(-channel_scaling as u32) as f32
    } else {
        value as f32 * 10i32.pow(channel_scaling as u32) as f32
    };

    match sample.metadata() {
        SampleMetadata::SymmetricalError { deviation:_, bias: _, scaling: _} |
        SampleMetadata::UnknownAccuracy |
        SampleMetadata::NoMeasurementError => {
            // 6 "Digits" available on the LCD display but
            // - '.' takes no space on the LCD display
            // - '°' takes up 2 bytes
            // so the buffer has to hold 8 bytes;
            let mut lcd_bytes = [0_u8; 8];

            lcd_bytes[5..8].copy_from_slice("°C".as_bytes());

            let start= if value >= 1000_f32 {
                unreachable!("No way that this sensor survives 1000 °C");
            }
            else if value >= 100_f32  {
                // Unlikely but possible
                let h = digit((value / 100_f32) as u32 - (value / 1000_f32) as u32 * 10);
                let d = digit((value / 10_f32) as u32 - (value / 100_f32) as u32 * 10);
                let u = digit((value / 1_f32) as u32 - (value / 10_f32) as u32 * 10);
                lcd_bytes[0..2].copy_from_slice("  ".as_bytes());
                lcd_bytes[2..3].copy_from_slice(h.as_bytes());
                lcd_bytes[3..4].copy_from_slice(d.as_bytes());
                lcd_bytes[4..5].copy_from_slice(u.as_bytes());
                2
            }
            else if value >= 0_f32 {
                let d = digit((value / 10_f32) as u32 - (value / 100_f32) as u32 * 10);
                let u = digit(value as u32 - ((value / 10_f32) as u32 * 10));
                let dec = digit((value * 10_f32) as u32 - value as u32 * 10);
                let cent = digit((value * 100_f32) as u32 - (value * 10_f32) as u32 * 10);
                lcd_bytes[0..1].copy_from_slice(d.as_bytes());
                lcd_bytes[1..2].copy_from_slice(u.as_bytes());
                lcd_bytes[2..3].copy_from_slice(".".as_bytes());
                lcd_bytes[3..4].copy_from_slice(dec.as_bytes());
                lcd_bytes[4..5].copy_from_slice(cent.as_bytes());
                0
            }
            else if value >= -100_f32 {
                let value = value.abs();
                let d = digit((value / 10_f32) as u32 - ((value / 100_f32) as u32 * 10));
                let u = digit(value as u32 - ((value / 10_f32) as u32 * 10));
                let dec = digit((value * 10_f32) as u32 - value as u32 * 10);

                lcd_bytes[0..1].copy_from_slice("-".as_bytes());
                lcd_bytes[1..2].copy_from_slice(d.as_bytes());
                lcd_bytes[2..3].copy_from_slice(u.as_bytes());
                lcd_bytes[3..4].copy_from_slice(".".as_bytes());
                lcd_bytes[4..5].copy_from_slice(dec.as_bytes());
                0
            }
            else {
                unreachable!("No way that this sensor survives -100°C");
            };

            lcd.clear();
            lcd.write_string(str::from_utf8(&lcd_bytes).unwrap(), start).unwrap();
            lcd.display();
        },
        _ => unimplemented!()
    }
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