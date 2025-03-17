use std::time::{Duration, Instant};

use crate::Error;

#[allow(missing_docs)]
#[expect(clippy::missing_errors_doc)]
pub fn benchmark<F: FnMut()>(iterations: usize, mut f: F) -> Result<usize, Error> {
    let before = Instant::now();

    for _ in 0..iterations {
        f();
    }

    // SysTick is downcounting, so `before - after` is correct.
    let total = before.elapsed().as_nanos();

    Ok(total as usize / iterations)
}
