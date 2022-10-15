from typing import cast, Any

import ormar
from jose import JWTError, jwt
from pydantic import BaseModel
from pydantic_openapi_schema.v3_1_0 import (
    Components,
    SecurityRequirement,
    SecurityScheme,
)
from starlite import (
    AbstractAuthenticationMiddleware,
    ASGIConnection,
    AuthenticationResult,
)
from starlite.exceptions import NotAuthorizedException
from starlite.middleware.base import DefineMiddleware
from starlite_jwt import JWTAuth

from users_service.models import User
from users_service.settings import settings


class Token(BaseModel):
    sub: str


def decode_jwt_token(encoded_token: str) -> Token:
    try:
        return Token(**jwt.decode(token=encoded_token, key=settings.jwt_secret))
    except JWTError as e:
        raise NotAuthorizedException("Invalid token") from e


def encode_jwt_token(user_id: str) -> str:
    return jwt.encode(Token(sub=user_id).dict(), settings.jwt_secret)


class JWTAuthenticationMiddleware(AbstractAuthenticationMiddleware):
    async def authenticate_request(
        self, connection: ASGIConnection[Any, Any, Any]
    ) -> AuthenticationResult:
        auth_header = connection.headers.get("x-token")
        if not auth_header:
            raise NotAuthorizedException("No token provided")

        token = decode_jwt_token(encoded_token=auth_header)

        try:
            user = await User.objects.get(id=int(token.sub))
        except ormar.NoMatch:
            raise NotAuthorizedException("User associated with this token not found")

        return AuthenticationResult(user=user, auth=token)


auth_middleware = DefineMiddleware(JWTAuthenticationMiddleware, exclude="schema")

security_components: Components = Components(
    securitySchemes={
        "Authentication token": SecurityScheme(
            type="apiKey",
            name="x-token",
            security_scheme_in="header",
        )
    }
)

security_requirement: SecurityRequirement = {"Authentication token": []}
