set -eo pipefail

mkdir -p db
rm -f db/positions.db*
export DATABASE_URL="sqlite:db/positions.db"
sqlx db create
sqlx migrate run
time cargo run --release
sqlite3 db/positions.db vacuum
