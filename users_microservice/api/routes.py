from fastapi.routing import APIRouter
from users_microservice.api.models import User


users = APIRouter()


@users.get("/", response_model=list[User])
async def get_all_users():
    return await User.objects.all()


@users.get("/{id}", response_model=User)
async def get_user(id: int):
    return await User.objects.get(id=id)
