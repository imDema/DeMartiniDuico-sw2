# CLup application server
## De Martini Luca - Duico Alessandro

[Application Server](https://github.com/luca-de-martini/DeMartiniDuico-sw2/tree/main/ITD/backend)

TODO: link to code

## Introduction

## Implemented features

## Adopted frameworks and languages


### Actix

Actix is a [A powerful, pragmatic, and extremely fast web framework for Rust](https://actix.rs/).

It was chosen for the implementation for three main reasons: Actor model concurrency, performance and safety.

### Vue.js

### PostgreSQL

[PostgreSQL is The World's Most Advanced Open Source Relational Database](https://www.postgresql.org/)

PostgreSQL is a powerful, open source object-relational database system with over 30 years of active development that has earned it a strong reputation for reliability, feature robustness, and performance.

### Redis

[Redis is an open source, in-memory data structure store, used as a database, cache, and message broker](https://redis.io/)

It provides a fast memory data structure that can be used for authentication and session storage without putting load on the relational database

## Source code structure

### Backend

The code for the application server is contained in the `backend` directory.

+ In the root the `Cargo.toml` file lists dependencies and the packages' configuration

+ The `migrations` directory contains the SQL files for the database migrations

+ The `tests` directory contains the integration tests for the backend

+ The `src` directory contains the Rust source files for the application server. This is the module structure:
  + `bin` contains the executable
  + `api` contains the endpoints for the API (MVC Controller)
  + `models` contains the models and associated Data Access Objects (MVC Model)
  + `utils` contains utils functions for session state, url encoding and tests



### Frontend

The code for the application server is contained in the `frontend` directory.

TODO:

## Testing

Testing has been done following the Design Documents guidelines, unit tests are written alongside the code for the model using a bottom up approach. Integration tests are written in the `tests` directory of the backend and test the API via simulated requests

## Installation instructions

### Using docker-compose

 + Install docker: https://docs.docker.com/get-docker/
 + Install docker-compose: https://docs.docker.com/compose/install/
 + Build `docker-compose build`
 + Set the `SESSION_KEY` environment variable ([see usage](#usage))
 + Run `docker-compose up`

### Building and installing

#### Requirements
 + **Rust** https://www.rust-lang.org/tools/install
 + **Sqlx CLI (Optional)** `cargo install sqlx-cli --no-default-features --features postgres`
 + **PostgreSQL** https://www.postgresql.org/
 + **Redis** https://redis.io/

#### Build
```
cargo build
```

#### Test
```
cargo test
```

#### Install
```
cargo install
```


### Usage

Configuration for the main binary is supplied through envirnoment variables. Example:
```
DATABASE_URL="postgresql://user:pass@localhost:5432/clup_sqlx"
REDIS_URL="127.0.0.1:6379"
SESSION_KEY="0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f"
API_URL="0.0.0.0:5000" # defaults to "0.0.0.0:5000"
```

When running the application server binary environment variable configurations will be read from a `.env` file, if present.