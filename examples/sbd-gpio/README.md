# sbd-gpio

## About

This application demonstrates GPIO through the higher level drivers for Buttons and Switches that exist in Ariel OS.

Three GPIO are used and need to present in the SBD file of the target board.

One GPIO is configured as output and used to drive an LED on the board.
The second GPIO is wrapped in a Button0 struct and speeds up or slows down toggling
of the LED when pressed.
When selecting the `with-switches` laze module,a third GPIO is wrapped in a Switch0 struct
and used to initiate the toggling and to stop the program. This laze module requires the target
boards to declare switches. `nrf9160dk-nrf9160` is currently the only one.

## How to run

In this directory, run

    laze build -b nrf52840dk run
