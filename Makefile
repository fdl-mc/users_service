build:
	@docker build -t fdl-mc/api/users .
deploy:
	@docker-compose up -d
