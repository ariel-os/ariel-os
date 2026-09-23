//! Provides helpers for access to memory regions.

#[cfg(feature = "memory-storage")]
use core::ops::Range;

/// Gets a [`Range`] from the linker that can be used for a global [`Storage`].
#[cfg(feature = "memory-storage")]
pub fn storage_range() -> Range<u32> {
    unsafe extern "C" {
        static _storage_start: u32;
        static _storage_length: u32;
    }

    let start = &raw const _storage_start as usize;
    let length = &raw const _storage_length as usize;

    #[expect(clippy::cast_possible_truncation)]
    let (start, end) = (start as u32, (start + length) as u32);

    start..end
}
