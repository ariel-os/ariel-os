//! Access to unique identifiers provided by the device.
//!
//! This module provides [`device_id_bytes()`] and related functions, which returns an identifier for the
//! concrete piece of hardware that the software is running on in byte serialized form.
//!
//! Concrete properties of a device identity are:
//!
//! * Identifiers are reasonably unique: They are either unique by construction (serial number, MAC
//!   address) or random identifiers (>= 64 bit).
//!
//! * The scope of the identifier is within an Ariel OS board. Their scope may be broader, eg. when
//!   a identifier is unique per MCU family, or even globally.
//!
//! * Identifiers do not change during regular development with a device, which includes the use of
//!   a programmer. Identifiers may change under deliberate conditions, eg. when a device has a
//!   one-time programmable identity, or when there is a custom functionality to overwrite the
//!   built-in identifier that is not triggered by the device erase that is performed as part of
//!   programming the device.
//!
//! Constructing an identifier fails rather than produce a dummy identifier.
//!
//! It is considered a breaking change in Ariel OS if a device's identifier changes or becomes an
//! error. Errors changing to valid identifiers is a compatible change.
//!
//! A device can have more identifying properties that are also handled through this module, such
//! as the EUI-48 addresses provided by [`interface_eui48()`], or (as examples of future
//! development) per-device UUIDs or an IDevID. Those identifiers might have different stability
//! guarantees, and may be filled with identifiers generated based on the [`device_id_bytes()`].
#![no_std]
#![deny(missing_docs)]
// required for tests:
#![cfg_attr(test, no_main)]

pub use ariel_os_embassy_common::identity::Eui48;

/// Obtains a unique identifier of the device in its byte serialized form.
///
/// See module level documentation for that identifier's properties.
///
/// # Errors
///
/// This function's errors are device dependent, and range from the function being infallibly typed
/// (where a device ID is accessible through memory mapped access) over bus errors (where a chip in
/// a faulty state might impede communication with an identity EEPROM) to erring unconditionally
/// (where no device ID is present or implemented).
pub fn device_id_bytes() -> Result<impl AsRef<[u8]>, impl core::error::Error> {
    use ariel_os_embassy_common::identity::DeviceId as _;

    ariel_os_hal::hal::identity::DeviceId::get().map(|d| d.bytes())
}

/// Generates an EUI-48 identifier ("6-byte MAC address") based on the device identity.
///
/// The argument `if_index` allows the system to generate addresses for consecutive interfaces.
///
/// The default implementation creates a static random identifier based on the board name, the
/// device ID bytes, incrementing by the interface index (following the common scheme of
/// sequential MAC addresses being assigned to multi-interface hardware). Those take the shape
/// `?2-??-??-??-??-??`: Their bits set to individual (I/G, unicast), administratively
/// locally administered (U/L, not indicating any particular manufacturer), and following the
/// SLAP (Structured Local Address Plan) semantics, they fall into the AII (Administratively
/// Assigned Identifier) quadrant. Wikipedia has a [good description of those address
/// details](https://en.wikipedia.org/wiki/MAC_address#Address_details).
///
/// The randomly generated identifiers aim to appear random, but can
/// be traced back to the device ID it is calculated from.
///
/// On devices that have access to globally unique EUI-48 identifiers, those are returned
/// for interface indices up to the number of available identifiers.
///
/// # Errors
///
/// Same as in [`device_id_bytes()`].
pub fn interface_eui48(if_index: u32) -> Result<Eui48, impl core::error::Error> {
    use ariel_os_embassy_common::identity::DeviceId as _;

    #[expect(
        clippy::question_mark,
        reason = "false positive: then the compiler couldn't infer the type of our error if the error went through `?`"
    )]
    let devid = match ariel_os_hal::hal::identity::DeviceId::get() {
        Ok(i) => i,
        Err(e) => return Err(e),
    };

    if let Some(eui) = devid.interface_eui48(if_index) {
        return Ok(eui);
    }

    Ok(generate_fallback_eui48(&devid, if_index))
}

/// Generates a fallback EUI-48 out of a board name, a [`DeviceId`], and an interface index.
fn generate_fallback_eui48(
    devid: &impl ariel_os_embassy_common::identity::DeviceId,
    if_index: u32,
) -> Eui48 {
    // Not even trying to hash for privacy: Many CPU IDs just have 32 variable bits (eg. EFM32
    // with a 32bit timestamp in a limited range, and a 32bit factory ID, or STM32's 96 bit
    // containing lot and wafer numbers and coordinates), and all SHA256 hashes of 2^32
    // possibilities can just be calculated on a graphics card in an hour.
    //
    // We do hash the board identifier, just to be sure to have a nice and random-looking
    // starting point. The sha1 function was chosen because it is widespread (enabling
    // re-implementation of the algorithm outside to predict addresses), available in a const
    // implementation, and because its output is large enough to spread the input over the full
    // address; it is not expected to be secure.

    // Ideally, BOARD should be passed into generate_aai_mac_address, but that function needs
    // to make sure the hashing is const, and we do not have str typed const generics yet.
    const BOARD_HASH: [u8; 20] = const_sha1::sha1(ariel_os_buildinfo::BOARD.as_bytes()).as_bytes();
    let truncated_board_hash = const {
        *BOARD_HASH
            .first_chunk()
            .expect("EUI-48 is shorter than SHA1")
    };

    // We use the whole device identity to be sure to use the variable parts even if they are
    // just in some location (which may not be the first or the last bits of the bytes(), in
    // general).
    //
    // We make it influence the middle 4 bytes of the MAC address: This is easy to do, and
    // ensures that consecutive serial numbers (no matter in which byte) don't cause chip N
    // interface 1 to have the same MAC as chip N+1 interface 0. Thus we miss the opportunity
    // to influence 12 more bits (out of the 16, because 4 are fixed by construction of an
    // Administratively Assigned Identifier), but the simplicity makes up for it, and whoever
    // runs a risk of having a realistic chance of a MAC collision in 32 bit space will use
    // globally unique addresses or actually manage addresses anyway.

    let device_id_bytes = devid.bytes();

    generate_aai_mac_address(truncated_board_hash, device_id_bytes, if_index)
}

/// Assembles an EUI-48 by XOR'ing pieces.
///
/// Pieces that go in are:
/// - an adequately sized base value (in practice: a truncated hash of the board name),
/// - device ID bytes that are completely spread into some of the base value, and
/// - an interface index that gets added into it in the last bytes.
// To test how this optimizes, replace the device_id_bytes type with [u8; 10],
// and put it through godbolt.org with `-O --target thumbv8m.main-none-eabihf`. Only for [u8; 4]
#[expect(
    clippy::missing_panics_doc,
    reason = "False positive. Clippy does not see that `u32::from_le_bytes(eui48[1..5].try_into().unwrap())` can not panic, even though the compiler produces panic free code."
)]
fn generate_aai_mac_address(
    truncated_board_hash: [u8; 6],
    device_id_bytes: impl AsRef<[u8]>,
    if_index: u32,
) -> Eui48 {
    let mut eui48 = truncated_board_hash;

    // This alternative algorithm is identical to the next paragraph (as easily evidenced by
    // running both on arbitrary inputs) but rustc doesn't optimize this simple version:
    //
    // ```
    // for (index, byte) in device_id_bytes.as_ref().into_iter().enumerate() {
    //     eui48[1 + index % 4] ^= byte;
    // }
    // ```

    // This would work the same in either little and big endian, but most machines are little
    // these days (and Rust has no simple and safe host-endianness conversion).
    let mut xor_me: u32 = u32::from_le_bytes(eui48[1..5].try_into().unwrap());
    for chunk in device_id_bytes.as_ref().chunks(4) {
        let mut full = [0; 4];
        #[allow(
            clippy::indexing_slicing,
            reason = "Works by construction; the equivalent array chunks based construction with accessing the `.remainder` is neither stable nor equally concise"
        )]
        full[..chunk.len()].copy_from_slice(chunk);
        xor_me ^= u32::from_le_bytes(full);
    }
    eui48[1..5].copy_from_slice(&xor_me.to_le_bytes()[..]);

    // Enforce the `?2-??-??-??-??-??` pattern of an AII (Administratively Assigned Identifier)
    eui48[0] &= 0xf0;
    eui48[0] |= 0x02;

    // Once the hashing is done in here too, there is some optimization potential in making
    // everything above this line into a function and inlining the rest all the way into where the
    // interface identifier is set, because then constant propagation may eliminate this unaligned
    // thing, but let's delay that until someone measures it.

    let with_if_index = u32::from_be_bytes(eui48[2..6].try_into().unwrap()).wrapping_add(if_index);
    eui48[2..6].copy_from_slice(&(with_if_index).to_be_bytes()[..]);

    Eui48(eui48)
}

#[cfg(test)]
#[embedded_test::tests]
mod tests {
    #[test]
    async fn has_device_id() {
        let device_id = ariel_os::identity::device_id_bytes();

        if cfg!(capability = "hw/device-identity") {
            assert!(device_id.is_ok());
        } else {
            assert!(device_id.is_err());
        }
    }
}
