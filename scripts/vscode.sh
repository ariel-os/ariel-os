#!/bin/sh

mkdir -p .vscode

extraArgs=$(echo '"'$CARGO_ARGS $FEATURES'"' | jq 'split(" ")' )
toolchain=""
if [ -n "$CARGO_TOOLCHAIN" ]; then
  toolchain=$(echo '"'RUSTUP_TOOLCHAIN'"' : '"'${CARGO_TOOLCHAIN#+}'"',) # remove the + to get the toolchain name
fi

rustflags=$(jq -n --arg toescape "${RUSTFLAGS}" '$toescape')

cat <<EOF > .vscode/settings.json
{
  "rust-analyzer.check.extraArgs": $extraArgs,
  "rust-analyzer.check.command": "clippy",
  "rust-analyzer.server.extraEnv": {
    $toolchain
    "CONFIG_BOARD": "$CONFIG_BOARD",
    "CARGO_BUILD_TARGET": "$CARGO_BUILD_TARGET",
    "${CARGO_TARGET_PREFIX}_RUSTFLAGS": $rustflags,
  },
  "rust-analyzer.check.allTargets": false
}
EOF