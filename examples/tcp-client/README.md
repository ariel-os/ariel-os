# tcp-client

## About

This application is testing basic
[Embassy](https://github.com/embassy-rs/embassy) _networking_ usage with Ariel OS.

## How to run

In this directory, run

    laze build -b rpi-pico-w run

The application will try to connect to [tcpbin.com](https://tcpbin.com/), a simple echo server using TCP.

The [networking chapter] of the book contains information on how to set up networking.

If everything goes well, you should see the server's response:

    [INFO ] txd: Hello world!

[networking chapter]: https://ariel-os.github.io/ariel-os/dev/docs/book/networking.html
