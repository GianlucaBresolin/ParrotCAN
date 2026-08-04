FROM rust:1.85-slim AS builder
WORKDIR /usr/src/app
COPY . .
RUN cargo build --release

FROM debian:bookworm-slim
COPY --from=builder /usr/src/app/target/release/virtual_can_bus usr/local/bin/virtual_can_bus
EXPOSE 8080
CMD ["virtual_can_bus"]