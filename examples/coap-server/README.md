# CoAP Server demo

## About

This application starts a minimal CoAP server.

The server offers a single resource, `/hello`, which returns a friendly message,
as well as a message accessible only to authorized users.

## Running

* Run on any board with networking, eg. `laze build -b st-nucleo-wb55 run`.
* [Set up networking](../README.md#networking).
* Run `aiocoap-client`
  to list the resources of the device,
  which are both defined in the [application's `main()` function](src/main.rs).

  ```console
  $ pipx install 'aiocoap[oscore,prettyprint]'
  $ aiocoap-client coap://10.42.0.61/.well-known/core --credentials regular-client.diag
  # application/link-format content was re-formatted
  </hello>,
  </admin>
  ```

  > If you prefer not to install the CoAP client, you can
  > replace any call to `aiocoap-client` with `pipx run --spec 'aiocoap[oscore,prettyprint]' aiocoap-client` instead.

  The output tells you about the `/hello` resource, so read that next:

  ```console
  $ aiocoap-client coap://10.42.0.61/hello --credentials regular-client.diag
  Hello from Ariel OS
  ```

* Explore different authorization levels:

  The argument `--credentials {filename}` and that file's content tell the client to establish a secure connection.
  The resource `/admin` is not accessible to everyone:

  ```console
  $ aiocoap-client coap://10.42.0.61/admin --credentials regular-client.diag
  4.01 Unauthorized
  ```

  but this example provides a configuration file with a key, so that can get in:

  ```console
  $ aiocoap-client coap://10.42.0.61/admin --credentials admin-client.diag
  Congratulations, you are authorized.
  ```

* Making sure you connect to the intended device:

  To make the example usable out of the box, both security configurations are set up to not authenticate the server --
  anyone could impersonate your device.

  In most real applications, you will want to also authenticate the server.
  To do this, watch the debug output of your device for a line like:

  ```
   [INFO ] CoAP server identity: {8:{1:{1:2, 2:h'', -1:1, -2:h'b9943adbc95be73fd6db7b700a2f9b20b311ed0691317cb418da61e14e03db07'}}}
  ```

  Inside `regular-client.diag` you can now replace the `"peer_cred": {"unauthenticated: true}` line
  (which tells your computer to not care whom it is talking to)
  with values copied from the debug output:

  ```
      "peer_cred": {14:{8:{1:{1:2, 2:h'', -1:1, -2:h'b9943adbc95be73fd6db7b700a2f9b20b311ed0691317cb418da61e14e03db07'}}}}
  ```

  Beware that depending on your device's capabilites,
  that key might change over time:
  If the device has support for [persistent storage](https://ariel-os.github.io/ariel-os/dev/docs/book/storage.html),
  it will stay constant across reboots,
  whereas devices without storage fall back to a mode where the device's identity key changes on every startup.

## Further references

There is a [chapter in the book](https://ariel-os.github.io/ariel-os/dev/docs/book/tooling/coap.html)
that describes more concepts and background.
