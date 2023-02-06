FROM rust:1.66.1 AS builder

WORKDIR /usr/src/project

# RUN rustup target add x86_64-unknown-linux-musl
# RUN apt update && apt install musl-dev musl-tools openssl -y

RUN cargo init .

COPY --chown=rust:rust Cargo.toml Cargo.toml
COPY --chown=rust:rust src src
# COPY --chown=rust:rust Cargo.lock Cargo.lock

# RUN rm src/*.rs

RUN cargo build --release 
RUN strip -s ./target/release/server-oauth-rs
# --target x86_64-unknown-linux-musl

# FROM gcr.io/distroless/static:nonroot
FROM gcr.io/distroless/cc

COPY --from=builder /usr/src/project/target/release/server-oauth-rs .

EXPOSE 3000

CMD ["/server-oauth-rs"]