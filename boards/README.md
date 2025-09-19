# About

This directory contains structured board description (sbd) files that Ariel OS is
using to generate the board support from.

## Generating board support

1. Make sure `sbd` is installed:

    cargo install --git https://github.com/ariel-os/sbd

2. Use the [sbd][sbd] utility to generate/update the `ariel-os-boards` crate from the
sbd files:

    sbd boards -o src/ariel-os-boards

See [sbd][bsd] for more information.

[sbd]: https://github.com/ariel-os/sbd
