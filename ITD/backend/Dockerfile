from rust:1.49-slim as builder
WORKDIR /usr/clup
COPY . .
RUN cargo install --path .

FROM debian:stable-slim
WORKDIR /usr/clup
COPY --from=builder /usr/local/cargo/bin/main .
CMD ["./main"]