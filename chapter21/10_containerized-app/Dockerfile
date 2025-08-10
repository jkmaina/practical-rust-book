# Build stage
FROM rust:1.68 as builder
WORKDIR /app
COPY . .
RUN cargo build --release
# Runtime stage
FROM debian:bullseye-slim
WORKDIR /app
COPY --from=builder /app/target/release/containerized-app .
EXPOSE 8080
CMD ["./containerized-app"]
