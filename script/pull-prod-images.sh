set -euo pipefail

set -a
if [ -f .env ]; then
  source .env
fi
set +a

TAG=${TAG:-latest}
INERTIA_BACKEND_IMAGE="${REPOSITORY}/inertia-backend:${TAG}"
INERTIA_WEB_IMAGE="${REPOSITORY}/inertia-web:${TAG}"

aws ecr get-login-password | docker login --username AWS --password-stdin ${REPOSITORY}

docker pull jc21/nginx-proxy-manager:latest
docker pull ${INERTIA_BACKEND_IMAGE}
docker pull ${INERTIA_WEB_IMAGE}
