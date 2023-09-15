# This is the build stage for Substrate. Here we create the binary.
FROM --platform=$BUILDPLATFORM rust:1.71.1 as builder
ARG BUILDPLATFORM
WORKDIR /substrate

RUN apt-get update -y && apt-get install libwayland-dev libprotobuf-dev  clang libclang-dev protobuf-compiler -y


COPY . /substrate
RUN cargo build --release -vvv

# This is the 2nd stage: a very small image where we copy the Substrate binary."
FROM --platform=$BUILDPLATFORM ubuntu:22.04

COPY --from=builder /substrate/target/release/hello_docker /usr/local/bin

RUN mkdir -p /root/.local/share && \
    mkdir /data && \
    ln -s /data /root/.local/share/hello_docker

VOLUME ["/data"]
EXPOSE 30333 9933 9944
ENTRYPOINT ["/usr/local/bin/hello_docker"]
