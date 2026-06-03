# sntp-router

## About

This example synchronizes `ariel-os-sntp` with an NTP server on the local
router (e.g.: `10.42.0.1:123` in many NetworkManager setups) and updates the global SNTP clock.
It starts a background task for the sync loop and a small monitor task that
prints the current time.

## How to run

In this directory, run:

    laze build -b nrf52840dk run

Look [here](../README.md#networking) for more information about network configuration.

