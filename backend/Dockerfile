FROM rust:latest AS builder
COPY . .
RUN rm -rf ./migration
RUN cargo build --release

FROM debian:bullseye-slim
COPY --from=builder ./target/release/backend ./target/release/backend
ENV PORT 3040
EXPOSE $PORT

CMD ["/target/release/backend"]
