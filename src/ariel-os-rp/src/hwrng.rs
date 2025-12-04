pub fn construct_rng(peripherals: &mut crate::OptionalPeripherals) {
    #[cfg(context = "rp2040")]
    let mut hwrng = {
        let _ = peripherals; // Mark used

        // NOTE(datasheet): The RP2040 RNG "does not meet the requirements of randomness for
        // security systems because it can be compromised".
        embassy_rp::clocks::RoscRng
    };

    #[cfg(context = "rp235xa")]
    let mut hwrng = {
        embassy_rp::bind_interrupts!(struct Irqs {
            TRNG_IRQ => embassy_rp::trng::InterruptHandler<embassy_rp::peripherals::TRNG>;
        });

        let trng = peripherals.TRNG.take().unwrap();

        // Enable all entropy checks to ensure acceptable results.
        let mut config = embassy_rp::trng::Config::default();
        config.disable_autocorrelation_test = false;
        config.disable_crngt_test = false;
        config.disable_von_neumann_balancer = false;

        embassy_rp::trng::Trng::new(trng, Irqs, config)
    };

    ariel_os_random::construct_rng(&mut ariel_os_random::RngAdapter(&mut hwrng));
}
