ARG DATABASE_URL
ARG FRONTEND_URL
ARG REDIS_URL
ARG RESEND_API_TOKEN
ARG RESEND_EMAIL_ADDRESS
ARG PORT

FROM rust:latest AS builder
COPY . .
RUN rm -rf ./migration
RUN cargo build --release

FROM debian:buster-slim
RUN apt-get update & apt-get install -y extra-runtime-dependencies & rm -rf /var/lib/apt/lists/*
COPY --from=builder ./target/release/backend ./target/release/backend
ENV PORT=$PORT
ARG DATABASE_URL=$DATABASE_URL
ARG FRONTEND_URL=$FRONTEND_URL
ARG REDIS_URL=$REDIS_URL
ARG RESEND_API_TOKEN=$RESEND_API_TOKEN
ARG RESEND_EMAIL_ADRESS=$RESEND_EMAIL_ADRESS

CMD ["/target/release/backend"]
