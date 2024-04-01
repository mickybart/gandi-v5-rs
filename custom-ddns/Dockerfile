# syntax=docker/dockerfile:1.4
FROM rust:1.76 as builder

WORKDIR /usr/src/

ENV RUSTFLAGS="-C target-feature=+crt-static"

COPY . .

RUN --mount=type=cache,id=rustcache,target=/usr/local/cargo/registry --mount=type=cache,id=rustcache,target=./target <<EOF
# unwind not required
sed -i 's/^# panic/panic/' Cargo.toml

cargo build --release --target x86_64-unknown-linux-gnu -p custom-ddns
EOF

RUN --mount=type=cache,id=rustcache,target=./target <<EOF
mv /usr/src/target/*/release/custom-ddns ./microservice
EOF

# Production image
FROM gcr.io/distroless/static-debian11

COPY --from=builder /usr/src/microservice /

CMD ["/microservice"]