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
sudo -iu postgres # Login into postgres user
initdb -D /var/lib/postgres/data # Initialize DB cluster
createuser -s diesel # Create passwordless superuser diesel
exit # Go back to user's shell
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

## Endpoints

* /
```
GET /
```

Responds `Ok`

* /user/register
```
POST /user/register
{
  "username": "<desired_username>"
  "password": "<desired_password>"
}
```

  * <desired_username>: username of the new user
  * <desired_password>: password of the new user
**NOTE*: Repeated usernames are not handled(yet)

* /user/login
```
POST /user/register
{
  "username": "<username_of_login>"
  "password": "<password_of_login>"
}
```

  * <username_of_login>: username of the login in user
  * <password_of_login>: password of the login in user
**NOTE**: User info stored is username in a cookie login

* /user/whoami
```
GET /user/whoami
```

Responds with the current logged in session username

## Tests

Only manual tests can be found for now:
```bash
./src/tests.sh
```
