#!/bin/sh

# script is used by github workflows

grep 'CARGO_TOOLCHAIN: .*nightly' laze-project.yml | grep -o 'nightly-[0-9-]\+'
