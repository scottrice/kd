# `builder` is an image that contains all the dependencies necessary to run
# a full build.
FROM rust:1.65-buster AS builder

# Install dev dependencies
RUN apt update && \
      apt install -y python3-pip && \
      pip3 install cram

WORKDIR /code

COPY . .

# `development` is a builder image that contains all the dependencies necessary
# to develop `kd`, including for testing and such.
FROM builder as development

RUN cargo build --locked

# Is this really the best way to create a development environment with Docker? Super fucking weird.
# CMD ["sleep", "infinity"]
CMD ["target/debug/kd"]

# `release` is a builder image that contains only the tooling necessary to build
# a release version of the app
FROM builder as release

RUN cargo build --release --locked

FROM alpine:latest AS production

COPY --from=release target/release/kd /usr/bin/kd

CMD ["kd"]