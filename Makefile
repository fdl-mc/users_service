build:
	@docker build -t fdl-mc/api/users:v1 .
run:
	@docker run fdl-mc/api/users:v1
deploy:
	@docker-compose up -d
deploy-shared:
	@docker-compose -f docker-compose_shared.yml up -d