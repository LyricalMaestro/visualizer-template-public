FROM fedora:41

SHELL ["/bin/bash", "-c"]

WORKDIR /root
RUN dnf install -y git unzip g++ cmake

# Install Rust
RUN curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh -s -- -y

# Install Volta (Node package manager)
RUN curl https://get.volta.sh | bash

ENV VOLTA_HOME="/root/.volta"
ENV CARGO_HOME="/root/.cargo"

ENV PATH="$VOLTA_HOME/bin:$CARGO_HOME/bin:$PATH"

# Install node and yarn
RUN volta install node@22
RUN volta install yarn

# Install cargo dependency
RUN cargo install wasm-pack

CMD [ "bash" ]
