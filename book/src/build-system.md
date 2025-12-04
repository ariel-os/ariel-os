# [laze]

Ariel OS makes use of the laze build system to run cargo with the
correct parameters for a specific board and application.

laze provides a `laze build -b <board>` command, which in Ariel OS, internally uses `cargo build`.

laze commands are by default applied to the application(s) within the directory laze is run.
For example, when run in `examples/hello-world`, `laze build -b nrf52840dk`
would build the hello-world example for the `nrf52840dk` board.
Alternatively, the `-C` option can be used to switch to the given directory.

laze allows to override global variables using e.g., `-DFOO=BAR`.

## laze tasks

For tasks like flashing and debugging, Ariel OS uses laze *tasks*.
laze tasks currently have the syntax `laze build -b <board> [other options] <task-name>`.
For example, to run the hello-world example from the `ariel-os` directory, the command would be:

    laze -C examples/hello-world build -b nrf52840dk run

Tasks available in Ariel OS include:

- `run`: Compiles, flashes, and runs an application. The [debug output](./debug-console.md) is printed in the terminal.
- `flash`: Compiles and flashes an application.
- `debug`: Starts a GDB debug session for the selected application.
  The application needs to be flashed using the `flash` task beforehand.
- `flash-erase-all`: Erases the entire flash memory, including user data. Unlocks it if locked.
- `reset`: Reboots the target.
- `tree`: Prints the application's `cargo tree`.
- `vscode-config`: update rust-analyzer configuration for VSCode, see [vscode-configuration](./vscode-configuration.md)
- `fmt`: Formats the project's Rust source code following the [Coding Conventions](./coding-conventions.md) (formats all Rust files except generated ones).

> [!IMPORTANT]
> As some tasks may trigger a rebuild, it is necessary to pass the same settings to related consecutive commands:
`laze build -DFOO=1 flash` followed by `laze build -DFOO=other debug` might not
work as expected, as the second command could be rebuilding the application
before starting the debug session.

## laze modules

laze allows enabling/disabling individual features using [*modules*](#laze-modules), which can be selected
or disabled on the command line using `--select <module>` or `--disable <module>`.
To enable or disable laze modules for an out-of-tree application, see [below](#enabling-laze-modules-for-an-application).

> [!NOTE]
> Modules are documented in their respective pages.

[laze]: https://kaspar030.github.io/laze/dev/

## laze contexts

The laze configuration defines a laze context for each MCU, MCU family, and board.
These can be found in the [support matrix](./hardware-functionality-support.html), where they are called “Ariel OS name”.

Out-of-tree applications can be restricted to specific laze contexts, see [below](#restricting-an-application-to-specific-mcusboards).

In addition, laze passes the names of all contexts related to the selected builder as rustc `--cfg context=$CONTEXT` flags.
This makes it possible to use the `#[cfg]` attribute to introduce feature-gates based on the MCU, MCU family, or board, when required.

## Out-of-tree applications

New application projects should be [started from a template](./getting-started.md#starting-an-application-project-from-a-template-repository).
Out-of-tree applications use the `laze-project.yml` file for configuration through laze.

### Importing Ariel OS

Ariel OS's source and configuration are imported using [laze's `imports`][laze-imports-book] feature.
The [project templates](./getting-started.md#starting-an-application-project-from-a-template-repository) use a [`git` import][laze-git-import-book] to ask laze to clone Ariel OS's repository.
The cloned repository is stored inside `build/imports`.

> [!NOTE]
> It is currently recommended to use Ariel OS's commit ID to track the repository, to avoid surprising changes.
> This commit ID needs to be updated to update the version of Ariel OS used by the application.

It is alternatively possible to clone the repository manually and specify the resulting directory using a [`path` import][laze-path-import-book].
This can be useful when needing to modify Ariel OS itself, when also working on an application.

### Enabling laze modules for an application

Instead of manually specifying [laze modules on the command line](#laze-modules), laze modules required for an application must be specified in the application's laze configuration file, `laze-project.yml`.

The [`selects` array][laze-module-selects-book] allows to specify a list of laze modules that will be enabled for the application, as follows:

```yaml
apps:
  - name: <project-name>
    selects:
      - network
      - random
```

> [!NOTE]
> Note that, while the [CLI option is named `--select`](#laze-modules), the configuration key is `selects`.

The specified modules will be enabled for the application, some of which may enable associated Cargo features (as individually documented for each laze module).
If a module is not available on a target—e.g., because networking is not available on the target, or not yet supported by Ariel OS—laze will prevent the application to be compiled for that target.

### Disabling laze modules for an application

Conversely, to disable laze modules through the configuration file, the [`conflicts` array][laze-module-conflicts-book] is used:

```yaml
apps:
  - name: <project-name>
    selects:
      - sw/threading
    conflicts:
      - multi-core
```

This enables [support for multithreading, but disables multicore usage](./multithreading.md#multicore-support).

### Restricting an application to specific MCUs/boards

Finally, an application may be restricted to specific MCUs, MCU families, or boards by explicitly specifying [laze contexts](#laze-contexts) the application is allowed to be compiled for:

```yaml
apps:
  - name: <project-name>
    context:
      - bbc-microbit-v2
      - nrf52
      - nrf5340
      - rpi-pico-w
```

[laze-imports-book]: https://kaspar030.github.io/laze/dev/reference/imports.html
[laze-git-import-book]: https://kaspar030.github.io/laze/dev/reference/import/git.html
[laze-path-import-book]: https://kaspar030.github.io/laze/dev/reference/import/path.html
[laze-module-selects-book]: https://kaspar030.github.io/laze/dev/reference/module/selects.html
[laze-module-conflicts-book]: https://kaspar030.github.io/laze/dev/reference/module/conflicts.html
