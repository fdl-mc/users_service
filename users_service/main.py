from starlite import Starlite, OpenAPIConfig

from users_service.database import engine, metadata
from users_service.hooks import startup, shutdown
from users_service.controller import UserController
from users_service.auth import security_components

metadata.create_all(engine)

app = Starlite(
    route_handlers=[UserController],
    on_startup=[startup],
    on_shutdown=[shutdown],
    openapi_config=OpenAPIConfig(
        title="FDL Users Service",
        version="1.0.0",
        components=[security_components],
    ),
)
