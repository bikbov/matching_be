ARG BINARY_NAME_DEFAULT=matching_be

FROM clux/muslrust:stable AS builder
ARG BINARY_NAME_DEFAULT
RUN groupadd -g 10001 -r dockergrp && useradd -r -g dockergrp -u 10001 dockeruser
COPY Cargo.lock Cargo.toml ./
COPY src ./src
RUN cargo build --target x86_64-unknown-linux-musl --release &&  \
mkdir -p /build_out && \
cp target/x86_64-unknown-linux-musl/release/$BINARY_NAME_DEFAULT /build_out/

FROM scratch
ARG BINARY_NAME_DEFAULT
COPY --from=0 /etc/passwd /etc/passwd
COPY --from=builder /build_out/$BINARY_NAME_DEFAULT /
USER dockeruser
CMD ["/matching_be"]
