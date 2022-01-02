from pydantic import BaseSettings


class Settings(BaseSettings):
    postgres_url: str


settings = Settings()
