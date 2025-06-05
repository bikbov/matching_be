ARG BINARY_NAME_DEFAULT=matching_be
ARG MY_GREAT_CONFIG_DEFAULT="someconfig-default-value"

FROM clux/muslrust:stable AS builder

RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser

ARG BINARY_NAME_DEFAULT
ENV BINARY_NAME=$BINARY_NAME_DEFAULT

COPY Cargo.lock Cargo.toml ./

RUN mkdir src && \
    echo "fn main() {}" > src/main.rs && \
    cargo build --target x86_64-unknown-linux-musl --release && \
    rm src/main.rs

COPY src ./src

RUN touch src/main.rs && \
    cargo build --target x86_64-unknown-linux-musl --release

RUN mkdir -p /build-out && \
    cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME /build-out/

FROM scratch

COPY --from=builder /etc/passwd /etc/passwd
COPY --from=builder /etc/group /etc/group

ARG BINARY_NAME_DEFAULT
COPY --from=builder /build-out/$BINARY_NAME_DEFAULT /app

USER dockeruser

ENV BINARY_NAME=$BINARY_NAME_DEFAULT
ARG MY_GREAT_CONFIG_DEFAULT
ENV MY_GREAT_CONFIG=$MY_GREAT_CONFIG_DEFAULT
ENV RUST_LOG="error,$BINARY_NAME_DEFAULT=info"

CMD ["/app"]