import ormar
from fastapi import Body, Depends, HTTPException
from fastapi.routing import APIRouter
from jose import jwt

from users_service.api.crypto import generate_salt, hash_password
from users_service.api.deps import admin_user, authenticated_user
from users_service.api.models import Credential, User
from users_service.api.schemas import (ChangePasswordRequest,
                                       CreateUserRequest, LoginRequest,
                                       LoginResponse)
from users_service.api.settings import settings

router = APIRouter()


@router.get("/", response_model=list[User])
async def get_all_users():
    return await User.objects.all()


@router.get("/me", response_model=User)
async def get_self(user: User = Depends(authenticated_user)):
    return user


@router.get("/find", response_model=list[User])
async def find_users(username: str):
    return await User.objects.filter(User.username.startswith(username)).all()


@router.get("/{id}", response_model=User)
async def get_user(id: int):
    try:
        return await User.objects.get(id=id)
    except ormar.NoMatch:
        raise HTTPException(404, detail="User not found")


@router.put("/", status_code=204)
async def create_user(
    user: User = Depends(admin_user), payload: CreateUserRequest = Body(...)
):
    user = await User.objects.create(username=payload.username)
    salt = generate_salt()
    password = hash_password(payload.password, salt)
    await Credential.objects.create(user=user, password=password, salt=salt)


@router.post("/login", response_model=LoginResponse)
async def login(payload: LoginRequest = Body(...)):
    try:
        credential = await Credential.objects.get(
            Credential.user.username == payload.username
        )
    except ormar.NoMatch:
        raise HTTPException(403, detail="Wrong username or password")

    if credential.password == hash_password(payload.password, credential.salt):
        token = jwt.encode({"user_id": credential.user.id}, settings.jwt_secret)
        return {"token": token}
    else:
        raise HTTPException(403, detail="Wrong username or password")


@router.patch("/password", status_code=204)
async def change_password(
    user: User = Depends(authenticated_user), payload: ChangePasswordRequest = Body(...)
):
    salt = generate_salt()
    new_password = hash_password(payload.new_password, salt)

    await Credential.objects.filter(user=user.id).update(
        password=new_password, salt=salt
    )
