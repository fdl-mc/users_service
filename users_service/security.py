from fastapi.security import HTTPBearer

security = HTTPBearer(scheme_name="jwt-token")
