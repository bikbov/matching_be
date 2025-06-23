ARG BINARY_NAME_DEFAULT=matching_be

FROM clux/muslrust:stable AS builder
ARG BINARY_NAME_DEFAULT
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN groupadd -g 10001 -r dockergrp && \
    useradd -r -g dockergrp -u 10001 dockeruser && \
    cargo build --target x86_64-unknown-linux-musl --release && \
    mkdir -p /build_out && \
    cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME_DEFAULT /build_out/

FROM scratch
ARG BINARY_NAME_DEFAULT
COPY --from=builder /etc/passwd /etc/group  /etc/
COPY --from=builder /build_out/$BINARY_NAME_DEFAULT /
USER dockeruser
CMD ["/matching_be"]
