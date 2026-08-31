
ARG RUST_VERSION=1.89
ARG APP_NAME=migrator_service
ARG CARGO_FEATURES=""


FROM rust:${RUST_VERSION}-alpine AS build
ARG APP_NAME
ARG CARGO_FEATURES
WORKDIR /app

RUN apk add --no-cache clang lld musl-dev git

RUN --mount=type=bind,source=app,target=app \
    --mount=type=bind,source=Cargo.toml,target=Cargo.toml \
    --mount=type=bind,source=Cargo.lock,target=Cargo.lock \
    # --mount=type=bind,source=lib.rs,target=lib.rs \
    --mount=type=cache,target=/app/target/ \
    --mount=type=cache,target=/usr/local/cargo/git/db \
    --mount=type=cache,target=/usr/local/cargo/registry/ \
    cargo build --release --locked $CARGO_FEATURES && \
    cp ./target/release/$APP_NAME /bin/server

FROM alpine:3.18 AS final

ARG UID=10001
RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    appuser
USER appuser

COPY --from=build /bin/server /bin/

CMD ["/bin/server"]
