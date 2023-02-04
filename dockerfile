FROM clux/muslrust:stable AS builder

WORKDIR /usr/src/project

# RUN rustup target add x86_64-unknown-linux-musl
# RUN apt update && apt install musl-dev musl-tools openssl -y

RUN cargo init .

COPY --chown=rust:rust Cargo.toml Cargo.toml
COPY --chown=rust:rust src src
# COPY --chown=rust:rust Cargo.lock Cargo.lock

# RUN rm src/*.rs

RUN cargo build --release 
# --target x86_64-unknown-linux-musl

# FROM gcr.io/distroless/static:nonroot
FROM scratch

COPY --from=builder /usr/src/project/target/x86_64-unknown-linux-musl/release/server-oauth-rs .

EXPOSE 3000

CMD ["/server-oauth-rs"]