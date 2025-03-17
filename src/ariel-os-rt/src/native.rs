#[no_mangle]
pub unsafe extern "C" fn main() {
    crate::startup();
}

pub fn init() {}
