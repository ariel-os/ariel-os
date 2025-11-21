use embassy_stm32::rng::Rng;
use embassy_stm32::{bind_interrupts, peripherals, rng};

#[cfg(not(any(
    capability = "hw/stm32-aes-rng",
    capability = "hw/stm32-aes-rng-lpuart1",
    capability = "hw/stm32-hash-rng",
    capability = "hw/stm32-rng",
    capability = "hw/stm32-rng-cryp",
    capability = "hw/stm32-rng-lpuart1",
)))]
compile_error!("no stm32 RNG capability selected");

bind_interrupts!(struct Irqs {
    #[cfg(capability = "hw/stm32-aes-rng")]
    AES_RNG => rng::InterruptHandler<peripherals::RNG>;
    #[cfg(capability = "hw/stm32-aes-rng-lpuart1")]
    AES_RNG_LPUART1 => rng::InterruptHandler<peripherals::RNG>;
    #[cfg(capability = "hw/stm32-hash-rng")]
    HASH_RNG => rng::InterruptHandler<peripherals::RNG>;
    #[cfg(capability = "hw/stm32-rng")]
    RNG => rng::InterruptHandler<peripherals::RNG>;
    #[cfg(capability = "hw/stm32-rng-cryp")]
    RNG_CRYP => rng::InterruptHandler<peripherals::RNG>;
    #[cfg(capability = "hw/stm32-rng-lpuart1")]
    RNG_LPUART1 => rng::InterruptHandler<peripherals::RNG>;
});

pub fn construct_rng(peripherals: &mut crate::OptionalPeripherals) {
    let p = peripherals
        .RNG
        .take()
        .expect("RNG has not been previously used");

    let mut rng = Rng::new(p, Irqs);

    ariel_os_random::construct_rng(&mut ariel_os_random::RngAdapter(&mut rng));
}
