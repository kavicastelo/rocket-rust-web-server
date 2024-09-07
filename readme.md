# Rocket Web Server with Diesel & PostgreSQL

This project is a web server built with [Rocket](https://rocket.rs), using [Diesel](https://diesel.rs) for database
management and PostgreSQL as the database.

## Features

- REST API built with Rocket
- PostgreSQL integration via Diesel ORM
- CRUD operations for database entities

## Requirements

- Rust & Cargo (latest stable version)
- PostgreSQL installed and running
- Diesel CLI
- Windows (with workaround for Diesel issue) or Linux/MacOS

## Setup

1. Install Rust:

   Follow the instructions at [Rust's official site](https://www.rust-lang.org/tools/install).
2. Install PostgreSQL:

   Download and install PostgreSQL from [PostgreSQL's official site](https://www.postgresql.org/download/).
3. Install Diesel CLI:

   Install the Diesel CLI with the following command:
   ```shell
   cargo install diesel_cli --no-default-features --features postgres
   ```
4. Set Up PostgreSQL Database:
    - Ensure PostgreSQL is running.
    - Create a database.
   ```shell
   createdb rocket_server
   ```
    - Update your rocket.toml file with the database URL:
   ```toml
   [global.databases]
   postgres_db = { url = "postgres://postgres:yourpassword@localhost/rocket_server" }
   ```
5. Setup Environment (Windows only workaround):
   If running on
   Windows, [follow this solution](https://github.com/diesel-rs/diesel/discussions/2947#discussioncomment-2025857.
6. Run Migrations:
   Create the Diesel migrations:
   ```shell
   diesel migration run
   ```
7. Run the Server:
   Run the Rocket server with:
   ```shell
   $env:ROCKET_LOG="debug"; cargo run # for Windows
   ROCKET_LOG=debug cargo run         # for Linux/MacOS
   ```
8. Access the server:
   The server will be available at `http://localhost:8000`.

## Troubleshooting

- If you're facing issues on Windows with Diesel, it’s a known problem. You may try running the project inside WSL or a
  Linux VM for better compatibility.

## License

This project is licensed under the [MIT license](LICENSE).
