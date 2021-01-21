# CLup application server

## Usage

Configuration for the main binary is supplied through envirnoment variables. Example:
```
DATABASE_URL="postgresql://user:pass@localhost:5432/clup_sqlx"
REDIS_URL="127.0.0.1:6379"
SESSION_KEY="0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f"
API_URL="0.0.0.0:5000" # defaults to "0.0.0.0:5000"
```

### Using docker-compose

To build and deploy using docker and docker compose
```
docker-compose up -d
```
Note: the `SESSION_KEY` variable must still be set

## Building and installing

### Requirements
 + **Rust** https://www.rust-lang.org/tools/install
 + **Sqlx CLI (Optional)** `cargo install sqlx-cli --no-default-features --features postgres`
 + **PostgreSQL** https://www.postgresql.org/
 + **Redis** https://redis.io/

### Build
```
cargo build
```

### Test
```
cargo test
```

### Install
```
cargo install
```
