FROM debian:11

# curl is needed by Rust update tool
RUN apt-get update \
    && apt-get install -y curl build-essential libgtk-4-dev \
    && apt-get clean ; rm -rf /var/lib/apt/lists/* /tmp/* /var/tmp/* /usr/share/doc/*

RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y # Download the latest stable Rust

ENV PATH="/root/.cargo/bin:${PATH}"

RUN cargo --version
