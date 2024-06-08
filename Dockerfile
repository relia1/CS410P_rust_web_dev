ARG RUST_VERSION=1.77.2
ARG APP_NAME=rust-web
FROM rust:${RUST_VERSION} AS build
ARG APP_NAME
WORKDIR /app

# Install host build dependencies.
# RUN apk add --no-cache clang lld musl-dev git

# Cache downloaded+built dependencies
COPY *.toml /app/
RUN \
    mkdir /app/src && \
    echo 'fn main() {}' > /app/src/main.rs && \
    cargo build --release && \
    rm -Rvf /repo/src

# Build our actual code
COPY src /app/src
COPY migrations /app/migrations
RUN \
    touch src/main.rs && \
    cargo build --release

# FROM alpine:3.18 AS final

FROM rust:${RUST_VERSION} AS final
# Create a non-privileged user that the app will run under.
# See https://docs.docker.com/go/dockerfile-user-best-practices/
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

# Copy the executable from the "build" stage.
COPY --from=build /app/target/release/rust-web /bin/
RUN ls -l 
# COPY --chown=appuser:appuser ./assets ./assets
#COPY --chown=appuser:appuser migrations/ /migrations/
# Expose the port that the application listens on.
EXPOSE 3000

# What the container should run when it is started.
CMD ["/bin/rust-web"]
