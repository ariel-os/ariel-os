#![expect(unsafe_code)]
#![expect(
    clippy::undocumented_unsafe_blocks,
    reason = "should be addressed eventually"
)]

use ariel_os_embassy_common::gpio::input::InterruptError;
use embassy_stm32::{OptionalPeripherals, Peri, exti::AnyChannel, gpio::Pin, peripherals};
use portable_atomic::{AtomicBool, AtomicU16, Ordering};

pub static EXTINT_REGISTRY: ExtIntRegistry = ExtIntRegistry::new();

pub struct ExtIntRegistry {
    initialized: AtomicBool,
    used_interrupt_channels: AtomicU16, // 16 channels
}

impl ExtIntRegistry {
    // Collect all channel peripherals so that the registry is the only one managing them.
    const fn new() -> Self {
        Self {
            initialized: AtomicBool::new(false),
            used_interrupt_channels: AtomicU16::new(0),
        }
    }

    #[expect(clippy::missing_panics_doc)]
    pub fn init(&self, peripherals: &mut OptionalPeripherals) {
        peripherals.EXTI0.take().unwrap();
        peripherals.EXTI1.take().unwrap();
        peripherals.EXTI2.take().unwrap();
        peripherals.EXTI3.take().unwrap();
        peripherals.EXTI4.take().unwrap();
        peripherals.EXTI5.take().unwrap();
        peripherals.EXTI6.take().unwrap();
        peripherals.EXTI7.take().unwrap();
        peripherals.EXTI8.take().unwrap();
        peripherals.EXTI9.take().unwrap();
        peripherals.EXTI10.take().unwrap();
        peripherals.EXTI11.take().unwrap();
        peripherals.EXTI12.take().unwrap();
        peripherals.EXTI13.take().unwrap();
        peripherals.EXTI14.take().unwrap();
        peripherals.EXTI15.take().unwrap();

        self.initialized.store(true, Ordering::Release);

        // Do nothing else, just consume the peripherals: they are ours now!
    }

    /// # Errors
    ///
    /// Returns `Err(InterruptError::IntChannelAlreadyUsed)` if the interrupt channel is already in
    /// use.
    ///
    /// # Panics
    ///
    /// Will panic if the interrupt channels have not been captured during initialization.
    pub fn get_interrupt_channel_for_pin<T: Pin>(
        &self,
        pin: &Peri<'_, T>,
    ) -> Result<Peri<'_, AnyChannel>, InterruptError> {
        // Make sure that the interrupt channels have been captured during initialization.
        assert!(self.initialized.load(Ordering::Acquire));

        let pin_number = pin.pin();

        // As interrupt channels are mutually exclusive between ports (ie., if channel i has
        // been bound for pin i of a port, it cannot be used for pin i of another port), we
        // only check the pin number.
        // NOTE(ordering): since setting a bit is an idempotent operation, and since we do not
        // allow clearing them, the ordering does not matter.
        let was_used = self
            .used_interrupt_channels
            .bit_set(pin_number.into(), Ordering::Relaxed);

        if was_used {
            return Err(InterruptError::IntChannelAlreadyUsed);
        }

        // They are the same
        let ch_number = pin_number;

        // NOTE(embassy): ideally we would be using `T::ExtiChannel::steal()` instead of this
        // match, but Embassy does not provide this.
        // SAFETY: this function enforces that the same channel cannot be obtained twice,
        // making sure multiple instances are not used at the same time as the mandatory
        // `init()` method has collected all channel peripherals beforehand.
        let ch = match ch_number {
            0 => unsafe { peripherals::EXTI0::steal() }.into(),
            1 => unsafe { peripherals::EXTI1::steal() }.into(),
            2 => unsafe { peripherals::EXTI2::steal() }.into(),
            3 => unsafe { peripherals::EXTI3::steal() }.into(),
            4 => unsafe { peripherals::EXTI4::steal() }.into(),
            5 => unsafe { peripherals::EXTI5::steal() }.into(),
            6 => unsafe { peripherals::EXTI6::steal() }.into(),
            7 => unsafe { peripherals::EXTI7::steal() }.into(),
            8 => unsafe { peripherals::EXTI8::steal() }.into(),
            9 => unsafe { peripherals::EXTI9::steal() }.into(),
            10 => unsafe { peripherals::EXTI10::steal() }.into(),
            11 => unsafe { peripherals::EXTI11::steal() }.into(),
            12 => unsafe { peripherals::EXTI12::steal() }.into(),
            13 => unsafe { peripherals::EXTI13::steal() }.into(),
            14 => unsafe { peripherals::EXTI14::steal() }.into(),
            15 => unsafe { peripherals::EXTI15::steal() }.into(),
            _ => unreachable!(),
        };

        Ok(ch)
    }
}
