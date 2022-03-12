# FE
FROM node:16.14.0-slim as FE
WORKDIR /usr/src/app
COPY house-fe/package*.json ./
RUN npm ci
COPY house-fe/. .
RUN npm run build

# BE
FROM lukemathwalker/cargo-chef:latest-rust-1.59.0 AS chef
WORKDIR /app
RUN rustup target add x86_64-unknown-linux-musl
RUN apt update && apt install -y musl-tools musl-dev pkg-config libssl-dev
RUN update-ca-certificates

FROM chef AS planner
COPY preference-be/ .
RUN cargo chef prepare --recipe-path recipe.json

FROM chef AS BE 
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --target x86_64-unknown-linux-musl --recipe-path recipe.json
COPY preference-be/ .
RUN cargo build --release --target x86_64-unknown-linux-musl --bin preference-be

FROM alpine:latest as FINAL
# Create appuser
ENV USER=baracca
ENV UID=10001

RUN adduser \
    --disabled-password \
    --gecos "" \
    --home "/nonexistent" \
    --shell "/sbin/nologin" \
    --no-create-home \
    --uid "${UID}" \
    "${USER}"

WORKDIR /

COPY --from=BE /app/target/x86_64-unknown-linux-musl/release/preference-be /baracca

RUN mkdir /static
COPY --from=FE /usr/src/app/build /static
USER baracca:baracca

ARG STATIC_DIRECTORY=/static
ENV STATIC_DIRECTORY /static
ENV foo bar

CMD ["./baracca"]
