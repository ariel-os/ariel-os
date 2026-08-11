//! Opt-in allocator wiring for PSRAM (external SPI RAM).
#![expect(unsafe_code, reason = "registers PSRAM as a heap region")]

/// Opt-in allocator for PSRAM (external SPI RAM).
///
/// This is a separate [`esp_alloc::EspHeap`] instance, deliberately kept off
/// the default global allocator (`esp_alloc::HEAP`). `esp_alloc::EspHeap`'s
/// `GlobalAlloc::alloc` allocates with an empty capability filter, which
/// matches *any* registered region regardless of its
/// [`esp_alloc::MemoryCapability`] tag — so a region registered on the
/// global heap is reachable from plain, unqualified `Box`/`Vec`/etc. calls
/// even if it is tagged `External`. Keeping PSRAM on its own heap guarantees
/// unqualified allocations can never land here, which matters because
/// atomic instructions do not work correctly on memory located in PSRAM on
/// ESP32, ESP32-S2 and ESP32-S3 (see `esp_alloc::psram_allocator!`'s
/// documentation).
///
/// Because `esp_alloc::EspHeap` implements `allocator_api2::alloc::Allocator`
/// unconditionally, code that wants to allocate specifically in PSRAM can
/// use this directly, e.g. `allocator_api2::boxed::Box::new_in(x, &PSRAM_HEAP)`
/// or `allocator_api2::vec::Vec::new_in(&PSRAM_HEAP)`.
///
/// Empty (no regions registered) until [`init`] runs.
pub static PSRAM_HEAP: esp_alloc::EspHeap = esp_alloc::EspHeap::empty();

/// Registers `psram`'s raw memory as the sole region of [`PSRAM_HEAP`].
///
/// Takes the `PSRAM` peripheral by value rather than requiring the caller to
/// justify a `# Safety` contract: `esp_hal::init()` is the only way to
/// obtain a `PSRAM` token, and it runs at most once per program (from
/// `crate::init()`), so receiving one here is itself proof that this region
/// isn't already in use elsewhere.
pub(crate) fn init(psram: esp_hal::peripherals::PSRAM<'_>) {
    let (start, size) = esp_hal::psram::psram_raw_parts(&psram);
    // SAFETY: `psram` is owned by this function, and the only source of a
    // `PSRAM` token — `esp_hal::init()` — runs at most once per program, so
    // this region is exclusively ours for the remainder of the program;
    // `size` is non-zero whenever PSRAM hardware is actually present.
    unsafe {
        PSRAM_HEAP.add_region(esp_alloc::HeapRegion::new(
            start,
            size,
            esp_alloc::MemoryCapability::External.into(),
        ));
    }
}
