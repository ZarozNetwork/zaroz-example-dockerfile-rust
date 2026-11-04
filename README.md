# zaroz-example-dockerfile-rust

A Zaroz Cloud **git-build example** for the **Dockerfile** path: a small Rust HTTP service
([axum](https://github.com/tokio-rs/axum) on tokio) with its own multi-stage `Dockerfile`. When a
repository has a `Dockerfile`, Zaroz builds that file instead of detecting the language.

- `src/main.rs`: `GET /` and `GET /healthz`, listens on `$PORT` (8080 when unset), drains on
  SIGTERM. The tests call the router directly, with no network.
- `Dockerfile`: builds a release binary with cached crates and `target/`, then copies only the
  binary into `debian:trixie-slim` and runs it as a non-root user.

## Run it locally

```bash
cargo test
cargo run                      # http://localhost:8080
docker build -t example . && docker run --rm -p 8080:8080 example
```

## Deploy it on Zaroz

> **Status:** deploying straight from a git repository is in development on Zaroz Cloud. This
> repository is one of the sample apps it is built and tested against, so the steps below describe
> the planned flow. A `zaroz.toml` example will be added once that file's format is published; this
> app needs none, because the defaults are enough.

1. Fork this repository.
2. In the Zaroz console, create a **Workload** from a git repository, connect GitHub (GitLab,
   Forgejo or any git URL also work) and pick your fork.
3. Zaroz builds every push to the default branch with the `Dockerfile` at the repository root.
   The `RUN --mount=type=cache` lines keep crates and compiled dependencies between builds.
4. The service listens on the `PORT` environment variable, which Zaroz sets.

Use the same pattern for any language: a repository with a `Dockerfile` is built from it.

## CI

`.github/workflows/ci.yml` runs `cargo fmt --check`, clippy and `cargo test`, then builds the
image, runs it, checks `/healthz` and checks that `docker stop` exits 0.

MIT licensed.
