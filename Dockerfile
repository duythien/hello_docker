# This is the build stage for run test
FROM --platform=$BUILDPLATFORM rust:1.71.1 as test
ARG BUILDPLATFORM
WORKDIR /substrate

RUN apt-get update -y && apt-get install libwayland-dev libprotobuf-dev  clang libclang-dev protobuf-compiler -y

COPY . /substrate
RUN cargo build --features "fast-runtime node-test"


# This is the build stage mainnet
FROM --platform=$BUILDPLATFORM rust:1.71.1 as mainnet
ARG BUILDPLATFORM
WORKDIR /substrate

RUN apt-get update -y && apt-get install libwayland-dev libprotobuf-dev  clang libclang-dev protobuf-compiler -y

COPY . /substrate
RUN cargo build --release

# This is the final stage: a very small image where we copy the Vban binary."
FROM ubuntu:22.04 as final

COPY --from=mainnet /substrate/target/release/vban-node /usr/local/bin

RUN mkdir -p /root/.local/share && \
    mkdir /data && \
    ln -s /data /root/.local/share/vban-node

VOLUME ["/data"]
EXPOSE 30333 9933 9944
ENTRYPOINT ["/usr/local/bin/vban-node"]
