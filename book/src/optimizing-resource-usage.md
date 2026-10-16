# Optimizing Resource Usage

This page contains techniques and hints on optimizing resource usage.

## Flash Usage

### Measuring Flash Usage

TODO:

- `size` laze task, `size -A` (needs to be documented in the Build System page, or to have a more generic laze task)
- `cargo-bloat` (try to have a more generic laze task instead of documenting the existing `bloat` task)

### Reducing Binary Size

TODO:

- `-d panic-printing`
- `-d debug-console`
- Tuning the log level
- Current cost of writing `async` functions
- Selecting only required Cargo features and laze modules (e.g., no `ipv4`?)
- Avoiding duplicate crates (`tree --duplicates`, `cargo-deny`)
- Favoring `defmt` over `log`
- Avoiding hard-coded strings
- Favoring smaller dependencies when possible

## RAM Usage

### Measuring RAM Usage

TODO:

- Using stack painting for measuring
- `size` for `.bss`, `.data`

### Reducing RAM Usage

TODO:

- "Async task `.bss` fragmentation"
    - Consider `-Zprint-type-sizes`, might need full rebuild
- Adjusting `executor_stacksize_required`
- (Adjusting `isr_stacksize_required`)
- Sizing of various buffers (network buffers, UART buffers)
- Timer queue sizing
- Async executor flavor
