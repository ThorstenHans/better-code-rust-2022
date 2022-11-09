.PHONY build run run-azure create-local-db create-azure-db kill-local kill-local-api-only create-k8s-secret:

build:
	@echo "Building Docker Image"
	@docker build . -t http-api:latest

run: 
	@echo "Starting PostgreSQL locally"
	@docker network create http-api-network --driver bridge
	@docker run -d -p 5432:5432 --name postgres --network http-api-network -e POSTGRES_PASSWORD=mysecretpassword postgres:alpine
	@docker run -d -p 8080:8080 --name api --read-only --network http-api-network -e DATABASE_URL=postgres://postgres:mysecretpassword@postgres/products http-api:latest

create-local-db:
	@diesel database setup && diesel migration run

create-azure-db: 
	@diesel database setup --DATABASE_URL=$(AZURE_PG)
	@diesel migration run --DATABASE_URL=$(AZURE_PG)

kill-local-api-only:
	@echo "Killing API container"
	@docker rm -f api

kill-local:
	@echo "killing local deployment"
	@docker rm -f postgres api
	@docker network rm http-api-network

run-azure:
	@docker run -d -p 8080:8080 --name api -e DATABASE_URL=$(AZURE_PG) http-api:latest

create-k8s-secret:
	@echo "Creating k8s secret"
	@echo "Provide your PostgreSQL Connection String: "; \
		read CSTR; \
	 	kubectl create secret generic azure-postgresql --from-literal=connectionString=$$CSTR --namespace=http-api
