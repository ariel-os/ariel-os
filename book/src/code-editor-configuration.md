# Code editor configuration

This chapter covers how to setup supported code editors to get features in-editor linting, go to definition, documentation on hover, inlay hints.

> [!NOTE]
> The `editor-config` laze task generates a configuration according to the [laze builder used][laze-builders-book] and the [modules selected][laze-modules-book] (in the cli and in `laze-project.yml`). 
> If you add/remove a module or want to target another builder, you will need to re-generate the configuration.

## VSCode

[Visual Studio Code](https://code.visualstudio.com/) is a popular code editor that can easily be configured to work with Ariel OS.

### Extensions

Rust language support is provided by `rust-analyzer`, available on the [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=rust-lang.rust-analyzer) and [Open VSX Registry](https://open-vsx.org/extension/rust-lang/rust-analyzer) for open source forks of VSCode.

It is also recommended to use the the `Even Better TOML` extension to have TOML support when editing `Cargo.toml` files: [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=tamasfe.even-better-toml), [Open VSX Registry](https://open-vsx.org/extension/tamasfe/even-better-toml).

`Dependi` can be used to view information about crates and their available versions: [Visual Studio Marketplace](https://marketplace.visualstudio.com/items?itemName=fill-labs.dependi), [Open VSX Registry](https://open-vsx.org/extension/fill-labs/dependi).

### Configuration for developing Ariel OS apps (vscode)

This is meant to be used on projects created using the `cargo-generate` command in the getting-started guide. The configuration works by targeting one [laze builder][laze-builders-book], avoiding complaints from the linter about double declarations.

To generate/update your vscode configuration in `.vscode/settings.json`, run in the root of your project:

```sh
laze build -b <builder> editor-config vscode
```

If you get an error about the JSON file being malformated, you may have comments or trailing commas in your configuration, try removing them in `.vscode/settings.json`.

## Helix

[Helix](https://helix-editor.com/) is a terminal file editor built in Rust with good Rust support and vi-like navigation and shortcuts.

### Dependencies

Helix uses the rust-analyzer binary from your system, you can install it using:

```sh
rustup component add rust-analyzer
```

### Configuration for developing Ariel OS apps (helix)

This is meant to be used on projects created using the `cargo-generate` command in the getting-started guide. The configuration works by targeting one [laze builder][laze-builders-book], avoiding complaints from the linter about double declarations.

To generate/update your helix configuration in `.helix/languages.json`, run in the root of your project:

```sh
laze build -b <builder> editor-config helix
```

## Zed

[Zed](https://zed.dev/) is a code editor built in Rust with a fast and lightweight user interface. The Rust extension and language server is automatically installed when opening a `.rs` file.

### Configuration for developing Ariel OS apps (Zed)

This is meant to be used on projects created using the `cargo-generate` command in the getting-started guide. The configuration works by targeting one [laze builder][laze-builders-book], avoiding complaints from the linter about double declarations.

To generate/update your Zed configuration in `.zed/settings.json`, run in the root of your project:

```sh
laze build -b <builder> editor-config zed
```

## Gram

[Gram](https://gram.liten.app/) is a hard fork of Zed disabling AI features and focusing on stability.

### Enabling Rust support on Gram

User action is needed to authorize Gram to download rust-analyzer. Open a Rust source file, then open the language server configuration by clicking the fifth icon on the bottom left of the editor (should have a red notification dot) or typing `lsp::OpenLanguageServerConfig` in the command palette and finally enable "Allow download" for `rust-analyzer`.

### Configuration for developing Ariel OS apps (Gram)

This is meant to be used on projects created using the `cargo-generate` command in the getting-started guide. The configuration works by targeting one [laze builder][laze-builders-book], avoiding complaints from the linter about double declarations.

To generate/update your Gram configuration in `.gram/settings.jsonc`, run in the root of your project:

```sh
laze build -b <builder> editor-config gram
```

## Generic rust-analyzer

rust-analyzer has a work in progress universal configuration file format supposed to work with any editor that uses rust-analyzer through the Language Server Protocol, the progress is tracked in [issue #13529 of rust-analyzer](https://github.com/rust-lang/rust-analyzer/issues/13529).

A configuration file for an Ariel OS app can be generated using this command:

```sh
laze build -b <builder> editor-config rust-analyzer
```

At the time of writing (2026-03-30) some features don't work compared to using the Helix or VSCode configuration.

[laze-builders-book]: ./build-system.md#laze-builders
[laze-modules-book]: ./build-system.md#laze-modules
