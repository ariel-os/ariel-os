//! Expressions for access policy as evaluated for a particular security context.

use coap_message::{MessageOption, ReadableMessage};

/// A data item representing the server access policy as evaluated for a particular security context.
pub trait Scope: Sized + core::fmt::Debug {
    /// Returns true if a request may be performed by the bound security context.
    fn request_is_allowed<M: ReadableMessage>(&self, request: &M) -> bool;

    /// Returns true if a bound security context should be preferably retained when hitting
    /// resource limits.
    fn is_admin(&self) -> bool {
        false
    }
}

impl Scope for core::convert::Infallible {
    fn request_is_allowed<M: ReadableMessage>(&self, _request: &M) -> bool {
        match *self {}
    }
}

/// Error type indicating that a scope could not be created from the given token scope.
///
/// As tokens are only accepted from trusted sources, the presence of this error typically
/// indicates a misconfigured trust anchor.
#[derive(Debug, Copy, Clone)]
pub struct InvalidScope;

/// A scope expression that allows all requests.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct AllowAll;

impl Scope for AllowAll {
    fn request_is_allowed<M: ReadableMessage>(&self, _request: &M) -> bool {
        true
    }
}

/// A scope expression that denies all requests.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug)]
pub struct DenyAll;

impl Scope for DenyAll {
    fn request_is_allowed<M: ReadableMessage>(&self, _request: &M) -> bool {
        false
    }
}

const AIF_SCOPE_MAX_LEN: usize = 64;

/// A representation of an RFC9237 using the REST-specific model.
///
/// It is arbitrarily limited in length; future versions may give more flexibility, eg. by
/// referring to data in storage.
///
/// This type is constrained to valid CBOR representations of the REST-specific model; it may panic
/// if that constraint is not upheld.
///
/// ## Caveats
///
/// Using this is not very efficient; worst case, it iterates over all options for all AIF entries.
/// This could be mitigated by sorting the records at construction time.
///
/// This completely disregards proper URI splitting; this works for very simple URI references in
/// the AIF. This could be mitigated by switching to a CRI based model.
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
pub struct AifValue([u8; AIF_SCOPE_MAX_LEN]);

impl AifValue {
    pub fn parse(bytes: &[u8]) -> Result<Self, InvalidScope> {
        let mut buffer = [0; AIF_SCOPE_MAX_LEN];

        buffer
            .get_mut(..bytes.len())
            .ok_or(InvalidScope)?
            .copy_from_slice(bytes);

        let mut decoder = minicbor::Decoder::new(bytes);
        for item in decoder
            .array_iter::<(&str, u32)>()
            .map_err(|_| InvalidScope)?
        {
            let (path, _mask) = item.map_err(|_| InvalidScope)?;
            if !path.starts_with("/") {
                return Err(InvalidScope);
            }
        }

        Ok(Self(buffer))
    }
}

impl Scope for AifValue {
    fn request_is_allowed<M: ReadableMessage>(&self, request: &M) -> bool {
        let code: u8 = request.code().into();
        let (codebit, false) = 1u32.overflowing_shl(
            u32::from(code)
                .checked_sub(1)
                .expect("Request codes are != 0"),
        ) else {
            return false;
        };
        let mut decoder = minicbor::Decoder::new(&self.0);
        'outer: for item in decoder.array_iter::<(&str, u32)>().unwrap() {
            let (path, perms) = item.unwrap();
            if perms & codebit == 0 {
                continue;
            }
            // BIG FIXME: We're iterating over options without checking for critical options. If the
            // resource handler router consumes any different set of options, that disagreement might
            // give us a security issue.
            let mut pathopts = request
                .options()
                .filter(|o| o.number() == coap_numbers::option::URI_PATH)
                .peekable();
            if path == "/" && pathopts.peek().is_none() {
                // Special case: For consistency should be a single empty option.
                return true;
            }
            if !path.starts_with("/") {
                panic!("Invalid AIF");
            }
            let mut remainder = &path[1..];
            while !remainder.is_empty() {
                let (next_part, next_remainder) = match remainder.split_once('/') {
                    Some((next_part, next_remainder)) => (next_part, next_remainder),
                    None => (remainder, ""),
                };
                let Some(this_opt) = pathopts.next() else {
                    // Request path is shorter than this AIF record
                    continue 'outer;
                };
                if this_opt.value() != next_part.as_bytes() {
                    // Request path is just different from this AIF record
                    continue 'outer;
                }
                remainder = next_remainder;
            }
            if pathopts.next().is_none() {
                // Request path is longer than this AIF record
                return true;
            }
        }
        // No matches found
        false
    }

    fn is_admin(&self) -> bool {
        self.0[0] >= 0x83
    }
}

/// A scope that can use multiple backends, erasing its type.
///
/// (Think "`dyn Scope`" but without requiring dyn compatibility).
///
/// This is useful when combining multiple authentication methods, eg. allowing ACE tokens (that
/// need an [`AifValue`] to express their arbitrary scopes) as well as a configured admin key (that
/// has "all" permission, which are not expressible in an [`AifValue`].
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Debug, Clone)]
pub enum UnionScope {
    AifValue(AifValue),
    AllowAll,
    DenyAll,
}

impl Scope for UnionScope {
    fn request_is_allowed<M: ReadableMessage>(&self, request: &M) -> bool {
        match self {
            UnionScope::AifValue(v) => v.request_is_allowed(request),
            UnionScope::AllowAll => AllowAll.request_is_allowed(request),
            UnionScope::DenyAll => DenyAll.request_is_allowed(request),
        }
    }

    fn is_admin(&self) -> bool {
        match self {
            UnionScope::AifValue(v) => v.is_admin(),
            UnionScope::AllowAll => AllowAll.is_admin(),
            UnionScope::DenyAll => DenyAll.is_admin(),
        }
    }
}

impl From<AifValue> for UnionScope {
    fn from(value: AifValue) -> Self {
        UnionScope::AifValue(value)
    }
}

impl From<AllowAll> for UnionScope {
    fn from(_value: AllowAll) -> Self {
        UnionScope::AllowAll
    }
}

impl From<DenyAll> for UnionScope {
    fn from(_value: DenyAll) -> Self {
        UnionScope::DenyAll
    }
}

impl From<core::convert::Infallible> for UnionScope {
    fn from(value: core::convert::Infallible) -> Self {
        match value {}
    }
}
