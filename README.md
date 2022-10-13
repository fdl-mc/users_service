# Users API Service
The main service for identifying users of the FDL ecosystem.


## Deploying guide

### Via Docker
1. Install the latest Docker version
2. Clone this repository
3. Run `make build` (or `docker build -t fdl-mc/api/users:v1 .`) in the project root directory to build an image
4. Run `make build` and then `make run` (or `make deploy`/`make deploy-shared` if you wanna run it as a daemon)

### Manually
1. Install the latest Python version
2. Clone this repository
3. Run `pip install -r requirements.txt` in the project root directory
4. Run the executable with `uvicorn users_service.main:app --proxy-headers --host 0.0.0.0 --port 8000`


## Environment variables
| Variable     | Purpose                                               |
|--------------|-------------------------------------------------------|
| JWT_SECRET   | JWT secret key                                        |
| DATABASE_URL | Database URL (supports Postgres, MySQL, SQlite, etc.) |


## License
The project is licensend under [GNU General Public License v3.0](https://github.com/fdl-mc/users_service/blob/main/LICENSE)