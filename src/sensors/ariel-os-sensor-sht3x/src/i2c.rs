//! Driver for the sensor used over I2C.

use ariel_os_sensors::{
    Category, Label, MeasurementUnit, Sensor,
    sensor::{
        Mode as SensorMode, ReadingChannel, ReadingChannels, ReadingError, ReadingResult,
        ReadingWaiter, Sample, Samples, SetModeError, State, TriggerMeasurementError,
    },
    signal::Signal as ReadingSignal,
};
use ariel_os_sensors_utils::AtomicState;
use embassy_sync::{
    blocking_mutex::raw::CriticalSectionRawMutex, mutex::Mutex, once_lock::OnceLock, signal::Signal,
};
use embassy_time::Timer;
use embedded_hal_async::i2c::{I2c, NoAcknowledgeSource::Data, ErrorKind as I2CErrorKind, Error as I2CError};
use portable_atomic::{AtomicU8, Ordering};

/// I2C address of the sensor device.
#[derive(Debug, Copy, Clone, PartialEq, Eq, Default)]
pub enum I2cAddress {
    /// The ADDR pin is connected to logic high.
    AddrLogicHigh = 0x45,
    /// The ADDR pin is connected to logic low.
    #[default]
    AddrLogicLow = 0x44,
}

/// Configuration of the sensor driver and device.
#[derive(Debug, Default)]
#[non_exhaustive]
pub struct Config {
    /// I2C address to use.
    pub address: I2cAddress,
    // TODO: Support other acquisition modes
}


ariel_os_hal::define_peripherals!(
    /// Peripherals required by the sensor driver.
    Peripherals {}
);

/// Driver to use an SHT3X over I2C.
pub struct Sht3x<I2C> {
    state: AtomicState,
    label: Option<&'static str>,
    i2c: OnceLock<Mutex<CriticalSectionRawMutex, I2C>>,
    address: AtomicU8,
    signaling: Signal<CriticalSectionRawMutex, ()>,
    reading: ReadingSignal<ReadingResult<Samples>>,
}

impl<I2C: I2c + Send> Sht3x<I2C> {
    /// Creates an uninitialized driver.
    #[expect(clippy::new_without_default)]
    #[must_use]
    pub const fn new(label: Option<&'static str>) -> Self {
        Self {
            state: AtomicState::new(State::Uninitialized),
            label,
            i2c: OnceLock::new(),
            address: AtomicU8::new(I2cAddress::AddrLogicLow as u8),
            signaling: Signal::new(),
            reading: ReadingSignal::new(),
        }
    }

    /// Initializes the driver.
    pub async fn init(
        &'static self,
        _peripherals: Peripherals,
        mut i2c_device: I2C,
        config: Config,
    ) {
        if !self.i2c.is_set() {
            self.address.store(config.address as u8, Ordering::Release);

            if Self::reset(&mut i2c_device, config.address).await.is_err() {
                return;
            }

            let _ = self.i2c.init(Mutex::new(i2c_device));

            self.state.set(State::Enabled);
        }
    }

    /// Soft resets the device
    ///
    /// # Errors
    /// When the I2C connection fails
    async fn reset(i2c_device: &mut I2C, address: I2cAddress) -> Result<(), ()> {
        i2c_device
            .write(
                address as u8,
                &(crate::Command::SoftReset as u16).to_be_bytes()
            )
            .await
            .map_err(|_| ())?;
        Ok(())
    }

    /// Turns on the onboard heater and check that it is indeed turned on
    ///
    /// # Errors
    /// When the I2C connection fails for any reason
    /// When the CRC of the status register is incorrect
    /// When the heater status bit of the status register doesn't show the heater as on
    ///
    pub async fn enable_heater(&'static self) -> Result<(), ()> {
        let mut i2c = self.i2c.get().await.lock().await;
        let address = self.address.load(Ordering::Acquire);

        // Turn on the heater
        i2c.write(address, &(crate::Command::HeaterEnable as u16).to_be_bytes())
            .await
            .map_err(|_| ())?;

        Timer::after_millis(1).await;

        // Read the status register + CRC
        let mut buf = [0u8; 3];
        let i = i2c.write_read(
                address,
                &(crate::Command::ReadStatusReg as u16).to_be_bytes(),
                &mut buf
            ).await;
        i.map_err(|_| ())?;
        let reg_buf = [buf[0], buf[1]];

        if !crate::check_crc(&reg_buf, buf[2]) {
            return Err(());
        }
        let reg = u16::from_be_bytes(reg_buf);

        if reg & crate::HEATER_STATUS == 0 {
            Err(())
        }
        else {
            Ok(())
        }
    }

    /// Reads the status register and return it as a u16
    ///
    /// # Errors
    /// When the I2C connection fails for any reason
    /// When the CRC of the status register is incorrect
    ///
    pub async fn read_status(&'static self) -> Result<u16, ()> {
        let mut i2c = self.i2c.get().await.lock().await;
        let address = self.address.load(Ordering::Acquire);

        // Read the status register including CRC;
        let mut buf = [0u8; 3];
        i2c.write_read(
                address,
                &(crate::Command::ReadStatusReg as u16).to_be_bytes(),
                &mut buf
            ).await
            .map_err(|_| ())?;

        let reg_buf = [buf[0], buf[1]];

        if !crate::check_crc(&reg_buf, buf[2]) {
            return Err(());
        }

        let reg = u16::from_be_bytes(reg_buf);
        return Ok(reg)
    }

    /// Turns off the onboard heater and check that it is indeed turned off
    ///
    /// # Errors
    /// When the I2C connection fails for any reason
    /// When the CRC of the status register is incorrect
    /// When the heater status bit of the status register doesn't show the heater as off
    ///
    pub async fn disable_heater(&'static self) -> Result<(), ()> {
        let mut i2c = self.i2c.get().await.lock().await;
        let address = self.address.load(Ordering::Acquire);

        // Turn off the heater
        i2c.write(address, &(crate::Command::HeaterDisable as u16).to_be_bytes())
            .await
            .map_err(|_| ())?;

        Timer::after_millis(1).await;

        // Read the status register
        let mut buf = [0u8; 3];
        let i = i2c.write_read(
                address,
                &(crate::Command::ReadStatusReg as u16).to_be_bytes(),
                &mut buf
            ).await;

        let reg_buf = [buf[0], buf[1]];

        if !crate::check_crc(&reg_buf, buf[2]) {
            return Err(());
        }

        let reg = u16::from_be_bytes(reg_buf);
        // Check bit 13
        if reg & crate::HEATER_STATUS != 0 {
            Err(())
        }
        else {
            Ok(())
        }
    }

    /// Listens for measurement requests generated by [`Sht3x::trigger_measurement()`], and
    /// responds to them.
    /// This should be called before [`Sht3x::wait_for_reading()`], as that method will otherwise
    /// not be able to respond to measurement requests from [`Sht3x::trigger_measurement()`].
    ///
    /// # Note
    ///
    /// [`Sht3x::init()`] needs to be called and `await`ed before calling this method.
    pub async fn run(&'static self) -> ! {
        loop {
            self.signaling.wait().await;

            self.reading.signal(self.measure().await);
        }
    }

    /// Trigger a measurement and process the results
    ///
    /// # Errors
    /// When the I2C connection fails
    /// When the CRC of either the temperature or the relative humidity are incorrect
    ///
    async fn measure(&'static self) -> ReadingResult<Samples> {
        let mut i2c = self.i2c.get().await.lock().await;
        let address = self.address.load(Ordering::Acquire);

        // Trigger a one-shot measurement.
        i2c.write(address, &(crate::Command::SingleShotDisabledMedium as u16).to_be_bytes())
            .await
            .map_err(|_| ReadingError::SensorAccess)?;
        // TODO: Support other measurement options

        // Wait for the measurement.
        Timer::after_millis(crate::MEDIUM_REPEAT_DELAY).await;
        let mut buf = [0u8; 6];
        loop {
            match i2c.read(address, &mut buf).await {
                // Busy
                Err(e) if e.kind() == I2CErrorKind::NoAcknowledge(Data) => { }
                Err(_) => {
                    return Err(ReadingError::SensorAccess);
                }
                Ok(()) => {
                    break;
                }
            }
            // This shouldn't actually be needed since we wait before
            Timer::after_millis(1).await;
        }
        // buf now contains |temp_1|temp_2|temp_crc|humi_1|humi_2|humi_crc|
        let temp_buf = [buf[0], buf[1]];
        if ! crate::check_crc(&temp_buf, buf[2]) {
            // FIXME: Use a better error type
            return Err(ReadingError::SensorAccess)
        }

        let humi_buf = [buf[3], buf[4]];
        if !crate::check_crc(&humi_buf, buf[5]) {
            // FIXME: Use a better error type
            return Err(ReadingError::SensorAccess)
        }

        // FIXME: Find a way to not use floats
        let temp_raw = u16::from_be_bytes(temp_buf);
        let temp_float = f32::from(temp_raw) * 175.0 / 65535.0 - 45.0;
        let temp = (100.0 * temp_float) as i32;

        let humi_raw = u16::from_be_bytes(humi_buf);
        let humi_float = f32::from(humi_raw) * 100.0 / 65535.0;
        let humi = (100.0 * humi_float) as i32;

        let t_accuracy = crate::t_accuracy(temp);
        let sample_temp = Sample::new(temp, t_accuracy);

        let h_accuracy = crate::h_accuracy(humi);
        let sample_humi = Sample::new(humi, h_accuracy);

        let samples = Samples::from_2(self, [sample_temp, sample_humi]);

        Ok(samples)
    }
}

impl<I2C: Send> Sensor for Sht3x<I2C> {
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
            State::Enabled => {
                ReadingWaiter::new_err(ReadingError::NotMeasuring)
            }
            State::Uninitialized | State::Disabled | State::Sleeping => {
                ReadingWaiter::new_err(ReadingError::NonEnabled)
            }
        }
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
        &[Category::RelativeHumidityTemperature]
    }

    fn reading_channels(&self) -> ReadingChannels {
        ReadingChannels::from([
            ReadingChannel::new(
                Label::Temperature,
                -2,
                MeasurementUnit::Celsius),
            ReadingChannel::new(
                Label::RelativeHumidity,
                -2,
                MeasurementUnit::PercentageRelativeHumidity,
            ),
        ])
    }

    fn label(&self) -> Option<&'static str> {
        self.label
    }

    fn display_name(&self) -> Option<&'static str> {
        Some("temperature/humidity sensor")
    }

    fn part_number(&self) -> Option<&'static str> {
        None
    }

    fn version(&self) -> u8 {
        0
    }
}
