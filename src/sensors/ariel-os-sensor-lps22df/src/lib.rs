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

const PART_NUMBER: &str = "LPS22DF";

// FIXME: the LSb depends on the pin configuration.
const TARGET_I2C_ADDR: u8 = 0x5d;

const WHO_AM_I_REG_ADDR: u8 = 0x0f;
const DEVICE_ID: u8 = 0xb4;

const CTRL_REG2_ADDR: u8 = 0x11;
const STATUS_ADDR: u8 = 0x27;
const PRESS_OUT_XL_ADDR: u8 = 0x28;

// `CTRL_REG2`.
const ONESHOT_BIT: usize = 0;
const BDU_BIT: usize = 3;

// Pressure data available.
const P_DA_BIT: usize = 0;

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

pub struct Lps22dfI2c {
    state: StateAtomic,
    label: Option<&'static str>,
    i2c: OnceLock<Mutex<CriticalSectionRawMutex, I2cDevice>>,
    // config: Config,
    signaling: SensorSignaling,
}

impl Lps22dfI2c {
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

            let mut i2c = self.i2c.get().await.lock().await;

            // Trigger one-shot pressure measurement.
            let ctrl = (1 << BDU_BIT) | (1 << ONESHOT_BIT);
            let res = i2c.write(TARGET_I2C_ADDR, &[CTRL_REG2_ADDR, ctrl]).await;

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
                let mask = 1 << P_DA_BIT;
                if buf[0] & mask == mask {
                    break;
                }

                // TODO: configuration
                Timer::after_millis(10).await;
            }

            // Requires `IF_ADD_INC` to be set (which is the default).
            let mut buf = [0u8; 3];
            let res = i2c
                .write_read(TARGET_I2C_ADDR, &[PRESS_OUT_XL_ADDR], &mut buf)
                .await;

            // TODO: increases text size a bit; remove this?
            drop(i2c);

            if let Err(_err) = res {
                self.signaling
                    .signal_reading_err(ReadingError::SensorAccess)
                    .await;
                continue;
            }

            // FIXME: read and use the pressure offset as well, but only once.

            const SENSITIVITY: i32 = 4096;
            let pressure = i32::from_be_bytes([0, buf[2], buf[1], buf[0]]) / SENSITIVITY;

            // TODO: refine this
            // `PAccT` + `P_drift` from Table 2 of the datasheet.
            let pressure_accuracy = Accuracy::SymmetricalError {
                deviation: 20 + 15,
                bias: 0,
                scaling: 0,
            };

            let mut samples = Samples::from([Sample::new(pressure, pressure_accuracy)]);
            self.signaling.signal_reading(samples).await;
        }
    }
}

impl Sensor for Lps22dfI2c {
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
        &[
            Category::Pressure,
            // TODO: maybe PressureTemperature
        ]
    }

    fn reading_channels(&self) -> ReadingChannels {
        ReadingChannels::from([ReadingChannel::new(
            Label::Main,
            2, // h(Pa)
            MeasurementUnit::Pascal,
        )])
    }

    fn label(&self) -> Option<&'static str> {
        self.label
    }

    fn display_name(&self) -> Option<&'static str> {
        Some("pressure sensor")
    }

    fn part_number(&self) -> Option<&'static str> {
        Some(PART_NUMBER)
    }

    fn version(&self) -> u8 {
        0
    }
}
