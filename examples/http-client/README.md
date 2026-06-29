# http-client

## About

This application is demonstrating making basic HTTP GET requests.

## How to run

In this directory, run

    laze build -b nrf52840dk run

This example does a GET request to <https://crab.ariel-os.org> or an URL configured
through the `ENDPOINT_URL` environment variable, the response body will be printed
as a string, along with some relevant HTTP response headers before the program exits.
TLS 1.3 and mDNS are supported; however the server is not authenticated.
Websites using TLS and a RSA signature won't work out of the box, you will need to
select the `alloc` laze module and enable the `rsa` feature on `reqwless`.

The [networking chapter] of the book contains information on how to set up networking.

[networking chapter]: https://ariel-os.github.io/ariel-os/dev/docs/book/networking.html
