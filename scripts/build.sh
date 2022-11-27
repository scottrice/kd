#!/bin/bash

set -euxo pipefail

docker build \
  --target development \
  --tag kd/development \
  .