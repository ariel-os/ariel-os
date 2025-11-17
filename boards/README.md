# About

This directory contains structured board description (sbd) files that Ariel OS is
using to generate the board support from.

## Generating board support

1. Make sure `sbd-gen` is installed:

    cargo install sbd-gen

2. Use the [sbd-gen][sbd-gen] utility to generate/update the `ariel-os-boards` crate from the
sbd files:

    sbd-gen generate-ariel --mode update boards -o src/ariel-os-boards

See [sbd-gen][sbd-gen] for more information.

[sbd-gen]: https://github.com/ariel-os/sbd
