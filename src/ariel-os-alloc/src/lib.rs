//! This crate glues a suitable allocator into Ariel OS.

#![no_std]
#![deny(missing_docs)]
#![expect(unsafe_code)]
// required for tests:
#![cfg_attr(test, no_main)]

// With embedded-test enabled, this crate gets built *twice*, once regularly
// as the system alloc it is supposed to be, and once as test application.
// In the latter case, `cfg(test)` is set.
// So we *only* set up the global stuff if *not* testing in order to avoid clashes.
#[cfg(not(test))]
pub use alloc::init;

#[cfg(not(test))]
mod alloc {
    const CONFIG_HEAPSIZE: usize =
        ariel_os_utils::usize_from_env_or!("CONFIG_HEAPSIZE", 2048, "heap size (in bytes)");

    /// Initializes the heap.
    ///
    /// This is called by `ariel-os-rt` early during system initialization.
    ///
    /// # Safety
    ///
    /// Call only once!
    pub unsafe fn init() {
        // SAFETY: Propagates the call-only-once requirement.
        unsafe {
            #[cfg(context = "cortex-m")]
            init_embedded_alloc();
            #[cfg(context = "esp")]
            init_esp_alloc();
            #[cfg(not(any(context = "cortex-m", context = "esp")))]
            init_none();
        }
    }

    /// Initializes an `embedded_alloc` heap.
    ///
    /// # Safety
    ///
    /// Call only once!
    #[cfg(context = "cortex-m")]
    unsafe fn init_embedded_alloc() {
        use ariel_os_debug::log::debug;

        use embedded_alloc::TlsfHeap as Heap;

        #[global_allocator]
        static HEAP: Heap = const { Heap::empty() };

        unsafe extern "C" {
            static __sheap: u32;
            static __eheap: u32;
        }

        let start = &raw const __sheap as usize;
        let size = &raw const __eheap as usize - start;

        // No `const { assert!(..) }` here unfortunately due to the use of linker
        // values.
        assert!(size >= CONFIG_HEAPSIZE);

        debug!(
            "ariel-os-alloc: initializing heap with {} bytes at 0x{:x}",
            size, start
        );

        unsafe { HEAP.init(start, size) }
    }

    /// Initializes an `esp_alloc` heap.
    ///
    /// # Safety
    ///
    /// Call only once!
    #[cfg(context = "esp")]
    unsafe fn init_esp_alloc() {
        use ariel_os_debug::log::debug;

        debug!(
            "ariel-os-alloc: initializing heap with {} bytes",
            CONFIG_HEAPSIZE
        );

        esp_alloc::heap_allocator!(CONFIG_HEAPSIZE);
    }

    /// Initializes **no** heap.
    ///
    /// Used as catch-all for tooling or when the heap is set-up elsewhere,
    /// as e.g., on "native".
    ///
    /// # Safety
    ///
    /// Not actually unsafe but we don't want the caller to get in trouble.
    #[cfg(not(any(context = "esp", context = "cortex-m")))]
    unsafe fn init_none() {
        // mark used
        let _ = CONFIG_HEAPSIZE;

        // compile-fail unless building docs etc.
        #[cfg(all(context = "ariel-os", not(context = "native")))]
        compile_error!("ariel-os-alloc: unsupported architecture!");
    }
}

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    #[test]
    async fn basic() {
        extern crate alloc;
        use alloc::vec::Vec;
        let i = 0xdeadbeefu32;

        let mut some_vec = Vec::new();
        some_vec.push(i);
        assert!(some_vec[0] == i);
    }
}
