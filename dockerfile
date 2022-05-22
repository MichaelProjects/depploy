FROM rust:1.58.1 as builder
ENV USER root
COPY . .
RUN rustup default nightly
RUN rustup update
RUN cargo build --release

FROM ubuntu:21.04
RUN apt update && apt install postgresql-client -y
COPY --from=builder /target/release/{{app_name}} {{app_name}}
RUN useradd -u 8877 notroot

RUN mkdir -p /var/log/{{app_name}}/
RUN chmod -R 777 /var/log/{{app_name}}
ENV RUST_LOG=debug
USER notroot
CMD ["./{{app_name}}"]
