from pydantic import BaseSettings


class Settings(BaseSettings):
    database_url: str
    jwt_secret: str


settings = Settings()
