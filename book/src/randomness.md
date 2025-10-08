# Randomness and Entropy Sources

Ariel OS provides RNGs which fulfill needs for both fast and cryptographically secure sources of randomness.

## Provided RNGs

The `random` [laze module][laze-modules-book] needs to be enabled to be able to obtain the provided RNGs.
Two different RNG interfaces are provided, which both implement `rand_core` traits:

- A fast RNG interface, not suitable for cryptography use, which can be obtained with [`random::fast_rng()`][fast-rng-fn-rustdoc].
- A cryptographically secure pseudo-RNG (CSPRNG) interface, which can be obtained with [`random::crypto_rng()`][crypto-rng-fn-rustdoc] when the `csprng` Cargo feature is enabled.
  In addition, the `csprng` Cargo feature also enables support for the [`getrandom` crate][getrandom-cratesio], even when it is present as a transitive dependency only.


> To ensure fast operation of the fast RNG, the obtained RNG must be reused between invocations, instead of obtaining new ones through [`random::fast_rng()`][fast-rng-fn-rustdoc].

## RNG Seeding

When the `random` module is selected, the `hwrng` [laze module][laze-modules-book] is automatically enabled as well, so that the RNGs get automatically seeded from the hardware RNG (i.e., the TRNG) at startup.

> In the future, Ariel OS may also support leveraging persistent storage in combination with a pre-provisioned seed to enable to use the CSPRNG on MCUs which do not provide a hardware RNG.

[fast-rng-fn-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/random/fn.fast_rng.html
[crypto-rng-fn-rustdoc]: https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/random/fn.crypto_rng.html
[laze-modules-book]: ./build-system.md#laze-modules
[getrandom-cratesio]: https://crates.io/crates/getrandom
