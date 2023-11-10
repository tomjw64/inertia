set -eo pipefail

rm -f db/boards.db*
export DATABASE_URL="sqlite:db/boards.db"
sqlx db create
sqlx migrate run
