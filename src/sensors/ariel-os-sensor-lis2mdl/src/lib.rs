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

const PART_NUMBER: &str = "LIS2MDL";

// This device has only one I2C address.
const TARGET_I2C_ADDR: u8 = 0b0011110;

const WHO_AM_I_REG_ADDR: u8 = 0x4f;
const DEVICE_ID: u8 = 0x40;

const CFG_REG_A_ADDR: u8 = 0x60;
const CFG_REG_C_ADDR: u8 = 0x62;
const STATUS_REG_ADDR: u8 = 0x67;
const OUTX_L_REG_ADDR: u8 = 0x68;

const COMP_TEMP_EN_BIT: u8 = 7;
const MD_SINGLE: u8 = 0b01;

const BDU_BIT: u8 = 4;

const ZYXDA_BIT: u8 = 3;

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

pub struct Lis2mdlI2c {
    state: StateAtomic,
    label: Option<&'static str>,
    i2c: OnceLock<Mutex<CriticalSectionRawMutex, I2cDevice>>,
    // config: Config,
    signaling: SensorSignaling,
}

impl Lis2mdlI2c {
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
        loop {
            self.signaling.wait_for_trigger().await;

            // Sensor configuration
            // FIXME

            let mut i2c = self.i2c.get().await.lock().await;

            // TODO: this should be moved in the init.
            let cfg = 1 << BDU_BIT;
            let res = i2c.write(TARGET_I2C_ADDR, &[CFG_REG_C_ADDR, cfg]).await;

            // TODO: enable offset cancellation in single measurement mode?

            if let Err(_err) = res {
                self.signaling
                    .signal_reading_err(ReadingError::SensorAccess)
                    .await;
                continue;
            }

            // Trigger measurement.
            let cfg = (1 << COMP_TEMP_EN_BIT) | MD_SINGLE;
            let res = i2c.write(TARGET_I2C_ADDR, &[CFG_REG_A_ADDR, cfg]).await;

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
                    .write_read(TARGET_I2C_ADDR, &[STATUS_REG_ADDR], &mut buf)
                    .await;

                // New data available.
                let mask = 1 << ZYXDA_BIT;
                if buf[0] & mask == mask {
                    break;
                }

                // TODO: configuration
                Timer::after_millis(10).await;
            }

            // Read XYZ registers.
            let mut buf = [0u8; 3 * 2];
            let res = i2c
                .write_read(TARGET_I2C_ADDR, &[OUTX_L_REG_ADDR], &mut buf)
                .await;

            // TODO: increases text size a bit; remove this?
            drop(i2c);

            if let Err(_err) = res {
                self.signaling
                    .signal_reading_err(ReadingError::SensorAccess)
                    .await;
                continue;
            }

            // Sensitivity is 1.5 (Table 2 of the datasheet).
            let mag_x = i32::from(i16::from_be_bytes([buf[1], buf[0]])) * 3 / 2;
            let mag_y = i32::from(i16::from_be_bytes([buf[3], buf[2]])) * 3 / 2;
            let mag_z = i32::from(i16::from_be_bytes([buf[5], buf[4]])) * 3 / 2;

            // TODO: could be refined by taking `TCOff`, `RMS` and the sensitivity range into
            // account.
            let mag_accuracy = Accuracy::SymmetricalError {
                deviation: 60, // `TyOff` from Table 2 in the datasheet.
                bias: 0,
                scaling: -7, // milligauss
            };

            let mut samples = Samples::from([
                Sample::new(mag_x, mag_accuracy),
                Sample::new(mag_y, mag_accuracy),
                Sample::new(mag_z, mag_accuracy),
            ]);
            self.signaling.signal_reading(samples).await;
        }
    }
}

impl Sensor for Lis2mdlI2c {
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
        &[Category::Magnetometer]
    }

    fn reading_channels(&self) -> ReadingChannels {
        ReadingChannels::from([
            ReadingChannel::new(
                Label::X,
                -7, // milligauss
                MeasurementUnit::Tesla,
            ),
            ReadingChannel::new(
                Label::Y,
                -7, // milligauss
                MeasurementUnit::Tesla,
            ),
            ReadingChannel::new(
                Label::Z,
                -7, // milligauss
                MeasurementUnit::Tesla,
            ),
        ])
    }

    fn label(&self) -> Option<&'static str> {
        self.label
    }

    fn display_name(&self) -> Option<&'static str> {
        Some("3-axis magnetometer")
    }

    fn part_number(&self) -> Option<&'static str> {
        Some(PART_NUMBER)
    }

    fn version(&self) -> u8 {
        0
    }
}
