/// Implement `RngCore` on top of an older `getrandom` crate.
struct Getrandom02Rng;

impl rand::rand_core::RngCore for Getrandom02Rng {
    fn next_u32(&mut self) -> u32 {
        let mut buf = [0; 4];
        self.fill_bytes(&mut buf);
        u32::from_le_bytes(buf)
    }

    fn next_u64(&mut self) -> u64 {
        let mut buf = [0; 8];
        self.fill_bytes(&mut buf);
        u64::from_le_bytes(buf)
    }

    fn fill_bytes(&mut self, dest: &mut [u8]) {
        if let Err(e) = getrandom::getrandom(dest) {
            panic!("Error: {}", e);
        }
    }
}

/// Constructs the hardware random number generator (RNG).
pub fn construct_rng(_peripherals: &mut crate::OptionalPeripherals) {
    ariel_os_random::construct_rng(&mut Getrandom02Rng);
}
