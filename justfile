set dotenv-load := true

start-db: 
    docker compose -f docker-compose.yml up postgres -d --force-recreate

start-pgadmin:
    docker compose -f docker-compose.yml up pgadmin -d --force-recreate

stop-everything:
    docker compose -f docker-compose.yml down --remove-orphans

generate-entities:
    sea-orm-cli generate entity -u $DATABASE_URL -o backend/src/entities

migrate-refresh:
    cd backend && sea-orm-cli migrate refresh
