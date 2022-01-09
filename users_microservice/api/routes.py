from fastapi.routing import APIRouter
from users_microservice.api.models import User
from users_microservice.api.discord import discord


users = APIRouter()


@users.get("/", response_model=list[User])
async def get_all_users():
    return await User.objects.all()


@users.get("/callback")
async def auth_callback(code: str):
    token, _ = await discord.get_access_token(code)
    user_data = await discord.request("/users/@me", token)
    return await User.objects.get_or_create(discord_id=user_data["id"])


@users.get("/{id}", response_model=User)
async def get_user(id: int):
    return await User.objects.get(id=id)
