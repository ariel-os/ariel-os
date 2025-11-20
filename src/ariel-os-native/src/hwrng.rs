/// Constructs the hardware random number generator (RNG).
pub fn construct_rng(_peripherals: &mut crate::OptionalPeripherals) {
    use rand::TryRngCore;
    // Going through unwraps because Ariel OS are not started so early in system startup that it
    // matters.
    ariel_os_random::construct_rng(&mut rand::rngs::OsRng.unwrap_mut());
}
