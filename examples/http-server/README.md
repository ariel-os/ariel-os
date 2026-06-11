# http-server

## About

This application demonstrates running an HTTP server with Ariel OS.

## How to run

In this directory, run

    laze build -b nrf52840dk run

Ariel OS will serve an example HTML homepage at <http://10.42.0.61/> and will
expose a JSON endpoint at <http://10.42.0.61/button> reporting on the state of
a connected push button if present, otherwise the endpoint will not be exposed
at all.

The [networking chapter] of the book contains information on how to set up networking.

[networking chapter]: https://ariel-os.github.io/ariel-os/dev/docs/book/networking.html
