//! Driver for the sensor used over I2C.

use ariel_os_sensors::{
    Category, Label, MeasurementUnit, Sensor,
    sensor::{
        Mode as SensorMode, ReadingChannel, ReadingChannels, ReadingError, ReadingResult,
        ReadingWaiter, Samples, SetModeError, State,
        TriggerMeasurementError,
    },
    signal::Signal as ReadingSignal,
};
use ariel_os_sensors_utils::AtomicState;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, once_lock::OnceLock, signal::Signal,
};
use embassy_time::Timer;
use embedded_hal_async::i2c::I2c;

#[allow(unused_imports, reason = "Hex and Debug2Format are actually used when a logging facade is selected")]
use ariel_os_debug_log::{Debug2Format, Hex};
use ariel_os_debug_log::debug;

const I2C_ADRESS: u8 = 0x29;

use crate::{AlsGain, AlsIntegrationTimeMs, AlsMeasurementRateMs, MeasuredAlsDataGain, Register, AlsPersist};
use crate::PART_NUMBER;

/// Configuration of the sensor
#[derive(Debug, Clone, Copy)]
pub struct Config {
    /// Gain used for the measurements. Impact the range of possible measurements.
    pub gain: AlsGain,
    /// Integration that sensor will use for its measurements.
    pub integration_time: AlsIntegrationTimeMs,
    /// Rate at which the sensor's data register get updated.
    pub measurement_rate: AlsMeasurementRateMs,
    /// Upper threshold used to trip the interrupt pin.
    pub upper_threshold: u16,
    /// Upper threshold used to trip the interrupt pin.
    pub lower_threshold: u16,
    /// Enables the interrupt functionnality
    pub interrupt_enable: bool,
    /// Reverses the level that will observe on the interrupt line
    /// in case of a sensor driver interrupt.
    pub reverse_interrupt_polarity: bool,
    /// Controls how many consecutives measurements are required to
    /// trip the interrupt pin.
    pub interrupt_persistence: AlsPersist,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            gain: AlsGain::default(),
            integration_time: AlsIntegrationTimeMs::default(),
            measurement_rate: AlsMeasurementRateMs::default(),
            upper_threshold: 0xFF,
            lower_threshold: 0x00,
            interrupt_enable: false,
            reverse_interrupt_polarity: false,
            interrupt_persistence: AlsPersist::default(),
        }
    }
}

/// Driver to use a LTR303ALS-01 over I2C.
pub struct Ltr303Als01<I2C> {
    state: AtomicState,
    label: Option<&'static str>,
    i2c: OnceLock<Mutex<CriticalSectionRawMutex, I2C>>,
    signaling: Signal<CriticalSectionRawMutex, ()>,
    reading: ReadingSignal<ReadingResult<Samples>>,
}

// FIXME: Find a way to use interrupt pins
ariel_os_hal::define_peripherals!(
    /// Peripherals required by the sensor driver.
    Peripherals {}
);

impl<I2C: I2c + Send> Ltr303Als01<I2C> {
    /// Creates an uninitialized driver.
    #[must_use]
    pub const fn new(label: Option<&'static str>) -> Self {
        Self {
            state: AtomicState::new(State::Uninitialized),
            label,
            i2c: OnceLock::new(),
            signaling: Signal::new(),
            reading: ReadingSignal::new(),
        }
    }

    /// Initialize the driver based on the given configuration
    pub async fn init(
        &'static self,
        _peripherals: Peripherals,
        mut i2c_device: I2C,
        config: Config,
    ) {
        debug!("[LTR-303ALS-01] Initilization started");
        if !self.i2c.is_set() {
            Timer::after_millis(crate::INITIAL_STARTUP_TIME_MS).await;

            let mut gain_ctrl: u8 = 0;
            gain_ctrl |= config.gain as u8;
            debug!("[LTR-303ALS-01] Configuring gain to {:?}", config.gain);

            if i2c_device
                .write(I2C_ADRESS, &[Register::Ctrl as u8, gain_ctrl])
                .await
                .is_err()
            {
                return;
            }

            let mut measurement_ctrl: u8 = 0;
            measurement_ctrl |= config.integration_time as u8;
            measurement_ctrl |= config.measurement_rate as u8;

            debug!(
                "[LTR-303ALS-01] Configuring integration time to {:?} and measurement rate to {:?}. Complete byte is {}",
                Debug2Format(&config.integration_time),
                Debug2Format(&config.measurement_rate),
                Hex(&[measurement_ctrl]),
            );

            if i2c_device
                .write(I2C_ADRESS, &[Register::MeasureCtrl as u8, measurement_ctrl])
                .await
                .is_err()
            {
                return;
            }

            let [up_th_low, up_th_high] = config.upper_threshold.to_le_bytes();
            let [lo_th_low, lo_th_high] = config.lower_threshold.to_le_bytes();

            debug!(
                "[LTR-303ALS-01] Setting upper threshold to {}",
                Hex(&config.upper_threshold.to_be_bytes())
            );

            if i2c_device
                .write(I2C_ADRESS, &[Register::UpperThresholdLow as u8, up_th_low])
                .await
                .is_err()
            {
                return;
            }
            if i2c_device
                .write(
                    I2C_ADRESS,
                    &[Register::UpperThresholdHigh as u8, up_th_high],
                )
                .await
                .is_err()
            {
                return;
            }

            debug!(
                "[LTR-303ALS-01] Setting lower threshold to {}",
                Hex(config.lower_threshold.to_be_bytes())
            );

            if i2c_device
                .write(I2C_ADRESS, &[Register::LowerThresholdLow as u8, lo_th_low])
                .await
                .is_err()
            {
                return;
            }

            if i2c_device
                .write(
                    I2C_ADRESS,
                    &[Register::LowerThresholdHigh as u8, lo_th_high],
                )
                .await
                .is_err()
            {
                return;
            }

            let _ = self.i2c.init(Mutex::new(i2c_device));

            self.state.set(State::Enabled);
        }
        debug!("[LTR-303ALS-01] Initilization successful");
    }

    /// Sets the configuration used by the sensor to the one provided by the user
    ///
    /// # Errors
    ///
    /// Returns `Err(())` when the I2C communication fails for any reasons.
    pub async fn set_config(&self, config: Config) -> Result<(), ()> {
        let mut i2c = self.i2c.get().await.lock().await;

        debug!("[LTR-303ALS-01] Changing configuration");

        let mut gain_ctrl: u8 = 0;
        gain_ctrl |= config.gain as u8;

        debug!(
            "[LTR-303ALS-01] Configuring gain to {:?}",
            Debug2Format(&config.gain)
        );

        i2c.write(I2C_ADRESS, &[Register::Ctrl as u8, gain_ctrl])
            .await
            .map_err(|_| ())?;

        let mut measurement_ctrl: u8 = 0;
        measurement_ctrl |= config.integration_time as u8;
        measurement_ctrl |= config.measurement_rate as u8;

        debug!(
            "[LTR-303ALS-01] Configuring integration time to {:?} and measurement rate to {:?}. Complete byte is {}",
            Debug2Format(&config.integration_time),
            Debug2Format(&config.measurement_rate),
            Hex(&[measurement_ctrl]),
        );

        i2c.write(I2C_ADRESS, &[Register::MeasureCtrl as u8, measurement_ctrl])
            .await
            .map_err(|_| ())?;

        let [up_th_low, up_th_high] = config.upper_threshold.to_le_bytes();
        let [lo_th_low, lo_th_high] = config.lower_threshold.to_le_bytes();

        debug!(
            "[LTR-303ALS-01] Setting upper threshold to {}",
            Hex(&config.upper_threshold.to_be_bytes())
        );

        i2c.write(I2C_ADRESS, &[Register::UpperThresholdLow as u8, up_th_low])
            .await
            .map_err(|_| ())?;

        i2c.write(
            I2C_ADRESS,
            &[Register::UpperThresholdHigh as u8, up_th_high],
        )
        .await
        .map_err(|_| ())?;

        debug!(
            "[LTR-303ALS-01] Setting lower threshold to {}",
            Hex(config.lower_threshold.to_be_bytes())
        );

        i2c.write(I2C_ADRESS, &[Register::LowerThresholdLow as u8, lo_th_low])
            .await
            .map_err(|_| ())?;

        i2c.write(
            I2C_ADRESS,
            &[Register::LowerThresholdHigh as u8, lo_th_high],
        )
        .await
        .map_err(|_| ())?;

        // TODO: Support the interrupts

        Ok(())
    }

    /// Listens for measurement requests generated by [`Ltr303Als01::trigger_measurement()`], and
    /// responds to them.
    /// This should be called before [`Ltr303Als01::wait_for_reading()`], as that method will otherwise
    /// not be able to respond to measurement requests from [`Ltr303Als01::trigger_measurement()`].
    ///
    /// # Note
    ///
    /// [`Ltr303Als01::init()`] needs to be called and `await`ed before calling this method.
    pub async fn run(&'static self) -> ! {
        loop {
            self.signaling.wait().await;

            self.reading.signal(self.measure().await);
        }
    }

    /// Triggers a one-shot measurement and asynchronously return the readings when available
    ///
    /// # Errors
    ///
    /// Returns `ReadingError::SensorAccess` in case of a communication error wth the
    /// device.
    pub async fn measure(&'static self) -> ReadingResult<Samples> {
        let mut i2c = self.i2c.get().await.lock().await;

        debug!("[LTR-303ALS-01] Activate sensor");

        let mut buf = [0u8];
        i2c.write_read(I2C_ADRESS, &[Register::Ctrl as u8], &mut buf)
            .await
            .map_err(|_| ReadingError::SensorAccess)?;
        let mut gain_ctrl = buf[0];
        gain_ctrl |= crate::ALS_MODE;

        i2c.write(I2C_ADRESS, &[Register::Ctrl as u8, gain_ctrl])
            .await
            .map_err(|_| ReadingError::SensorAccess)?;

        Timer::after_millis(crate::WAKEUP_TIME_MS).await;

        // Wait for new data to be there
        let status_reg = loop {
            let mut buf = [0u8];
            i2c.write_read(I2C_ADRESS, &[Register::StatusReg as u8], &mut buf)
                .await
                .map_err(|_| ReadingError::SensorAccess)?;

            // New data is available
            if buf[0] & crate::ALS_DATA_STATUS != 0 {
                // The data is valid
                if buf[0] & crate::ALS_DATA_VALIDITY == 0 {
                    break buf[0];
                }
                // The new data is invalid.
                // TODO: read the config to know how much to wait for the measurement
                Timer::after_millis(100).await;
            }
            Timer::after_millis(20).await;
        };

        debug!("[LTR-303ALS-01] Reading the raw lux values");
        let mut buf = [0u8; 4];
        i2c.write_read(I2C_ADRESS, &[Register::Channel1Low as u8], &mut buf)
            .await
            .map_err(|_| ReadingError::SensorAccess)?;

        let ch1 = u16::from_le_bytes([buf[0], buf[1]]);
        let ch0 = u16::from_le_bytes([buf[2], buf[3]]);

        debug!(
            "[LTR-303ALS-01] ch1: {}, ch0: {}",
            Hex(&buf[..2]),
            Hex(&buf[2..])
        );

        let used_gain = MeasuredAlsDataGain::from_status_reg(status_reg);

        // TODO: just store the configuration somewhere so as to not have to read it back from the sensor
        let mut buf_int_time = [0u8];
        i2c.write_read(
            I2C_ADRESS,
            &[Register::MeasureCtrl as u8],
            &mut buf_int_time,
        )
        .await
        .map_err(|_| ReadingError::SensorAccess)?;
        let used_int_time = AlsIntegrationTimeMs::from_measure_ctrl_reg(buf_int_time[0]);

        let sample = crate::physical_lux_from_raw(ch1, ch0, used_gain, used_int_time);

        let samples = Samples::from_1(self, [sample]);

        debug!("[LTR-303ALS-01] De-Activate sensor");

        let mut buf = [0u8];
        i2c.write_read(I2C_ADRESS, &[Register::Ctrl as u8], &mut buf)
            .await
            .map_err(|_| ReadingError::SensorAccess)?;
        let mut gain_ctrl = buf[0];
        gain_ctrl &= !crate::ALS_MODE;

        i2c.write(I2C_ADRESS, &[Register::Ctrl as u8, gain_ctrl])
            .await
            .map_err(|_| ReadingError::SensorAccess)?;

        Ok(samples)
    }
}

impl<I2C: Send> Sensor for Ltr303Als01<I2C> {
    fn trigger_measurement(&self) -> Result<(), TriggerMeasurementError> {
        self.reading.clear();

        match self.state.get() {
            State::Measuring => {}
            State::Enabled => {
                self.state.set(State::Measuring);
            }
            State::Uninitialized | State::Disabled | State::Sleeping => {
                return Err(TriggerMeasurementError::NonEnabled);
            }
        }

        self.signaling.signal(());

        Ok(())
    }

    fn wait_for_reading(&'static self) -> ReadingWaiter {
        match self.state.get() {
            State::Measuring => {
                self.state.set(State::Enabled);

                ReadingWaiter::new(self.reading.wait())
            }
            State::Enabled => ReadingWaiter::new_err(ReadingError::NotMeasuring),
            State::Uninitialized | State::Disabled | State::Sleeping => {
                ReadingWaiter::new_err(ReadingError::NonEnabled)
            }
        }
    }

    fn set_mode(&self, mode: SensorMode) -> Result<State, SetModeError> {
        self.state.set_mode(mode)
    }

    fn state(&self) -> State {
        self.state.get()
    }

    fn categories(&self) -> &'static [Category] {
        &[Category::Light]
    }

    fn reading_channels(&self) -> ReadingChannels {
        ReadingChannels::from([ReadingChannel::new(
            Label::Heading,
            -2,
            MeasurementUnit::Lux,
        )])
    }

    fn label(&self) -> Option<&'static str> {
        self.label
    }

    fn display_name(&self) -> Option<&'static str> {
        Some("ambient light sensor")
    }

    fn part_number(&self) -> Option<&'static str> {
        Some(PART_NUMBER)
    }

    fn version(&self) -> u8 {
        0
    }
}
