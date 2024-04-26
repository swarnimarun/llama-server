# use server-cuda, server-intel and server-rocm as base images for all image flavours
ARG BASE_IMAGE="ghcr.io/ggerganov/llama.cpp:server"

FROM rust:latest as builder
ADD . /code
WORKDIR /code
RUN cargo build --release

FROM ${BASE_IMAGE}
COPY --from=builder /code/target/release/llama-server /llama-server 
ENTRYPOINT [ "/llama-server" ]
