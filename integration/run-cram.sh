#!/bin/bash

# Run the integration tests

# TODO - Get these running as part of `cargo test`

set -e

CRAM="$(which cram)"

# TODO - It would be nice to provide Cram in the repo so we don't have to rely
# on it being installed on the PATH
if [ -z "$CRAM" ]; then
  echo "Missing Cram binary. Please install Cram and put it on your PATH"
  exit 1
fi

THIS_DIR=$(cd -P "$(dirname "${BASH_SOURCE[0]}")" >/dev/null && pwd)
REPO_ROOT=$(cd "$THIS_DIR/.." && pwd)

# Add the location of the built binary as an environment variable
# so tests can use it
KD_PATH="$REPO_ROOT/target/debug/kd"

KD_PATH="$KD_PATH" \
  exec "$CRAM" --shell /bin/bash "$@"