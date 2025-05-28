//! Helpers for using [`defmt_or_log`]

/// A newtype around byte slices used for all of Ariel OS's logging facades that prefers
/// interpreting the data as CBOR.
///
/// Its preferred output is CBOR Diagnostic Notation (EDN), but showing hex is also acceptable.
///
/// Instead of writing some variation of `info!("Found bytes {:cbor}", item)`, you can write
/// `info!("Found bytes {}", Cbor(item))`.
///
/// Note that using this wrapper is not necessary when using a
/// [`cboritem::CborItem`](https://docs.rs/cboritem/latest/cboritem/struct.CborItem.html) as it
/// already does something similar on its own.
pub struct Cbor<T: AsRef<[u8]>>(pub T);

impl<T: AsRef<[u8]>> core::fmt::Display for Cbor<T> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "{:02x?}", self.0.as_ref())
    }
}

#[cfg(feature = "defmt")]
impl<T: AsRef<[u8]>> defmt::Format for Cbor<T> {
    fn format(&self, f: defmt::Formatter) {
        ::defmt::write!(f, "{=[u8]:cbor}", self.0.as_ref())
    }
}
