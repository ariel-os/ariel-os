# Changelog of coapcore

## Unreleased

### Added

* Observe notifications can be protected with `OscoreEdhocHandler::take_correlation` and `OscoreEdhocHandler::protect_notification`, using the new `Correlation` and `NotificationError` types.
* Security contexts can be pinned with `OscoreEdhocHandler::set_pinned`; pinned contexts are never evicted, and a handshake that finds no unpinned slot is answered with 5.03. `OrderedPool::insert_evicting` evicts selectively.

## 0.1.1

### Added

* Fields of ACE and error details are made public to facilitate implementation of own policies.
* Credentials are now sent by value if requested using the `EAD_CRED_BY_VALUE` option of the [EDHOC OSCORE profile of ACE](https://www.ietf.org/archive/id/draft-ietf-ace-edhoc-oscore-profile-09.html#name-requesting-authentication-c).

### Changed

* Updated internal dependencies.
* Increased strictness on linting; refactored where indicated.
* Edition changed to 2024.
