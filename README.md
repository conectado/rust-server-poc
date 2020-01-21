# Rust server POC

## Description

This is a POC to test Rust current capabilities as a web-server

We have chosen to use
* Actix (as a webserver framework)
* Diesel (as an ORM library)

We have chosen Actix due to its ability to work on stable and its simplicity and it seemingly fast response times

We have chosen Diesel to easily abstract from the database as it seems popular for this job

## Prerequisities
* Git

## Setup this project
```bash
git clone git@github.com:conectado/rust-server-poc.git # Clone repo
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh # Install rustup
sudo pacman -S postgresql # Install posgres
cargo install diesel_cli # Install diesel cli for DB
diesel setup # Setup DB and tables
cargo install # Install dependencies
```

## Run this project

```bash
cargo run
```

## Run with auto-reload
```bash
systemfd --no-pid -s http::3000 -- cargo watch -x run
```
