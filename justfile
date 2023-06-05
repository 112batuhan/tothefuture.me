start-server DOCKER_SERVICES="all": 
    COMPOSE_PROFILES={{DOCKER_SERVICES}} 
    docker compose -f docker-compose.yml up --force-recreate
