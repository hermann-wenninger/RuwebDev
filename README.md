# Rust wasm React Docker - Postgres - Json todo webdev 
## Running one of the rust server's

cargo run -p to-do-actix-server


or:

cargo run -p to-do-axum-server

or:

cargo run -p to-do-rocket-server

or:

cargo run -p to-do-hyper-server


### Install wasma pack

cargo install wasm-pack

### build wasm
cd ../rust-interface
wasm-pack build --target web

### build react frontend
cd ../frontend
npm run build

## start docker postgres image
docker compose up

## Commands

+ cargo install sqlx-cli
+ sqlx migration add initial-setup
