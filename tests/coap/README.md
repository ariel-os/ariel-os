# CoAP tests

## About

In this application,
the use of CoAP with EDHOC security is tested and explored
beyond what is in the [CoAP server example](../../examples/coap-server).

## Running

* Run on any board with networking, eg. `laze build -b st-nucleo-wb55 run`.
* [Set up networking](../../examples/README.md#networking).
* Run `pipx run coap-console coap://10.42.0.61 --credentials admin-client.diag`,
  which establishes a secure CoAP connection using EDHOC and OSCORE,
  and shows the log of the device.
* Run `pipx run --spec 'aiocoap[oscore,prettyprint]' aiocoap-client coap://10.42.0.61/.well-known/core --credentials admin-client.diag`
  to show what else the device can do.
  If you look at the logs (by passing `-D LOG=debug` to laze or `-vv` to aiocoap-client) or monitor the network traffic with Wireshark, you will see that every new command runs through EDHOC once:
  aiocoap does not currently attempt to persist EDHOC derived OSCORE contexts across runs.
* Running multiple concurrent terminal instances is supported,
  up to the maximum number of security contexts that are stored (currently 4).

### Variation

* CoAP in NoSec mode: Building a smaller binary at the cost of confidentiality and integrity protection.
    * Add `-s coap-server-config-unprotected` to the laze invocation; this replaces the demokeys setup.
    * All resources are now only accessible without `--credentials`.
    * Note that even in the default configuration,
      the `/poem` resource is accessible this way,
      because the security policy in `peers.yml` says so.

See also the [main CoAP server example's "beyond the basics" section](../../examples/coap/README.md#exploration-beyond-the-basics):
most of the content there applies here just as well.

## Roadmap

Eventually, all of this should be covered by 20-line examples.

Until the CoAP roadmap is complete,
this serves as a work bench, test bed, demo zone and playground at the same time.
This application will grow as parts of the roadmap are added,
and shrink as they become default or are wrapped into components of Ariel OS.
