FROM python

WORKDIR /app

ENV PYTHONDONTWRITEBYTECODE 1
ENV PYTHONUNBUFFERED 1

COPY requirements.txt ./
RUN pip install -r requirements.txt

COPY . ./

CMD ["uvicorn", "users_microservice.main:app", "--host", "0.0.0.0"]

