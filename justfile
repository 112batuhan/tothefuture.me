start-server: 
    docker compose -f docker-compose.yml up postgres -d --force-recreate

start-pgadmin:
    docker compose -f docker-compose.yml up pgadmin -d --force-recreate
