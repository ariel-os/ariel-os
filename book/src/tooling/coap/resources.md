# CoAP resources provided by ArielOS

For CoAP servers, Ariel OS can provide the following resources:

* `/.well-known/core`: Provided for all servers;
  lists other resources in a format described by [RFC 6690](https://www.rfc-editor.org/info/rfc6690/)
* `/.well-known/edhoc`: Entry point for EDHOC.
  Provided if EDHOC is enabled.
* `/authz-info`: Entry point for the ACE OSCORE profile.
  Provided if token parsing is enabled.
* `/id`: Build and device information.
  This is provided when the [laze module][laze-modules-book] `coap-server-resource-identity` is enabled.
  Its representation is a CBOR map with tag 3050242967, indexed with integers that provide the following items as they are currently available:
  - 1: Operating system name
  - 6: Board name
  - 8: Device identity

  The full format is specified by this [CDDL](https://www.rfc-editor.org/info/rfc8610/):

  ```
  description = #6.3050242967({
    ? os-name => tstr,
    ? os-version => version,
    ? app-name => tstr,
    ? app-version => version,
    ? source-hash => hash,
    ? board-name => tstr,
    ? build-hash => hash,
    ? device-id => device-id-value,

    * extension => any,
  })

  ; Versions can be structured into sortable parts, but are
  ; generally free-form depending on the OS or version.
  version = [+ (tstr / hash / int)] / any

  ; Precise hashes (semantics and length) depend on the underlying
  ; build or versioning system, but are usually shown in hex.
  hash = bstr

  ; Structure depends on the OS; Ariel OS uses byte strings.
  device-id-value = any

  ; Consumers should ignore any key/value pairs that are only
  ; processable through the "extended" feature (eg. added keys or
  ; unknown value types).
  extension = any .feature "extended"

  os-name = 1
  os-version = 2
  app-name = 3
  app-version = 4
  source-hash = 5
  board-name = 6
  build-hash = 7
  device-id = 8
  ```

  *Privacy considerations:* The device identity is a stable identifier, and as such may allow tracking a user across networks.
  Like all other resources that are not part of establishing secure connections, it is only available to authorized users, unless the "unprotected" mode is selected.

[laze-modules-book]: ../build-system.md#laze-modules
