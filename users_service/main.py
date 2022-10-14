from fastapi import FastAPI
from fastapi.middleware.cors import CORSMiddleware

from users_service.database import database, engine, metadata
from users_service.routes import router

metadata.create_all(engine)

app = FastAPI(
    title="Users API",
    description="The main service for identifying users of the FDL ecosystem.",
    generate_unique_id_function=lambda r: r.name,
)

app.include_router(router)
app.add_middleware(
    CORSMiddleware,
    allow_origins=["*"],
    allow_credentials=True,
    allow_methods=["*"],
    allow_headers=["*"],
)


@app.on_event("startup")
async def startup() -> None:
    if not database.is_connected:
        await database.connect()


@app.on_event("shutdown")
async def shutdown() -> None:
    if database.is_connected:
        await database.disconnect()
