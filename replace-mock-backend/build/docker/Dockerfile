FROM rust:1.70.0 AS builder
WORKDIR /app
COPY . .
RUN cargo build --release

FROM gcr.io/distroless/cc
COPY --from=builder /app/target/release/replace-mock-backend /
ENTRYPOINT ["./replace-mock-backend"]
