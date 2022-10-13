FROM python
WORKDIR /app

COPY requirements.txt .
RUN pip install --no-cache-dir --upgrade -r requirements.txt

COPY . .

CMD ["uvicorn", "users_service.main:app", "--proxy-headers", "--host", "0.0.0.0"]