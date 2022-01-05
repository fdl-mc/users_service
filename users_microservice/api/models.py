import ormar
from users_microservice.api.database import database, metadata


class User(ormar.Model):
    class Meta:
        tablename: str = "users"
        database = database
        metadata = metadata

    id: int = ormar.Integer(primary_key=True)
    discord_id: str = ormar.String(max_length=64)
    nickname: str = ormar.String(max_length=16, nullable=True)
    balance: int = ormar.Integer(default=0)
