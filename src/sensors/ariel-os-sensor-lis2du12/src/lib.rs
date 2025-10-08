#![no_std]

use ariel_os_debug::log::{debug, info};
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

const PART_NUMBER: &str = "LIS2DU12";

// FIXME: the LSb depends on the pin config
const TARGET_I2C_ADDR: u8 = 0b0011001;

const WHO_AM_I_REG_ADDR: u8 = 0x43;
const DEVICE_ID: u8 = 0b01000101;

const CTRL1_ADDR: u8 = 0x10;
const CTRL4_ADDR: u8 = 0x13;
const CTRL5_ADDR: u8 = 0x14;
const STATUS_ADDR: u8 = 0x25;
const OUT_X_L_ADDR: u8 = 0x28;

const IF_ADD_INC_BIT: u8 = 4;

const SOC_BIT: u8 = 1;
const BDU_BIT: u8 = 5;

const ONE_SHOT: u8 = 0xf0;

const DRDY_BIT: u8 = 0;

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

pub struct Lis2du12I2c {
    state: StateAtomic,
    label: Option<&'static str>,
    i2c: OnceLock<Mutex<CriticalSectionRawMutex, I2cDevice>>,
    // config: Config,
    signaling: SensorSignaling,
}

impl Lis2du12I2c {
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

    pub async fn run(&'static self) -> ! {
        // Sensor configuration
        {
            // TODO: possible errors could be stored and reported from inside the loop.

            let mut i2c = self.i2c.get().await.lock().await;
            let ctrl = 1 << IF_ADD_INC_BIT;
            let _ = i2c.write(TARGET_I2C_ADDR, &[CTRL1_ADDR, ctrl]).await;

            let ctrl = ONE_SHOT;
            let _ = i2c.write(TARGET_I2C_ADDR, &[CTRL5_ADDR, ctrl]).await;
        }

        loop {
            self.signaling.wait_for_trigger().await;

            let mut i2c = self.i2c.get().await.lock().await;

            // Trigger acceleration measurement.
            let ctrl = (1 << BDU_BIT) | (1 << SOC_BIT);
            let res = i2c.write(TARGET_I2C_ADDR, &[CTRL4_ADDR, ctrl]).await;

            if let Err(_err) = res {
                self.signaling
                    .signal_reading_err(ReadingError::SensorAccess)
                    .await;
                continue;
            }

            // Wait for the measurement.
            loop {
                let mut buf = [0u8];
                let res = i2c
                    .write_read(TARGET_I2C_ADDR, &[STATUS_ADDR], &mut buf)
                    .await;

                // New data available.
                let mask = 1 << DRDY_BIT;
                if buf[0] & mask == mask {
                    break;
                }

                // TODO: configuration
                Timer::after_millis(10).await;
            }

            // Read all acceleration registers.
            let mut buf = [0u8; 3 * 2];
            let res = i2c
                .write_read(TARGET_I2C_ADDR, &[OUT_X_L_ADDR], &mut buf)
                .await;

            // TODO: increases text size a bit; remove this?
            drop(i2c);

            if let Err(_err) = res {
                self.signaling
                    .signal_reading_err(ReadingError::SensorAccess)
                    .await;
                continue;
            }

            let accel_scale = 16; // FIXME: why is this 16 and not 2?
            let accel_x = i32::from(i16::from_be_bytes([buf[1], buf[0]])) / accel_scale;
            let accel_y = i32::from(i16::from_be_bytes([buf[3], buf[2]])) / accel_scale;
            let accel_z = i32::from(i16::from_be_bytes([buf[5], buf[4]])) / accel_scale;

            // `TyOff` from Table 2 of the datasheet.
            let accel_accuracy = Accuracy::SymmetricalError {
                deviation: 11, // TODO: this could possibly be refined by taking into account `An` as well.
                bias: 0,
                scaling: -3,
            };

            let mut samples = Samples::from([
                Sample::new(accel_x, accel_accuracy),
                Sample::new(accel_y, accel_accuracy),
                Sample::new(accel_z, accel_accuracy),
            ]);
            self.signaling.signal_reading(samples).await;
        }
    }
}

impl Sensor for Lis2du12I2c {
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
        &[Category::Accelerometer]
    }

    fn reading_channels(&self) -> ReadingChannels {
        ReadingChannels::from([
            ReadingChannel::new(Label::X, -3, MeasurementUnit::AccelG),
            ReadingChannel::new(Label::Y, -3, MeasurementUnit::AccelG),
            ReadingChannel::new(Label::Z, -3, MeasurementUnit::AccelG),
        ])
    }

    fn label(&self) -> Option<&'static str> {
        self.label
    }

    fn display_name(&self) -> Option<&'static str> {
        Some("3-axis accelerometer")
    }

    fn part_number(&self) -> Option<&'static str> {
        Some(PART_NUMBER)
    }

    fn version(&self) -> u8 {
        0
    }
}
