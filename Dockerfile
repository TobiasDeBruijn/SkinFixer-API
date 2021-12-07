FROM docker-registry.k8s.array21.dev/rust-base:latest as BUILDER

COPY ./src /usr/src/skinfixer_api/src/
COPY ./Cargo.toml ./target* /usr/src/skinfixer_api/
COPY ./migrations /usr/src/skinfixer_api/migrations/

WORKDIR /usr/src/skinfixer_api/

ENV RUSTFLAGS='-C link-arg=-s'
RUN cargo +beta build --release --target x86_64-unknown-linux-musl

FROM alpine:latest
RUN apk add --no-cache ca-certificates

COPY --from=BUILDER /usr/src/skinfixer_api/target/x86_64/unknown-linux-musl/release/skinfixer_api /usr/bin/skinfixer_api

RUN chmod a+rx /usr/local/bin/*
RUN adduser skinfixer_api -s /bin/false -D -H
USER skinfixer_api

EXPOSE 8080
WORKDIR /usr/local/bin
ENTRYPOINT [ "/usr/local/bin/skinfixer_api" ]