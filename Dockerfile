ARG RUST_VERSION=1.84.0
ARG NODE_VERSION=23.11.0
ARG APP_NAME=sync-play

FROM node:${NODE_VERSION} AS frontend
WORKDIR /app

COPY frontend /app/frontend
COPY svelte-path-finder /app/svelte-path-finder

RUN --mount=type=cache,target=/app/frontend/node_modules \
    cd frontend && \
    npm install && \
    npm run dependencies && \
    npm run build

FROM rust:${RUST_VERSION} AS build
ARG APP_NAME
WORKDIR /app

COPY --from=frontend /app/frontend/build /app/frontend/build

RUN apt update && apt install -y musl-tools pkg-config libssl-dev

RUN --mount=type=bind,source=src,target=src \
    --mount=type=bind,source=svelte-path-finder,target=svelte-path-finder \
    --mount=type=bind,source=frontend/src,target=frontend/src \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=build.rs,target=build.rs \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    dpkgArch="$(dpkg --print-architecture)"; \
    case "${dpkgArch##*-}" in \
        i386) target="i686-unknown-linux-gnu";; \
        amd64) target="x86_64-unknown-linux-gnu";; \
        armhf) target="armv7-unknown-linux-gnueabihf";; \
        arm64) target="aarch64-unknown-linux-gnu";; \
        *) echo >&2 "unsupported architecture: ${dpkgArch}"; exit 1 ;; \
    esac; \
    rustup target add $target; \
    cargo build --release --target $target && \
    cp "./target/$target/release/$APP_NAME" /bin/$APP_NAME

FROM debian:12 AS final
ARG APP_NAME
WORKDIR /config

RUN apt update && apt install -y openssl

COPY --from=build /bin/$APP_NAME /bin/$APP_NAME

ENTRYPOINT ["sync-play"]
