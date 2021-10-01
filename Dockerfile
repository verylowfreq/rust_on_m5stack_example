FROM espressif/idf-rust-examples

RUN rustup install stable
RUN rustup component add rustfmt --toolchain stable
