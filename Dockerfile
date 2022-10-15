FROM python
WORKDIR /app

# install dependencies via poetry
COPY pyproject.toml poetry.lock ./
RUN pip install poetry
RUN poetry config virtualenvs.create false
RUN poetry install --no-root --only main

# cleanup
RUN pip uninstall poetry -y
RUN rm -rf /root/.cache

COPY users_service users_service

CMD ["uvicorn", "users_service.main:app", "--proxy-headers", "--host", "0.0.0.0"]