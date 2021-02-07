# Integration and Testing Deliverable
## De Martini Luca - Duico Alessandro

[Application Server source code](https://github.com/luca-de-martini/DeMartiniDuico-sw2/tree/main/ITD/backend)

[Client Web Application source code](https://github.com/luca-de-martini/DeMartiniDuico-sw2/tree/main/ITD/frontend)

## Introduction
Clup is a three-tier system, consisting of a Database (Postgres), the Clup binary (`ITD/backend`) and the Clup Webapp (`ITD/frontend`). A middleware (Redis) is employed for session storage.
### Scope
This document gives a concrete description of the Implementation and Testing procedure. 
In particular, it covers:
- the reasons for choosing those requirements that are implemented;
- the adopted development technologies: languages, frameworks and API standards;
- the structure of the source code;
- how testing is performed;
-  the prerequisites and instructions for building and installing.

## Implemented features

Being a Proof-of-Concept, only the core features for generating tickets to line-up at a Shop were implemented, plus the monitoring functions available to the staff. 

Referring to the DD, the following requirements are satisfied:

- **[R1]** The system shall keep track of the list of Customers waiting to visit each Shop
- **[R2] **The system shall allow customers to request the right to visit a shop as soon as possible
- **[R3]** The system shall give Customers a Token associated with their position in the waiting line
- **[R5]** The Staff shall be able to scan Customer generated Tokens using a textual input
- **[R6]** Given a Token, the Staff application shall be able to verify its validity
- **[R7]** Given a Token, the Staff application shall be able to verify its position in the waiting line
- **[R8]** Given a Token, the Staff application shall be able to mark it as used and update the list of Customers currently inside the Shop
- **[R12]** The  system  shall ask Customers  to  specify the approximate duration of their visit
- **[R13]** The system shall automatically infer an estimate visit duration for returning customers
- **[R14]** The system shall give Customers an estimate of the waiting time remaining before it’s their turn
- **[R16]** The system shall be able to connect to a Maps service to show information about travel time
- **[R17]** Customers shall be able to specify the categories of items they intend to buy
- **[R18]** The system shall keep track of the number of customers visiting the Shop on a Department basis

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

Testing has been done following the Design Documents guidelines. Unit tests are written alongside the code for the model using a bottom up approach.  
Integration tests are written in the `ITD/backend/tests` directory and test the API via simulated requests. The source code is self-explanatory, there is no need to explain them here but in a very concise way: 

- `account_api_test.rs`verifies  the main steps of account creation

- `get_ticket_test.rs` simulates the process of obtaining tickets

- `full_enter_exit_test.rs` generates more complex multi-Customer situations, to check the constraints of the queue

GitLab CI is used as a Continuous Integration platform.  This way, every commit triggers the execution of the workflow in `.github/rust-ci.yml`that broken commits made on feature branches were never merged into the main branch.

## Installation instructions

### Using docker-compose

 + Install docker: https://docs.docker.com/get-docker/
 + Install docker-compose: https://docs.docker.com/compose/install/
 + Build `docker-compose build`
 + Set the `SESSION_KEY` environment variable ([see usage](#usage))
 + Run `docker-compose up`

### Building and installing  - Backend

#### Requirements

 + **Rust** https://www.rust-lang.org/tools/install

 + **Sqlx CLI (Optional)** `cargo install sqlx-cli --no-default-features --features postgres`

 + **PostgreSQL** https://www.postgresql.org/

 + **Redis** https://redis.io/


#### Environment variables
Configuration for the main binary is supplied through environment variables. Example:
```
DATABASE_URL="postgresql://user:pass@localhost:5432/clup_sqlx"
REDIS_URL="127.0.0.1:6379"
SESSION_KEY="0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f0f"
API_URL="0.0.0.0:5000" # defaults to "0.0.0.0:5000"
```
When running the application server binary environment variable configurations will be read from a `.env` file, if present.

#### Working directory
```
cd ITD/backend
```
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
#### Run
```
cargo run
```

### Building and installing  - Frontend

#### Requirements

 + **Node.js** https://nodejs.org/en/download/
 + **Yarn** (or any other package manager for Node.js)  https://yarnpkg.com/getting-started/install

#### Environment variables
```
VUE_APP_API_BASE_URL="http://localhost:5000" # should point to API_URL
```

#### Working directory
```
cd ITD/frontend
```

#### Install
```
yarn install
```

#### Run
```
yarn run serve
```
This command will start a development web server that is not suited for production. 

Alternatively, you can run:

```bash
yarn run build
```
and use a web server (such as Nginx or Lighttpd) to serve the `dist` folder, which is done automatically when **Using docker-compose**.
