#!/bin/bash

set -euxo pipefail

# TODO: Set up the current folder as a volume in the container, so that changes
# to the container get reflected in the repo
# --volume  .:/code
docker run \
  --name kd-develop \
  --rm \
  kd/development