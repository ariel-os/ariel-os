# Structured Board Descriptions

Ariel OS introduces declarative, structured board descriptions.
These are intended to encode the information of what hardware is present on the board, in a way that is machine processable.
In Ariel OS, they are used to generate the `ariel-os-boards` package, that applications can use directly.

Hardware is diverse, and Ariel OS strives not to provide dedicated implementation for specific boards.
Instead, we describe the relevant properties of the board, and provide support for that property (even if, at some point in time, that property only applies to one board).

## SBD files

Structured board description (SBD) files are YAML files that follow the schema defined by the [`sbd-gen-schema` package][sbd-gen-schema-docsrs].
For each *target* of a board, they specify the chip, some OS-specific configuration required for the target, and what hardware is present on the board and is accessible and relevant for the target.
A board/SBD file may comprise multiple targets, e.g., when the board features multiple microcontrollers.
Targets correspond exactly to [Ariel OS's laze builders][laze-builders-book].
Usable chip names currently are Ariel OS's chip laze contexts.

> [!NOTE]
> The goal is that SBD files become completely OS-agnostic, however this is not entirely the case right now.

Existing SBD files are found in the [`boards/` directory][boards-dir-repo-main].
See [the documentation of `sbd_gen_schema`][sbd-gen-schema-docsrs] for the meaning of the various constructs.

See [the Developer Guide][adding-support-board-book] to learn how to add support for a new board in tree.
When adding a new SBD file, or updating an existing one, [`sbd-gen`][sbd-gen-cratesio] is used to re-generate the `ariel-os-boards` package from the SBD files.

## Using the Generated `ariel-os-boards` from Applications

To use the board information provided by `ariel-os-boards`, the package must be added as a dependency of the application, as already done in [the application templates][app-templates-book].

The types exposed by `ariel-os-boards`, generated from the SBD files, are documented in the relevant sections of this user guide.

## Defining Boards Out of Tree

> [!NOTE]
> Ariel OS has experimental support for defining boards out of tree.
> Support is experimental because documentation is lacking and ergonomics could be improved.
> Additionally, out-of-tree boards currently need to be defined along with the [out-of-tree application][out-of-tree-applications-book], and cannot easily be re-used across applications.

### Setting Up an Out-of-Tree Board Package

The following steps allow to set up and generate an out-of-tree board package, similar to `ariel-os-boards`, which can then be used in an out-of-tree application:

1. Starting from an [application template][application-template-book], set up a Cargo workspace for the application: each workspace package must be in a dedicated directory.
   Exclude the `build/` directory from workspace by adding `workspace.exclude = ["build"]` to the workspace manifest.
1. Define the board and generate the out-of-tree board package:
   1. Create a `boards` directory, and copy Ariel OS's `boards/ariel-chips.yaml`, which defines chips supported by Ariel OS, into it.
   1. Create an SBD file for the board, in the `boards` directory: the process is the same as for [in-tree SBD files][adding-support-board-book].
   1. Generate the board package using the following command, providing `<output-directory>` and `<board-package>` as necessary:

      ```sh
      sbd-gen generate-ariel boards -o <output-directory> --package <board-package> --mode create
      ```

      `<output-directory>` is the path of the directory to create for the board package inside the workspace, and `<board-package>` is the name to use for the Cargo package.
       The directory and the package names should typically be identical.
1. The generated board package currently expects some keys to be defined in the workspace manifest:
   - Define the following `workspace.package` keys:
       - `edition`
       - `rust-version`
       - `license`
   - Define the following `workspace.dependencies` as path dependencies, leveraging the laze import:
       - `ariel-os-embassy-common`
       - `ariel-os-hal`
1. The application package currently needs to enable the `no-boards` Cargo feature on the `ariel-os` package, which currently disables support for SBD `quirks`.
1. Add a [`subdirs`][laze-subdirs-docs] entry in the application's `laze-project.yml`, listing the out-of-tree board package directory.

> [!TIP]
> Multiple boards can be defined out-of-tree this way in the same application.

### Using an Out-of-Tree Board Package in an Out-of-Tree Application

Once the board packages have been generated, they need to be added as dependencies to the application, along with or instead of the existing `ariel-os-boards`.
If the application needs portability between `ariel-os-boards` boards and the out-of-tree boards, a simple `cfg_select!` can be used for dispatching in the application, based on the boards' [laze contexts][laze-contexts-book].

### Updating an Out-of-Tree Board Package

To later update the generated board package after editing its SBD file, use `--mode update` instead of `--mode create` in the `sbd-gen` command above.

[boards-dir-repo-main]: https://github.com/ariel-os/ariel-os/tree/main/boards
[sbd-gen-schema-docsrs]: https://docs.rs/sbd-gen-schema/latest/sbd_gen_schema/
[laze-builders-book]: ./build-system.md#laze-builders
[laze-contexts-book]: ./build-system.md#laze-contexts
[adding-support-board-book]: ./adding-board-support.md
[sbd-gen-cratesio]: https://crates.io/crates/sbd-gen
[app-templates-book]: ./getting-started.md#starting-an-application-project-from-a-template-repository
[out-of-tree-applications-book]: ./build-system.md#out-of-tree-applications
[application-template-book]: ./getting-started.md#starting-an-application-project-from-a-template-repository
[laze-subdirs-docs]: https://kaspar030.github.io/laze/dev/reference/subdirs.html
