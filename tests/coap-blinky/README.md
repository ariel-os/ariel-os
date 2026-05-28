# coap blinky

## About

This application makes the GPIO pin from the blinky example accessible over the network.

## Running

* Run on any board with networking, eg. `laze build -b particle-xenon run`.
* [Set up networking](../../examples/README.md#networking).
* Run `pipx run --spec 'aiocoap[prettyprint,oscore]' aiocoap-client coap://10.42.0.61/led -m PUT --content-format application/cbor --payload true --credentials client.diag`
  or `false`.

## Roadmap

Right now, this demonstrates how easily code written for RIOT OS can be shared with Ariel OS.

In the long run, any configured LED should be exposed,
its connection direction (active high vs. active low) should be pulled from the board descriptions,
and maybe exposed GPIO pins could even become usable and run-time configurable between input and output mode.
