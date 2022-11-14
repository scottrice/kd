#!/bin/bash

# Starts up a shell running in the `development` docker container.
# Useful for debugging.

THIS_DIR=$(cd -P "$(dirname "${BASH_SOURCE[0]}")" >/dev/null && pwd)

docker run \
  --name kd-shell \
  --volume ${THIS_DIR}:/code \
  --entrypoint bash \
  --interactive \
  --tty \
  --rm \
  kd/development