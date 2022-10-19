docker compose -f ../docker-compose.yml -f ./docker-compose.override.yml -f ../docker-compose-host.yml down -v
docker compose -f ../docker-compose.yml -f ./docker-compose.override.yml -f ../docker-compose-host.yml up
