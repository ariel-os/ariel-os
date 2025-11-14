use crate::irqs::Irqs;

/// Constructs the hardware random number generator (RNG) for the nRF family.
///
/// # Panics
///
/// Panics if the RNG peripheral has been previously used/taken.
pub fn construct_rng(peripherals: &mut crate::OptionalPeripherals) {
    cfg_if::cfg_if! {
        // The union of all contexts that wind up in a construct_rng should be synchronized
        // with laze-project.yml's hwrng module.
        if #[cfg(any(context = "nrf51", context = "nrf52", context = "nrf5340-net"))] {
            let p =
                peripherals
                    .RNG
                    .take()
                    .expect("RNG has not been previously used");

            let mut rng = embassy_nrf::rng::Rng::new(p, Irqs);

            ariel_os_random::construct_rng(&mut ariel_os_random::RngAdapter(&mut rng));
        } else if #[cfg(context = "ariel-os")] {
            compile_error!("hardware RNG is not supported on this MCU family");
        }
    }
}
