set -euo pipefail

docker compose -f ./docker-compose.prod.yml up -d
docker rmi -f $(docker images -f "dangling=true" -q)
