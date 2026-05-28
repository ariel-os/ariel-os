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

## Exploration beyond the basics

There are several ways in which you can go beyond the simple demo of the example.
All those can be run independently or combined.

### Add own resources

More paths can be added to the server by extending the list in `main.rs`.

Whenever something is added there,
you also need to decide whether it should be accessible to unauthenticated users,
and which methods they may invoke on it.
(Multiple methods can be given in a list, e.g. `/my-resource: [GET, PUT, POST]`).

Copying the handlers in the example quickly gets boring, as they only serve static strings.
While you can register any implementation of [`coap_hander::Handler`](https://docs.rs/coap-handler/latest/coap_handler/trait.Handler.html)` + `[`coap_handler::Reporting`](https://docs.rs/coap-handler/latest/coap_handler/trait.Reporting.html) with the [`.at()`](https://docs.rs/coap-handler-implementations/latest/coap_handler_implementations/trait.HandlerBuilder.html#method.at) builder
(or just of `Handler` when using [`.at_with_attributes()`](https://docs.rs/coap-handler-implementations/latest/coap_handler_implementations/trait.HandlerBuilder.html#method.at_with_attributes)),
it is often convenient not to implement those traits yourself,
but rather use building blocks:

* The [`SimpleRenderable`](https://docs.rs/coap-handler-implementations/latest/coap_handler_implementations/trait.SimpleRenderable.html) trait is practical if you want to simply write data into a response, in paritcular text.
  Any implementation of it can be packed into the [`SimpleRendered()`](https://docs.rs/coap-handler-implementations/latest/coap_handler_implementations/struct.SimpleRendered.html) wrapper,
  just like the strings in the example.
* The [`GetRenderable`](https://docs.rs/coap-handler-implementations/latest/coap_handler_implementations/trait.GetRenderable.html) family of traits, wrapped in a [`TypeHandler`](https://docs.rs/coap-handler-implementations/latest/coap_handler_implementations/struct.TypeHandler.html),
  are convenient for building resources that can be PUT or POSTed to,
  deserialize data from thsoe requests, and serialize representations into responses.

The [source of the `coap-message-demos` crate](https://codeberg.org/chrysn/coap-tools/src/branch/main/coap-message-demos/src)
has concrete examples for all of them.

### Explore different authorization levels

The argument `--credentials {filename}` and that file's content tell the client to establish a secure connection;
the `regular-client.diag` file says to not present any particular identity.
But the resource `/admin` is not accessible to everyone (as defined in the policy at `peers.yml`):

```console
$ aiocoap-client coap://10.42.0.61/admin --credentials regular-client.diag
4.01 Unauthorized
```

This example provides a configuration file with a key, so that can get in:

```console
$ aiocoap-client coap://10.42.0.61/admin --credentials admin-client.diag
Congratulations, you are authorized.
```

### Making sure you connect to the intended device

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
You can find out whether storage is supported by running `laze build -b THE_BOARD_YOU_USE info-modules` --
if that contains `coap-server-config-storage`, the key will persist across reboots.

In subsequent CoAP exchanges,
the client can verify that it is talking to the right server.
You can simulate failure by changing details of the credential (e.g. replacing `h''` with `h'1234'`),
or by trying to use it with a different device.

### Using your own key

This test's setup ships a hard-coded key.
That is dangerous: Everyone who finds the top secret Ariel OS repository can use your device!!!

Better generate your own key:

```console
$ aiocoap-keygen -k 0123 mysecret.cosekey
```

Update your `admin-client.diag` file:
* Put the output of that command in the `"own_cred"` key.
* Replace `"private_key": ...,` with `"private_key_file": "mysecret.cosekey",`.

(This better lives in a separate file rather than inside the configuration,
as the key file has stricter access permissions set up).

… and update your `peers.yml` file:

* Put **only what is inside of** the `{14:` / `}` of the command's output
  into the `kccs` of key of `peers.yml`.

* Flash your device again:

  Now it is only you who can use its protected resources.

## Security expectations

Ariel OS has not received external security review,
and neither have several cryptographic components that it uses:
The 'isprovided "as is"' disclaimers of the licenses apply.

That being said,
after following the ['Using your own key'](#using-your-own-key) and ['Making sure you connect to the intended device'](#making-sure-you-connect-to-the-intended-device) sections,
this example protects your application-level communication with all that Ariel OS can offer on your platform.

## Further references

There is a [chapter in the book](https://ariel-os.github.io/ariel-os/dev/docs/book/tooling/coap.html)
that describes more concepts and background.
