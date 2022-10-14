from fastapi import Depends, HTTPException, Security
from fastapi.security import HTTPAuthorizationCredentials
from jose import jwt

from users_service.models import User
from users_service.security import security
from users_service.settings import settings


async def jwt_claims(
    credentials: HTTPAuthorizationCredentials = Security(security),
) -> dict:
    try:
        return jwt.decode(credentials.credentials, settings.jwt_secret)
    except jwt.JWTError:
        raise HTTPException(401, "Invalid token")


async def authenticated_user(claims: dict = Depends(jwt_claims)) -> User:
    return await User.objects.get(id=claims["user_id"])


async def admin_user(user: User = Depends(authenticated_user)) -> User:
    if not user.admin:
        raise HTTPException(403, "You are not admin")
    return user
