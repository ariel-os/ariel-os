# Coding Conventions

## Rust

### Item Order in Rust Modules

Items SHOULD appear in Rust modules in the following order, based on [the one used by rust-analyzer](https://rust-analyzer.github.io/manual.html#auto-import):

1. Inner doc comment
1. [Inner attributes](https://doc.rust-lang.org/reference/attributes.html)
1. Unconditional—i.e., not feature-gated—bodyless [modules](https://doc.rust-lang.org/reference/items/modules.html)
1. Conditional—i.e., feature-gated—bodyless modules
1. Unconditional imports from the following:
    1. `std`/`alloc`/`core`
    1. External crates (including crates from the same workspace)
    1. Current crate, paths prefixed by `crate`
    1. Current module, paths prefixed by `self`
    1. Super module, paths prefixed by `super`
    1. Re-exports—i.e., `pub` imports not used in the module
1. Conditional imports from the same
1. [Const items](https://doc.rust-lang.org/reference/items/constant-items.html)
1. [Static items](https://doc.rust-lang.org/reference/items/static-items.html)
1. Other items

TODO: type aliases before other items?

TODO: how to organize type definitions w.r.t. related logic?

### Imports

Imports from the same crate with the same visibility MUST be [merged into a single use statement](https://rust-analyzer.github.io/manual.html#auto-import).

#### Imports from Re-exports

When using whole-crate re-exports from [`ariel_os::reexports`](https://ariel-os.github.io/ariel-os/dev/docs/api/ariel_os/reexports/index.html), two imports SHOULD be used: one to bring the re-exported crate into the scope, and the other one to import the required items from that crate, as it it were a direct dependency of the crate, as follows:

```rust
use ariel_os::reexports::embassy_usb;
use embassy_usb::class::hid::HidReaderWriter;
```

### Comments

#### Doc Comments

All public items listed in the documentation—i.e., not marked with [`#[doc(hidden)]`](https://doc.rust-lang.org/rustdoc/write-documentation/the-doc-attribute.html#hidden)—SHOULD be documented.

Doc comments MUST use the [line comment style](https://doc.rust-lang.org/reference/comments.html#doc-comments), not the block style.

Doc comments MUST be written in third person present, not in the imperative mood, as recommended by [RFC 1574](https://github.com/rust-lang/rfcs/blob/master/text/1574-more-api-documentation-conventions.md#summary-sentence).
Each sentence in doc comments—including the first one, before an empty line—SHOULD end with a period.
For instance, instead of:

```rust
/// Get the underlying value
```

write:

```rust
/// Returns the underlying value.
```

More generally, use the [`std` docs](https://doc.rust-lang.org/stable/std/) as inspiration.

When possible—i.e., when items are in scope—items mentioned in the documentation MUST be linked to (see [C-LINK](https://rust-lang.github.io/api-guidelines/documentation.html#prose-contains-hyperlinks-to-relevant-things-c-link)).
This is useful for readers, to quickly access the mentioned item, but it also helps prevent the docs from lagging behind, as broken links are tested for in CI, making it easy to spot renamed or removed items.

### `unsafe` Code

When `unsafe` is used, a `SAFETY` comment MUST be added, in the style [supported by Clippy](https://rust-lang.github.io/rust-clippy/master/index.html#/undocumented_unsafe_blocks).

TODO: enforce it in CI

### Naming Conventions

Names SHOULD adhere to the [official API guidelines](https://rust-lang.github.io/api-guidelines/naming.html).

TODO: how to name error types/error enum variants (`CannotDoSth` vs `DoingSth`)?

## Dependencies

If the same dependency is used in multiples crates within the workspace, that dependency SHOULD be specified in the *workspace*'s `Cargo.toml` file and workspace crates should import them from there.

## Adding a new workspace crate, exposed by `ariel-os`

To add a new workspace crate re-exported by `ariel-os`, follow these steps:

1. Create the new crate's directory in `src/`.
1. Run `cargo init --lib` in that directory.
1. Add `#![deny(missing_docs)]` to the crate; some lints are already inherited from the workspace and do not need to be added to the new crate.
1. In the workspace's `Cargo.toml` `workspace.members` array, ensure the new entry preserves the lexicographic order of that array.
1. In the workspace's `Cargo.toml` `dependencies` array, add a (properly sorted) entry.
1. Re-export the crate from the `ariel-os` crate, inline it in the docs as done for the other crates, and feature-gate it if necessary.
1. Add the crate to the list of crates checked by Clippy in `.github/workflows/main.yml`, preserving lexicographic order.
1. If the crate is expected to have tests that can be run with `cargo test`:
    1. Add a feature named `_test` that enables all features that can be tested.
    1. Add a `laze.yml` with an application for the crate named `crates/your-crate-name` that selects the `host-test-only` module (see e.g., `src/ariel-os/laze.yml`)
    1. Add the crate's directory to its parent's `laze.yml` `subdirs`.
1. If the new crate is feature-gated and if this is possible, add the feature that enables it to the ones used by cargo doc in `.github/workflows/main.yml` and in `.github/workflows/build-deploy-docs.yml`, preserving lexicographic order.

## laze

### Modules

laze modules in examples and tests MUST use [`selects`](https://kaspar030.github.io/laze/dev/reference/module/selects.html) instead of [`depends`](https://kaspar030.github.io/laze/dev/reference/module/depends.html).
Even though their behaviors are different in the general case, they are equivalent in our case.
