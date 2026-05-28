# CoAP tests

## About

In this application,
the use of CoAP with EDHOC security is tested and explored
beyond what is in the [CoAP server example](../../examples/coap-server).

## Running

* Run on any board with networking, eg. `laze build -b st-nucleo-wb55 run`.
* [Set up networking](../../examples/README.md#networking).
* Run `pipx run coap-console coap://10.42.0.61 --credentials admin-client.diag`,
  which establishes a secure CoAP connection using EDHOC and OSCORE,
  and shows the log of the device.
* Run `pipx run --spec 'aiocoap[oscore,prettyprint]' aiocoap-client coap://10.42.0.61/.well-known/core --credentials admin-client.diag`
  to show what else the device can do.
  If you look at the logs (by passing `-D LOG=debug` to laze or `-vv` to aiocoap-client) or monitor the network traffic with Wireshark, you will see that every new command runs through EDHOC once:
  aiocoap does not currently attempt to persist EDHOC derived OSCORE contexts across runs.
* Running multiple concurrent terminal instances is supported,
  up to the maximum number of security contexts that are stored (currently 4).

### Variation

* CoAP in NoSec mode: Building a smaller binary at the cost of confidentiality and integrity protection.
    * Add `-s coap-server-config-unprotected` to the laze invocation; this replaces the demokeys setup.
    * All resources are now only accessible without `--credentials`.
    * Note that even in the default configuration,
      the `/poem` resource is accessible this way,
      because the security policy in `peers.yml` says so.

* Using your own key:

  This test's setup ships a hard-coded key.
  That is dangerous: Everyone who finds the top secret Ariel OS repository!!!

  Better generate your own key:

  ```console
  pipx run --spec 'aiocoap[oscore,prettyprint]' aiocoap-keygen -k 0123 mysecret.cosekey
  ```

  Update your `admin_client.diag` file:
  * Put the output of that command in the `"own_cred"` key.
  * Replace `"private_key": ...,` with `"private_key_file": "mysecret.cosekey",`.

    (This better lives in a separate file rather than inside the configuration,
    as the key file has stricter access permissions set up).
  
  … and update your `peers.yml` file:

  * Put **only what is inside of** the `{14:` / `}` of the command's output
    into the `kccs` of key of `peers.yml`.

  * Flash your device again:
    Now it is only you who can use its protected resources.

* Authenticating the server:
    * Watch for a line like this in the server's output:

      ```
      [INFO ] CoAP server identity: {8:{1:{1:2, 2:h'', -1:1, -2:h'6fe816e6686167dbc745b6f14cb6adb9e69e19eb4558ad7ef4c27a2ae564edc5'}}}
      ```

      Note that this key may or may not persist when you change the firmware.
      On devices without Ariel "storage" feature, it even changes on every reboot.
      You can find out what is built by running `laze build -b THE_BOARD_YOU_USE info-modules`:
      If that contains `coap-server-config-storage`, the key will persist across reboots.

    * Alter `admin-client.diag`: replace the `{"unauthenticated": true}` values with `{14: ...}` (replacing `...` with the `{8:...}` text from your output).

      In subsequent CoAP exchanges,
      the client can verify that it is talking to the right server.
      You can simulate failure by changing details of the credential (e.g. replacing `h''` with `h'1234'`),
      or by trying to use it with a different device.

## Roadmap

Eventually, all of this should be covered by 20-line examples.

Until the CoAP roadmap is complete,
this serves as a work bench, test bed, demo zone and playground at the same time.
This application will grow as parts of the roadmap are added,
and shrink as they become default or are wrapped into components of Ariel OS.
