# syntax=docker/dockerfile:1
# Zaroz builds with this file because it exists (a Dockerfile always wins over auto-detection).

FROM rust:1.97-slim-trixie AS build
WORKDIR /src
COPY Cargo.toml Cargo.lock ./
COPY src ./src
# The cache mounts keep downloaded crates and compiled dependencies between builds. They are not
# part of the image, so the final binary is copied out of target/ in the same step.
RUN --mount=type=cache,target=/usr/local/cargo/registry \
    --mount=type=cache,target=/src/target \
    cargo build --release --locked \
 && cp target/release/zaroz-example-dockerfile-rust /app

FROM debian:trixie-slim
RUN useradd --uid 10001 --no-create-home --shell /usr/sbin/nologin app
COPY --from=build /app /usr/local/bin/app
USER app
ENV PORT=8080
EXPOSE 8080
# The binary is PID 1 and handles SIGTERM itself: a stop drains instead of being killed.
CMD ["/usr/local/bin/app"]
