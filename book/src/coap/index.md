# CoAP

[CoAP] is an **application layer protocol similar to HTTP** in its use
(eg. you could POST data onto a resource on a server that is identified by a URL)
but geared towards IoT devices in its format and its security mechanisms.

CoAP provides a versatile set of transports
(with IETF Proposed Standards for running [over UDP] including multicast, [over TCP and WebSockets],
other standards for running [over SMS and NB-IoT], and more in development).
It relies on proxies to span across transports and to accommodate the characteristics of particular networks,
and offers features exceeding the classical REST set such as [observation].

**Ariel OS supports** the use of CoAP
for implementing clients, servers or both in a single device.
As part of our mission for strong security,
we use encrypted CoAP traffic by default as explained below.
*Currently*, Ariel OS supports CoAP on its original UDP transport.
Its CoAP server implementation supports several security mechanisms,
whereas client support is not mature yet, and lacks security support.

[CoAP]: https://coap.space/
[over UDP]: https://datatracker.ietf.org/doc/html/rfc7252
[over TCP and WebSockets]: https://datatracker.ietf.org/doc/html/rfc8323
[over SMS and NB-IoT]: https://www.omaspecworks.org/wp-content/uploads/2018/10/Whitepaper-11.1.18.pdf
[observation]: https://datatracker.ietf.org/doc/html/rfc7641

The following chapters describe
* [how to use CoAP in Ariel OS](./usage.md),
* [how it is secured by default, and which options and policies exist](./security.md), and
* [which resources are provided by modules of Ariel OS](./resources.md).
