# Examples

## Overview

This directory contains example applications that showcase how to use Ariel OS.

- [alloc/](./alloc): Demonstrates how to use an allocator
- [benchmark/](./benchmark): How to use `benchmark()`
- [blinky/](./blinky): Demonstrates basic GPIO output usage
- [coap-server](./coap-server) and [coap-client](./coap-client): Application level networking examples
- [device-metadata/](./device-metadata): Retrieve metadata about the running device
- [gpio/](./gpio): GPIO pin control example.
- [hello-world/](./hello-world): A classic, async version
- [hello-world-threading/](./hello-world-threading): A classic, using a thread
- [http-client/](./http-client): HTTP client example
- [http-server/](./http-server): HTTP server example
- [log](./log): Example demonstrating different log levels for printing feedback messages.
- [minimal/](./minimal): Minimized to the max Ariel OS config
- [power/](./power): Demonstrates power management functionality
- [random/](./random): Demonstrates obtaining random values
- [storage/](./storage): Demonstrates persistent storage interaction
- [tcp-echo/](./tcp-echo): TCP echo example
- [testing/](./testing): Demonstrates `embedded-test` integration
- [thread-async-interop/](./thread-async-interop): How to make async tasks and preemptively scheduled threads interoperate
- [threading/](./threading): How to start and use preemptively scheduled threads
- [threading-channel/](./threading-channel): How to use `ariel_os::thread::sync::Channel` for passing messages between threads
- [threading-event/](./threading-event): How to use `ariel_os::thread::sync::Event`
- [threading-multicore/](./threading-multicore): Demonstrates basic threading on multicore
- [udp-echo/](./udp-echo): UDP echo example
- [usb-keyboard/](./usb-keyboard): USB HID example
- [usb-serial/](./usb-serial): USB serial example

## Networking

Some examples demonstrate networking capabilities. By default, they will try to
get an IP address via DHCPv4.

See the [networking documentation][book-networking] to learn how to set up networking.

[book-networking]: https://ariel-os.github.io/ariel-os/dev/docs/book/networking.html
