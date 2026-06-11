# http-client

## About

This application is demonstrating making basic HTTP GET requests.

## How to run

In this directory, run

    laze build -b nrf52840dk run

This example needs to be provided with an endpoint URL to send the HTTP GET
request to, through the `ENDPOINT_URL` environment variable.
TLS 1.3 and mDNS are supported; however the server is not authenticated.
A GET request will be made every 3 seconds, even in case of failure and the
response body will be printed as a string, along with some relevant HTTP
response headers.

The [networking chapter] of the book contains information on how to set up networking.

[networking chapter]: https://ariel-os.github.io/ariel-os/dev/docs/book/networking.html
