# Getting started with Ariel OS

## Install prerequisites

1. install needed system dependencies. On Ubuntu, the following is sufficient:

        apt install build-essential curl git python3 pkg-config \
                   libssl-dev llvm-dev cmake libclang-dev gcc-arm-none-eabi \
                   clang libnewlib-nano-arm-none-eabi unzip lld ninja-build

1. install [rustup](https://rustup.rs/)

1. install [laze](https://github.com/kaspar030/laze): `cargo install laze`

1. install [probe-rs](https://github.com/probe-rs/probe-rs): `cargo install probe-rs-tools --locked`

1. clone this repository and cd into it

1. install rust targets: `laze build install-toolchain`

## Clone template repository

    git clone https://github.com/ariel-os/ariel-os-hello

or

    cargo generate ariel-os/ariel-os-template --name <some-name>

or

    cargo generate ariel-os/ariel-os-template --name <some-name> --lib
