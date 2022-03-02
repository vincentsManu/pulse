FROM rust:1.58.1 as planner
WORKDIR /app

RUN cargo install cargo-chef
COPY . .
RUN cargo chef prepare --recipe-path recipe.json

#
FROM rust:1.58.1 as cacher
WORKDIR /app
RUN cargo install cargo-chef
COPY --from=planner /app/recipe.json recipe.json
RUN cargo chef cook --release --recipe-path recipe.json

#
FROM rust:1.58.1 as builder

WORKDIR /app
COPY . .
ENV SQLX_OFFLINE true
COPY --from=cacher /app/target target
COPY --from=cacher /usr/local/cargo /usr/local/cargo
RUN cargo build --release --bin pulstats

#
FROM gcr.io/distroless/cc-debian10:nonroot

COPY --from=builder /app/target/release/pulstats pulstats
COPY configuration configuration
ENTRYPOINT ["./pulstats"]