set -euo pipefail

docker compose -f ./docker-compose.prod.yml up -d
docker rmi $(docker images -f "dangling=true" -q)
