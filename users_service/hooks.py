from users_service.database import database


async def startup() -> None:
    if not database.is_connected:
        await database.connect()


async def shutdown() -> None:
    if database.is_connected:
        await database.disconnect()
