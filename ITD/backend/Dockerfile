from rust:1.49-alpine
RUN apk add --no-cache musl-dev
WORKDIR /usr/clup
COPY . .
RUN cargo install --path .

CMD ["main"]