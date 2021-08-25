FROM rust:1.54 as builder
WORKDIR /usr/src/rocket-demo
COPY . .
RUN cargo install --path .

FROM debian:buster-slim
RUN apt-get update && apt-get install -y postgresql && rm -rf /var/lib/apt/lists/*
COPY --from=builder /usr/local/cargo/bin/rocket-demo /usr/local/bin/rocket-demo
CMD ["rocket-demo"]
