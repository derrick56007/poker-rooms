FROM ubuntu:latest

ENV PATH "$PATH:/home/developer/.cargo/bin"

RUN apt update \
    && DEBIAN_FRONTEND=noninteractive apt install -y \
    curl git build-essential pkg-config libssl-dev jq

RUN curl -fsSL https://get.docker.com -o get-docker.sh
RUN sh get-docker.sh
RUN rm get-docker.sh

RUN useradd -ms /bin/bash developer
USER developer
WORKDIR /home/developer

RUN curl https://sh.rustup.rs -sSf | sh -s -- --default-toolchain=1.63.0 -y
RUN rustup component add rust-analysis