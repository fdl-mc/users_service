import ormar
from users_service.database import database, metadata


class User(ormar.Model):
    class Meta:
        tablename = "users"
        database = database
        metadata = metadata

    id: int = ormar.Integer(primary_key=True)
    username: str = ormar.String(max_length=16, unique=True)
    admin: bool = ormar.Boolean(default=False)


class Credential(ormar.Model):
    class Meta:
        tablename = "credentials"
        database = database
        metadata = metadata

    id: int = ormar.Integer(primary_key=True)
    user: User = ormar.ForeignKey(User, skip_reverse=True)
    password: str = ormar.String(max_length=128)
    salt: str = ormar.String(max_length=128)
