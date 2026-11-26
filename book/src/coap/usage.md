# Using CoAP

## Server side

An example of a CoAP server is [provided as `examples/coap-server`], see [its `coap_run()` task] for the practical steps.
This requires selecting the `coap-server` [laze module][laze-modules-book], and on top of the user's handlers, it runs any operating-system-provided CoAP handlers
(which would be started in an independent task if `coap-server` was not selected).

A CoAP server is created by assembling several **resource handlers on dedicated paths**:
There might be a path `/s/0` representing a particular sensor,
and a path `/fwup` for interacting with firmware updates.
The CoAP implementation can put additional resources at well-known locations,
eg. `/.well-known/core` for discovery or `/.well-known/edhoc` for establishing secure connections.

The handler needs to concern itself with security aspects of the request content
(eg. file format parsers should treat incoming data as possibly malformed),
but the decision whether or not a request is allowed is delegated to an [access policy](#server-access-policy).

[provided as `examples/coap-server`]: https://github.com/ariel-os/ariel-os/tree/main/examples/coap-server
[its `coap_run()` task]: https://github.com/ariel-os/ariel-os/blob/a5483e1cef1bba9b345719ed7e785d7013b8cf73/examples/coap-server/src/main.rs#L20


## Client side

The example [provided as `examples/coap-client`], which sends a single POST request.
It requires selecting the `coap-client` [laze module][laze-modules-book].

A program that triggers a CoAP request provides[^whatsinarequest] some components to the CoAP stack before phrasing the actual request:

* A **URL describing the resource**, eg. `coap://coap.summit.riot-os.org/agenda` or `coap+tcp://[2001:db8::1]/.well-known/core`.

  Note that while the address is printed here as a text URL
  (and may even be entered as such in code),
  its memory and transmitted representations are binary components.

* Optionally, directions regarding how to reach or find the host.

  This is not needed when there are IP literals involved, or DNS is available to resolve names.
  Directions may be available if the application discovered a usable proxy,
  or when it is desired to use opt-in discovery mechanisms such as multicast.

* A policy reference on how to authenticate the server, and which identity to present.
  This is optional if there is a global policy,
  or if there is an implied security mechanism for the URL.


[^whatsinarequest]: The components required for a request are not documented as such in the CoAP RFCs,
    but it is the author's opinion that they are a factual requirement:
    Implementations may implicitly make decisions on those,
    but the decisions are still made.
    At the time of writing, there [is an open issue](https://github.com/core-wg/corrclar/issues/41) to clarify this in the specifications.

[provided as `examples/coap-client`]: https://github.com/ariel-os/ariel-os/tree/main/examples/coap-client
[laze-modules-book]: ../build-system.md#laze-modules
