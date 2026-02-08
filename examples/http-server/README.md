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

## Using USB Ethernet

You can run this example with USB Ethernet by running

    laze build -b esp32-s3-devkitc-1 -s use-usb-ethernet run

This will configure the example to set up a DHCP server for the connected PC. The
device will have a static IP address, and will serve the HTML site at <http://10.42.0.61/>.

Look [here](../README.md#networking) for more information about network configuration.
