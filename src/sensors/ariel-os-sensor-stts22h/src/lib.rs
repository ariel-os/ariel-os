#![no_std]

use ariel_os_debug::log::debug;
use ariel_os_embassy::{api::time::Timer, asynch::Spawner};
use ariel_os_hal::i2c::controller::I2cDevice;
use ariel_os_sensors::{
    Category, Label, MeasurementUnit, Sensor,
    sensor::{
        Accuracy, Mode as SensorMode, ReadingChannel, ReadingChannels, ReadingError, ReadingWaiter,
        Sample, Samples, SetModeError, State, TriggerMeasurementError,
    },
};
use ariel_os_sensors_utils::{SensorSignaling, StateAtomic};
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, once_lock::OnceLock,
};
use embedded_hal_async::i2c::I2c;

const PART_NUMBER: &str = "STTS22H";

// FIXME: depends on the pin config
const TARGET_I2C_ADDR: u8 = 0x7e >> 1;

const WHO_AM_I_REG_ADDR: u8 = 0x01;
const DEVICE_ID: u8 = 0xa0;

const CTRL_REG_ADDR: u8 = 0x04;
const STATUS_REG_ADDR: u8 = 0x05;
const TEMP_L_OUT_REG_ADDR: u8 = 0x06;
const TEMP_H_OUT_REG_ADDR: u8 = 0x07;

const ONE_SHOT_BIT: usize = 0;
const FREERUN_BIT: usize = 2;
const IF_ADD_INC_BIT: usize = 3;
const BDU_BIT: usize = 6;
const BUSY_BIT: usize = 0;

#[derive(Debug)]
#[non_exhaustive]
pub struct Config {
    // FIXME
}

impl Default for Config {
    fn default() -> Self {
        Self {}
    }
}

ariel_os_embassy::hal::define_peripherals!(Peripherals {});

pub struct Stts22hI2c {
    state: StateAtomic,
    label: Option<&'static str>,
    i2c: OnceLock<Mutex<CriticalSectionRawMutex, I2cDevice>>,
    // config: Config,
    signaling: SensorSignaling,
}

impl Stts22hI2c {
    #[expect(clippy::new_without_default)]
    #[must_use]
    pub const fn new(label: Option<&'static str>) -> Self {
        Self {
            state: StateAtomic::new(State::Uninitialized),
            label,
            i2c: OnceLock::new(),
            // config: Config {},
            signaling: SensorSignaling::new(),
        }
    }

    pub async fn init(
        &'static self,
        _spawner: Spawner,
        peripherals: Peripherals,
        i2c_device: I2cDevice,
        config: Config,
    ) {
        if !self.i2c.is_set() {
            // FIXME
            // self.config = config;

            let _ = self.i2c.init(Mutex::new(i2c_device));

            self.state.set(State::Enabled);
            debug!("{} enabled", PART_NUMBER);
        }
    }

    pub async fn run(&self) -> ! {
        loop {
            self.signaling.wait_for_trigger().await;

            // Sensor configuration
            let mut ctrl = 0u8;
            ctrl |= 1 << ONE_SHOT_BIT;
            ctrl |= 1 << IF_ADD_INC_BIT;
            ctrl |= 1 << BDU_BIT;

            let mut i2c = self.i2c.get().await.lock().await;

            // Trigger one-shot measurement
            let res = i2c.write(TARGET_I2C_ADDR, &[CTRL_REG_ADDR, ctrl]).await;

            if let Err(_err) = res {
                self.signaling
                    .signal_reading_err(ReadingError::SensorAccess)
                    .await;
                continue;
            }

            // Wait for the measurement
            loop {
                let mut buf = [0u8];
                let res = i2c
                    .write_read(TARGET_I2C_ADDR, &[STATUS_REG_ADDR], &mut buf)
                    .await;

                // Not BUSY anymore
                if buf[0] & (1 << BUSY_BIT) == 0 {
                    break;
                }

                // TODO: configuration
                Timer::after_millis(10).await;
            }

            // Reads both temperature bytes thanks to IF_ADD_INC.
            let mut buf = [0u8; 2];
            let res = i2c
                .write_read(TARGET_I2C_ADDR, &[TEMP_L_OUT_REG_ADDR], &mut buf)
                .await;

            // TODO: increases text size a bit; remove this?
            drop(i2c);

            if let Err(_err) = res {
                self.signaling
                    .signal_reading_err(ReadingError::SensorAccess)
                    .await;
                continue;
            }

            // Smaller text size than using `i32::from_be_bytes()`
            let temp = i32::from(buf[1]) << 8 | i32::from(buf[0]);

            let accuracy = accuracy(temp);
            let sample = Sample::new(temp, accuracy);

            self.signaling.signal_reading(Samples::from([sample])).await;
        }
    }
}

impl Sensor for Stts22hI2c {
    fn trigger_measurement(&self) -> Result<(), TriggerMeasurementError> {
        if self.state.get() != State::Enabled {
            return Err(TriggerMeasurementError::NonEnabled);
        }

        self.signaling.trigger_measurement();

        Ok(())
    }

    fn wait_for_reading(&'static self) -> ReadingWaiter {
        if self.state.get() != State::Enabled {
            return ReadingWaiter::Err(ReadingError::NonEnabled);
        }

        self.signaling.wait_for_reading()
    }

    fn set_mode(&self, mode: SensorMode) -> Result<State, SetModeError> {
        let new_state = self.state.set_mode(mode);

        if new_state == State::Uninitialized {
            Err(SetModeError::Uninitialized)
        } else {
            Ok(new_state)
        }
    }

    fn state(&self) -> State {
        self.state.get()
    }

    fn categories(&self) -> &'static [Category] {
        &[Category::Temperature]
    }

    fn reading_channels(&self) -> ReadingChannels {
        ReadingChannels::from([ReadingChannel::new(
            Label::Main,
            -2,
            MeasurementUnit::Celsius,
        )])
    }

    fn label(&self) -> Option<&'static str> {
        self.label
    }

    fn display_name(&self) -> Option<&'static str> {
        Some("thermometer")
    }

    fn part_number(&self) -> Option<&'static str> {
        Some(PART_NUMBER)
    }

    fn version(&self) -> u8 {
        0
    }
}

fn accuracy(temp: i32) -> Accuracy {
    // FIXME: Figure 2 of the datasheet is unclear
    // Accuracy of 0.5 °C between -5 °C and +55 °C
    if -500 < temp && temp < 5500 {
        return Accuracy::SymmetricalError {
            deviation: 50,
            bias: 0,
            scaling: -2,
        };
    }

    // Accuracy of 1.0 °C otherwise
    return Accuracy::SymmetricalError {
        deviation: 100,
        bias: 0,
        scaling: -2,
    };
}
