/// Constructs the hardware random number generator (RNG).
///
pub fn construct_rng(_peripherals: &mut crate::OptionalPeripherals) {
    cfg_if::cfg_if! {
        if #[cfg(context = "native")] {
            ariel_os_random::construct_rng(rand::rngs::OsRng::default());
        } else if #[cfg(context = "ariel-os")] {
            compile_error!("hardware RNG is not supported on this MCU family");
        }
    }
}
