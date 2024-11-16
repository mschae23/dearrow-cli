FROM rust:alpine as builder
ENV OPENSSL_STATIC=true
ENV CARGO_REGISTRIES_MSCHAE23_INDEX=sparse+https://mschae23.de/git/api/packages/mschae23/cargo/
WORKDIR /dearrow-cli
RUN apk add --no-cache \
    openssl-dev \
    openssl-libs-static \
    musl-dev 
RUN rustup target add x86_64-unknown-linux-musl
COPY . .
RUN cargo build --release --target=x86_64-unknown-linux-musl
RUN chmod +x /dearrow-cli/target/x86_64-unknown-linux-musl/release/dearrow-cli

FROM scratch
COPY --from=builder /dearrow-cli/target/x86_64-unknown-linux-musl/release/dearrow-cli /dearrow-cli
ENTRYPOINT ["/dearrow-cli"]
