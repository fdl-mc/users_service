FROM python:3.11 as builder
ENV PATH=/root/.local/bin:$PATH
COPY requirements.txt .
RUN pip install --user --no-cache-dir -r requirements.txt

FROM python:3.11-slim
ENV PATH=/root/.local/bin:$PATH
RUN apt update && apt install libpq5 -y
COPY --from=builder /root/.local /root/.local
COPY users_service users_service
CMD ["uvicorn", "users_service.main:app", "--proxy-headers", "--host", "0.0.0.0", "--port", "8010"]