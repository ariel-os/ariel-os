# coap tests

## About

This application is a work in progress demo of running CoAP with OSCORE/EDHOC security on Ariel OS.

## Running

* Run on any board with networking, eg. `laze build -b particle-xenon run`.
* [Set up networking](../../examples/README.md#networking).
* Run `chmod go-rwX client.cosekey`.
  This file contains the key we authenticate with, and aiocoap gets jumpy around world readable key material.
* Run `pipx run coap-console coap://10.42.0.61 --credentials client.diag`,
  which establishes a secure CoAP connection using EDHOC and OSCORE,
  and shows the log of the device.
* Run `pipx run --spec 'aiocoap[oscore,prettyprint]' aiocoap-client coap://10.42.0.61/.well-known/core --credentials client.diag`
  to show what else the device can do.
  If you kept the log running, you will see that every new command runs through EDHOC once:
  aiocoap does not currently attempt to persist EDHOC derived OSCORE contexts across runs.
* Running multiple concurrent terminal instances is supported,
  up to the maximum number of security contexts that are stored (currently 4).
* There is also `./fauxhoc.py`, which did EDHOC manually before it was integrated in aiocoap.

### Variation

* CoAP in NoSec mode: Building a smaller binary at the cost of confidentiality and integrity protection.
    * Add `-s coap-server-config-unprotected` to the laze invocation; this replaces the demokeys setup.
    * All resources are now only accessible without `--credentials`. (The "fauxhoc" script does not work in that mode).

* CoAP with more than just demo keys:
    * Add `-s coap-server-config-storage` to the laze invocation; this replaces the demokeys setup.
    * Alter the client.diag file have the `peer_cred` reflect the "CoAP server identity" line it procduces at startup
      <!-- FIXME: should be trivial after https://github.com/knurling-rs/defmt/pull/916 -->
      after running the hex values there through https://cbor.me's bytes to diagnostic converter.

    The build system now reads `peers.yml`, which currently encodes similar authorizations for the demo key as the demo setup,
    but in a user configurable way:
    You can add your own private key there, or replace the demo key, and configure resources that should be accessible.

    That file also describes that unauthenticated users may access the `/poem` resource.
    You can access that in an unauthenticated way by running aiocoap without `--credentials` as in NoSec mode.
    You can also access them over an encrypted connection:
    Remove the `own_cred`, `own_cred_style` and `private_key_file` from `client.diag` and replace them with `"own_cred": {"unauthenticated": true}`.
    Now, the request is encrypted, and the client tool verifies the server's identity without identifying itself.

    Instead of using a hard-coded key, the device generates one at first startup,
    and reports the credential that contains its public key in the standard output.

## Roadmap

Eventually, all of this should be covered by 20-line examples.

Until the CoAP roadmap is complete,
this serves as a work bench, test bed, demo zone and playground at the same time.
This application will grow as parts of the roadmap are added,
and shrink as they become default or are wrapped into components of Ariel OS.
