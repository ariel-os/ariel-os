# Testing

Ariel OS supports in-hardware testing using the [`embedded-test`][embedded-test-docs] crate.
`embedded-test`, used in conjunction with `probe-rs`, serves as a replacement for the regular `cargo
test` based test harness, as the latter cannot be used on `no_std`
(embedded) devices.
Please refer to the [`embedded-test` documentation][embedded-test-docs] for
more info.

The build system of Ariel OS integrates the `embedded-test`-based testing so that
once set up, tests can be run by issuing `laze build -b <board> test`.
`embedded-tests` can be used for any target that has `probe-rs` support (which currently means all targets).
Both async and non-async code can be tested.

> Currently, Ariel OS requires a fork of `embedded-test`. When using Ariel's
build system, this will be used automatically.

## Differences from vanilla `embedded-test`

In Ariel OS, the OS itself will start and initialize components *before* the
tests are run. Logging, networking, ... will be available as for regular
Ariel OS applications.

As a consequence, no Cargo features other than `ariel-os` should be enabled on the `embedded-test` dependency.
In order to not require `default-features = false`, the (default)
`panic-handler` feature is ignored when the `ariel-os` feature is enabled.

## Setting up `embedded-test` for Ariel OS applications or libraries

Steps for enabling tests:

1. Add `embedded-test` as a dev-dependency of your crate, and enable its `ariel-os` Cargo feature, as follows:

```yaml
[dev-dependencies]
embedded-test = { version = "0.6.0", features = ["ariel-os"] }
```

2. Disable the default test harness:

This depends on whether a lib, a bin or a separate test should be tested.

Add the following to your `Cargo.toml`:

```yaml
# for a library crate
[lib]
harness = false
```

or

```yaml
# for the default `bin`, "name" needs to match the package name
[[bin]]
name = "ariel-os-hello"
harness = false
```

or

```yaml
# for a separate test in `test.rs`
[[test]]
name = "test"
harness = false
```

3. Enable the `embedded-test` or `embedded-test-only` [laze module](./build-system.md#laze-modules):

```yaml
apps:
# for an application:
  - name: your-application
    selects:
      - embedded-test

# for a library:
  - name: crate/your-library
    selects:
      - embedded-test-only
```

> Even a library crate needs an entry in laze's `apps` in order to make the `test` task available.
> Selecting `embedded-test-only` will make sure that `laze run` is disabled.

4. Add the following boilerplate to `lib.rs`, `main.rs` or `test.rs`:

```rust
# This goes to the top of the file
#![no_main]
#![no_std]
```

5. Write the tests:

```rust
#[cfg(test)]
#[embedded_test::tests]
mod tests {
    // Optional: An init function which is called before every test
    #[init]
    fn init() -> u32 {
        return 42;
    }

    // A test which takes the state returned by the init function (optional)
    // This is an async function, it will be executed on the system executor.
    #[test]
    async fn trivial_async(n: u32) {
        assert!(n == 42)
    }
}
```

Again, please refer to the [`embedded-test` documentation][embedded-test-docs] for
more information.

## Running the tests

To run a test, execute from within the crate's directory:

```shell
laze build -b <board> test
```

[embedded-test-docs]: https://docs.rs/embedded-test/latest/embedded_test/
