# http-ota-update

## About

This application demonstrates over the air software updates using SUIT and HTTP.

## How to run

In this directory, run

    laze build -b rpi-pico2-w run

This will create an HTTP server with a single gettable resource '/version' that returns a number in JSON and can be updated using SUIT.

The [networking chapter] of the book contains information on how to set up networking.

[networking chapter]: https://ariel-os.github.io/ariel-os/dev/docs/book/networking.html

## Updating the version

To update the version you need to:
- write the new version number in `new-version.txt`
- update the sequence number in `suit/manifest.json` to a number that is strictly above the last accepted sequence number (starts at 0)
- update the URL and port that the http client can use to fetch `new-version.txt`
- Set up an http server that can provide the file using `python3 -m http.server <port>` for example.
- generate and sign the manifest using `suit-tool`
```sh
suit-tool create -i suit/manifest.json -o suit/manifest.suit

suit-tool sign -m suit/manifest.suit -k suit/demo-private-key.pem -o suit/manifest.signed.suit
```
- send the manifest to the server
```sh
curl -X PUT http://<server-address>/suit -T suit/manifest.signed.suit
```
