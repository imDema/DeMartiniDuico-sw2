# CLup application server
## De Martini Luca - Duico Alessandro

[Application Server](https://github.com/luca-de-martini/DeMartiniDuico-sw2/tree/main/ITD/backend)

[Client Web Application](https://github.com/luca-de-martini/DeMartiniDuico-sw2/tree/main/ITD/frontend)

TODO: link to code

## Introduction

## Implemented features

## Adopted frameworks and languages


### Actix

Actix is a [A powerful, pragmatic, and extremely fast web framework for Rust](https://actix.rs/).

It was chosen for the implementation for three main reasons: Actor model concurrency, performance and safety.

### PostgreSQL

[PostgreSQL is The World's Most Advanced Open Source Relational Database](https://www.postgresql.org/)

PostgreSQL is a powerful, open source object-relational database system with over 30 years of active development that has earned it a strong reputation for reliability, feature robustness, and performance.

### Redis

[Redis is an open source, in-memory data structure store, used as a database, cache, and message broker](https://redis.io/)

It provides a fast memory data structure that can be used for authentication and session storage without putting load on the relational database

### Vue.js

[Vue.js](https://vuejs.org/v2/guide/) is an open-source MVVM (model–view–viewmodel) front end JavaScript framework for building user interfaces and single-page applications.

Its main features are:

- reusability of Components, defined with the use of an HTML-based syntax that allows binding the rendered DOM to the underlying Vue instance's data;
- reactivity, that means each component keeps track of its dependencies during its render, so the system knows precisely when to re-render, and which components to re-render;
- supports single-page applications. Instead of the default method of the browser loading entire new pages, it interacts with the user by dynamically rewriting the current web page with new data. The consistency of the navigation history is kept by the "vue-router" package, which provides an API to update the application's URL.

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

- `package.json` lists the main info about the Node.js package, provides release and debug dependencies and useful scripts that can be executed with `yarn run <script>`
- `App.vue` is the main Vue.js component, which contains every other page

- The `public` directory hosts static files that are served as they are by the webserver

- The `src` directory contains the source code for the reactive part of the Web Application:
    - `assets` — any assets that are imported into your components
    - `components`— all the components of the projects that are not inside `views`
    - `router` — scripts that handle the routing
    - `store` — scripts related to the Vuex store
    - `translations` — locales files, not present
    - `views` — the components that are routed. They represent the pages of traditional HTML.

## Testing

Testing has been done following the Design Documents guidelines. Unit tests are written alongside the code for the model using a bottom up approach. Integration tests are written in the `tests` directory of the backend and test the API via simulated requests

## Installation instructions

### Using docker-compose

 + Install docker: https://docs.docker.com/get-docker/
 + Install docker-compose: https://docs.docker.com/compose/install/
 + Build `docker-compose build`
 + Set the `SESSION_KEY` environment variable ([see usage](#usage))
 + Run `docker-compose up`

### Building and installing

#### Requirements

##### Backend

 + **Rust** https://www.rust-lang.org/tools/install

 + **Sqlx CLI (Optional)** `cargo install sqlx-cli --no-default-features --features postgres`

 + **PostgreSQL** https://www.postgresql.org/

 + **Redis** https://redis.io/

  ##### Frontend

 + **Node.js** https://nodejs.org/en/download/
 + **Yarn** (or any other package manager for Node.js)  https://yarnpkg.com/getting-started/install

#### Build
##### Backend
```
cargo build
```

#### Test
##### Backend
```
cargo test
```

#### Install
##### Backend
```
cargo install
```
##### Frontend
```
yarn install
```

### Usage

Configuration for the main binary is supplied through environment variables. Example:
```
DATABASE_URL="postgresql://user:pass@localhost:5432/clup_sqlx"
REDIS_URL="127.0.0.1:6379"
SESSION_KEY="0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f"
API_URL="0.0.0.0:5000" # defaults to "0.0.0.0:5000"

VUE_APP_API_BASE_URL="http://localhost:5000" # should point to API_URL
```

When running the application server binary environment variable configurations will be read from a `.env` file, if present.

#### Running
##### Backend
```
cargo run
```
##### Frontend
```
yarn run serve
```

This command will start a development web server that is not suited for production. Alternatively you can run:

```bash
yarn run build
```

and use a web server (such as Nginx or Lighttpd) to serve the `dist` folder, which is done automatically when **Using docker-compose**.

