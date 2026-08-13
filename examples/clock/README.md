# Clock application

## About

This application provides the best clock experience the hardware and Ariel OS can offer.

Aspirationally, this will take time for wherever is possible (preferably trusted),
and show it using any means customary for a clock
(e.g. on a display, or run bell tower behavior if a speaker is available).

At the time of writing, options are limited:
- The only available source for "human time"
  (i.e., time accurate to within a few seconds, taking time zones into account)
  is BLE Current Time Service.
- The only available output mode is printing to the console.

Worse, neither continuously running RTC backends (not even across reboots, let alone low-power situations)
nor time zones are persisted,
so on every restart, the clock starts up in an indefinite state.
See [our wishlist entry for a time abstraction](https://github.com/ariel-os/ariel-os/discussions/642#discussioncomment-14153832)
and [the one for displays](https://github.com/ariel-os/ariel-os/discussions/642#discussioncomment-11618055)
for more ideas on what this could be supporting.

## Running

In its current state, this needs any board with BLE support;
start it as:

```sh-session
$ laze build -b particle-xenon run
```

Then, connect to the peripheral from any host that runs the Current Time Service
(and preferably the Next DSC Change Service as well).
Options both for Android and for Linux are [listed in najnesnja's blog entry on Current Time Service](https://najnesnaj.github.io/pinetime-zephyr/current-time.html).

## Implementation status

Currently, this example is not performing any abstracting separations.
As Ariel OS grows in provided features
(or possibly as part of making display and clock provided features),
its code size is expected to shrink considerably.
