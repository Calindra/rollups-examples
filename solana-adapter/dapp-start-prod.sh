
# clean
docker compose -f ../docker-compose.yml -f ./docker-compose.override.yml down -v

# build
docker buildx bake --load

# Run
docker compose -f ../docker-compose.yml -f ./docker-compose.override.yml up