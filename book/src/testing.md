# Testing

Ariel OS supports testing using the [`embedded-test`][embedded-test-docs] crate.

`embedded-test` (and `probe-rs`) serve as replacement for the regular `cargo
test` / libtest based testing harness, as the latter cannot be used on `no_std`
(embedded) devices.

Please refer to the [`embedded-test` documentation][embedded-test-docs] for
more info.

The build system of Ariel OS integrates the `embedded-test`-based testing to that
once set up, tests can be run by issuing `laze build -b <board> test`.

Both async and non-async code can be tested.

Note: Currently, Ariel OS requires a fork of embedded-test. When using Ariel's
build system, this will be used automatically.

`embedded-tests` can be used for any target that has `probe-rs` support (which currently means all targets).

## Differences to vanilla `embedded-tests`

In Ariel OS, the OS itself will start and initialize components *before* the
tests are run. Logging, networking, ... will be available as for regular
Ariel OS applications.

As a consequence, no `embedded-test` features other than `ariel-os` should be enabled.
In order to not require `default-features = false`, the (default)
`panic-handler` feature is ignored when the `ariel-os` feature is enabled)

## Setting up `embedded-test` for Ariel OS applications or libraries

Steps for enabling tests:

1. Add `embedded-test` to `dev-dependencies`, enabling the `ariel-os`:**

Add the following to your Cargo.toml:

```yaml
[dev-dependencies]
embedded-test = { version = "0.5.0", features = ["ariel-os"] }
```

2. Disable the default `libtest` based test harness:

This depends on whether a lib, a bin or a seperate test should be tested.

Add the following to your Cargo.toml:

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
# for a seperate test in `test.rs`
[[test]]
name = "test"
harness = false
```

3. Enable the `embedded-test` or `embedded-test-only` laze feature:

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

Note: Even a library crate needs an entry in laze's `apps` in order to make the
      `test` task available.
      Selecting `embedded-test-only` will make sure that `laze run` is disabled.

4. Add some boilerplate to `lib.rs`, `main.rs` or `test.rs`:

```Rust
# This goes to the top of the file
#![no_main]
#![no_std]
#![feature(used_with_arg)]
#![feature(impl_trait_in_assoc_type)]
```

5. Write the tests:

```Rust
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

To run a test, from whithin the crate's folder run:

```shell
laze build -b <board> test
```

[embedded-test-docs]: https://docs.rs/embedded-test/latest/embedded_test/
