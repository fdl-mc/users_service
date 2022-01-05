import databases
import sqlalchemy
from users_microservice.api.config import settings

database = databases.Database(settings.postgres_url)
metadata = sqlalchemy.MetaData()
engine = sqlalchemy.create_engine(settings.postgres_url)
