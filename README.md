# Users API Service
The main service for identifying users of the FDL ecosystem.


## Deploying guide

### Via Docker
1. Install the latest Docker version
2. Clone this repository
3. Run `make build` (or `docker build -t fdl-mc/api/users .`) in the project root directory to build an image
4. Run `make build` and then `make run` (or `make deploy` if you wanna run it as a daemon)

### Manually
1. Install the latest Rust version
2. Clone this repository
3. Run `cargo build --release` in the project root directory
4. Run the executable in `./target/release/users_service`


## Environment variables
| Variable     | Purpose                                            |
|--------------|----------------------------------------------------|
| JWT_SECRET   | JWT secret key                                     |
| DATABASE_URL | Database URL (supports Postgres)                   |


## License
The project is licensend under [MIT License](https://github.com/fdl-mc/users_service/blob/main/LICENSE)