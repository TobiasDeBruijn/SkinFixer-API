FROM rust:alpine as BUILDER
RUN apk add --no-cache musl-dev openssl openssl-dev

COPY ./src /usr/local/src/skinfixer_api/src/
COPY ./Cargo.toml /usr/local/src/skinfixer_api/


WORKDIR /usr/local/src/skinfixer_api

RUN cargo build --release


FROM alpine:latest
ENV GLIBC_REPO=https://github.com/sgerrand/alpine-pkg-glibc
ENV GLIBC_VERSION=2.30-r0

RUN set -ex && \
    apk --update add libstdc++ curl ca-certificates && \
    for pkg in glibc-${GLIBC_VERSION} glibc-bin-${GLIBC_VERSION}; \
        do curl -sSL ${GLIBC_REPO}/releases/download/${GLIBC_VERSION}/${pkg}.apk -o /tmp/${pkg}.apk; done && \
    apk add --allow-untrusted /tmp/*.apk && \
    rm -v /tmp/*.apk && \
    /usr/glibc-compat/sbin/ldconfig /lib /usr/glibc-compat/lib

RUN apk add --no-cache openssl ca-certificates

COPY --from=BUILDER /usr/local/src/skinfixer_api/target/release/skinfixer_api /usr/bin/skinfixer_api
EXPOSE 8080
CMD ["sh", "-c", "/usr/bin/skinfixer_api"]