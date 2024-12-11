FROM rust:1.83.0-alpine3.21 as builder
WORKDIR /app
RUN apk update
RUN apk add --no-cache musl-dev bash git
COPY . .
RUN cargo build --locked --release -p systeroid
RUN cargo build --locked --release --no-default-features -p systeroid-tui
RUN mkdir -p build-out/
RUN ["/bin/bash", "-c", "cp target/release/systeroid{,-tui} build-out/"]
RUN ["/bin/bash", "-c", "strip build-out/systeroid{,-tui}"]
RUN ["/bin/bash", "-c", "scripts/clone-linux-docs.sh"]

FROM scratch
WORKDIR /app
COPY --from=builder \
    /app/build-out/systeroid \
    /app/build-out/systeroid-tui \
    /usr/local/bin/
COPY --from=builder \
    /app/linux-docs \
    /usr/share/doc/linux-docs
USER 1000:1000
ENV NO_CACHE=1
ENTRYPOINT ["systeroid"]
