import databases
import sqlalchemy

from users_service.settings import settings

database = databases.Database(settings.database_url)
metadata = sqlalchemy.MetaData()
engine = sqlalchemy.create_engine(settings.database_url)
