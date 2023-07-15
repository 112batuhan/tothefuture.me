set dotenv-load := true

start-postgres: 
    docker compose -f docker-compose.yml up postgres -d --force-recreate

start-redis:
    docker compose -f docker-compose.yml up redis -d --force-recreate

start-db: start-postgres start-redis

start-pgadmin:
    docker compose -f docker-compose.yml up pgadmin -d --force-recreate

stop-everything:
    docker compose -f docker-compose.yml down --remove-orphans

generate-entities:
    cd backend/migration && just generate-entities-main-command

migrate-refresh:
    cd backend && sea-orm-cli migrate refresh

update-db: start-postgres migrate-refresh generate-entities
