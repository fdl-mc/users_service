from pydantic import BaseSettings


class Settings(BaseSettings):
    postgres_url: str
    discord_client_id: str
    discord_client_secret: str
    discord_redirect_url: str


settings = Settings()
