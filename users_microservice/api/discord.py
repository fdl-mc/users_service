from fastapi_discord import DiscordOAuthClient
from users_microservice.api.settings import settings

discord = DiscordOAuthClient(
    settings.discord_client_id,
    settings.discord_client_secret,
    settings.discord_redirect_url,
)
