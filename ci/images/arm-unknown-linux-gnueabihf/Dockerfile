FROM rustembedded/cross:arm-unknown-linux-gnueabihf-0.1.16

RUN apt-get update && \
      apt-get install -y \
      curl

RUN curl https://sh.rustup.rs -sSf | \
      sh -s -- --default-toolchain stable-arm-unknown-linux-gnueabihf -y

ENV PATH ${PATH}:/root/.cargo/bin
