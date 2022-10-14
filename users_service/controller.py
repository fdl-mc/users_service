from typing import Any
import ormar
from starlite import Body, Request
from starlite.controller import Controller
from starlite.handlers import get, post, put, patch
from starlite.exceptions import (
    NotFoundException,
    NotAuthorizedException,
    PermissionDeniedException,
)

from users_service.models import User, Credential
from users_service.schemas import (
    LoginRequest,
    LoginResponse,
    CreateUserRequest,
    ChangePasswordRequest,
)
from users_service.crypto import hash_password, generate_salt
from users_service.auth import security_requirement, auth_middleware, encode_jwt_token
from users_service.guards import admin_user_guard


class UserController(Controller):
    @get(
        path="/",
        description="Fetch all registered users.",
    )
    async def get_all_users(self) -> list[User]:
        return await User.objects.all()

    @get(
        "/{id:int}",
        raises=[NotFoundException],
        description="Fetch a user by their ID.",
    )
    async def get_user(self, id: int) -> User:
        try:
            return await User.objects.get(id=id)
        except ormar.NoMatch:
            raise NotFoundException(detail="User not found")

    @get(
        "/me",
        description="Get self user object.",
        raises=[NotAuthorizedException],
        security=[security_requirement],
        middleware=[auth_middleware],
    )
    async def get_self(self, request: Request) -> User:
        return request.user

    @get(
        "/find",
        description="Find users by username query",
    )
    async def find_users(self, username: str) -> list[User]:
        return await User.objects.filter(User.username.startswith(username)).all()

    @put(
        "/",
        description="Create a new user.",
        status_code=201,
        raises=[NotAuthorizedException, PermissionDeniedException],
        guards=[admin_user_guard],
        security=[security_requirement],
        middleware=[auth_middleware],
    )
    async def create_user(
        self,
        data: CreateUserRequest = Body(),
    ) -> None:
        user = await User.objects.create(username=data.username)
        salt = generate_salt()
        password = hash_password(data.password, salt)
        await Credential.objects.create(user=user, password=password, salt=salt)

    @post(
        "/login",
        raises=[NotAuthorizedException],
        description="Get a token by username and password.",
        status_code=200,
    )
    async def login(self, data: LoginRequest = Body()) -> LoginResponse:
        try:
            credential: Credential = await Credential.objects.get(
                Credential.user.username == data.username
            )
        except ormar.NoMatch:
            raise NotAuthorizedException(detail="Wrong username or password")

        if credential.password == hash_password(data.password, credential.salt):
            token = encode_jwt_token(str(credential.user.id))
            return LoginResponse(token=token)
        else:
            raise NotAuthorizedException(detail="Wrong username or password")

    @patch(
        "/password",
        description="Change your password.",
        status_code=204,
        raises=[NotAuthorizedException],
        security=[security_requirement],
        middleware=[auth_middleware],
    )
    async def change_password(
        self,
        request: Request[User, Any],
        data: ChangePasswordRequest = Body(),
    ) -> None:
        salt = generate_salt()
        new_password = hash_password(data.new_password, salt)

        await Credential.objects.filter(user=request.user.id).update(
            password=new_password, salt=salt
        )
