FROM python
WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir -r requirements.txt
COPY users_service users_service

CMD ["uvicorn", "users_service.main:app", "--proxy-headers", "--host", "0.0.0.0", "--port", "8010"]