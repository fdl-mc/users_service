from typing import Any

from starlite import BaseRouteHandler, Request
from starlite.exceptions import PermissionDeniedException

from users_service.models import User


def admin_user_guard(request: Request[User, Any], _: BaseRouteHandler[Any]) -> None:
    if not request.user.admin:
        raise PermissionDeniedException("You are not admin")
