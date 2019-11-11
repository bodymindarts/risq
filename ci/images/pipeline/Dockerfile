FROM rust

RUN rustup component add clippy

RUN git clone https://github.com/bats-core/bats-core.git \
            && cd bats-core \
            && ./install.sh /usr/local

RUN apt-get update \
            && apt-get install -y \
            jq vim
