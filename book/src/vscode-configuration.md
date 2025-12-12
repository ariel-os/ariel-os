# VSCode configuration

This chapter covers how to setup [Visual Studio Code](https://code.visualstudio.com/) to get features in-editor linting, go to definition, documentation on hover, inlay hints.

## Extensions

Rust language support is provided by `rust-analyzer`, available on the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [Open VSX Registry](https://open-vsx.org/extension/rust-lang/rust-analyzer) for open source forks of VSCode.

It is also recommended to use the the `Even Better TOML` exentsion to have TOML support when editing `Cargo.toml` files: [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml), [Open VSX Registry](https://open-vsx.org/extension/tamasfe/even-better-toml).

`Dependi` can be used to view information about crates and their available versions: [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi), [Open VSX Registry](https://open-vsx.org/extension/fill-labs/dependi).

## Configuration for developing Ariel OS apps

This is meant to be used on projects created using the `cargo-generate` command in the getting-started guide. The configuration works by targeting one [laze builder][laze-builders-book], avoiding the linter to be confused about double declarations.

You will need to have a nightly version of Rust installed, you can install the latest one using:

```sh
rustup toolchain install nightly
```

Then install configure the toolchain by running this at the root of your project:

```sh
laze build -D CARGO_TOOLCHAIN=+nightly install-toolchain
```

To generate/update your vscode configuration in `.vscode/settings.json`, run in the root of your project:

```sh
laze build -b <builder> vscode-config
```

If you get an error about the JSON file being malformated, you may have comments or trailing commas in your configuration, try removing them in `.vscode/settings.json`.

[laze-builders-book]: ./build-system.md#laze-builders
