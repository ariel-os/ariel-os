# Changelog of coapcore

## Unreleased

### Fixed

* `alloc` is linked, so the crate builds when minicbor's `alloc` or `std` feature is enabled elsewhere in the build.

## 0.1.1

### Added

* Fields of ACE and error details are made public to facilitate implementation of own policies.
* Credentials are now sent by value if requested using the `EAD_CRED_BY_VALUE` option of the [EDHOC OSCORE profile of ACE](https://www.ietf.org/archive/id/draft-ietf-ace-edhoc-oscore-profile-09.html#name-requesting-authentication-c).

### Changed

* Updated internal dependencies.
* Increased strictness on linting; refactored where indicated.
* Edition changed to 2024.
