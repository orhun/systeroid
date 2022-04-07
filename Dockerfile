FROM ekidd/rust-musl-builder:latest as builder
WORKDIR /home/rust/src
COPY . .
RUN cargo build --locked --release -p systeroid
RUN cargo build --locked --release --no-default-features -p systeroid-tui
RUN mkdir -p build-out/
RUN ["/bin/bash", "-c", "cp target/x86_64-unknown-linux-musl/release/systeroid{,-tui} build-out/"]
RUN ["/bin/bash", "-c", "strip build-out/systeroid{,-tui}"]
RUN ["/bin/bash", "-c", "scripts/clone-linux-docs.sh"]

FROM scratch
WORKDIR /app
COPY --from=builder \
    /home/rust/src/build-out/systeroid \
    /home/rust/src/build-out/systeroid-tui \
    /usr/local/bin/
COPY --from=builder \
    /home/rust/src/linux-docs \
    /usr/share/doc/linux-docs
USER 1000:1000
ENV NO_CACHE=1
ENTRYPOINT ["systeroid"]
