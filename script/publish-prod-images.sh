set -euo pipefail

set -a
if [ -f .env ]; then
  source .env
fi
set +a

TAG=${TAG:-latest}
INERTIA_BACKEND_IMAGE="${REPOSITORY}/inertia-backend:${TAG}"
INERTIA_WEB_IMAGE="${REPOSITORY}/inertia-web:${TAG}"

docker build -f inertia-async-server/dockerfile -t ${INERTIA_BACKEND_IMAGE} --target prod .
docker build -f inertia-web/dockerfile -t ${INERTIA_WEB_IMAGE} --target prod .

aws ecr get-login-password | docker login --username AWS --password-stdin ${REPOSITORY}

docker push ${INERTIA_BACKEND_IMAGE}
docker push ${INERTIA_WEB_IMAGE}
