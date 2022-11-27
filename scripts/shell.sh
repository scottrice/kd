#!/bin/bash

# Starts up a shell running in the `development` docker container.
# Useful for debugging.

CURRENT_DIR=$(pwd)

docker run \
  --name kd-shell \
  --volume ${CURRENT_DIR}:/code \
  --entrypoint bash \
  --interactive \
  --tty \
  --rm \
  kd/development