# Inertia

## Run Locally
Running locally is as easy as runnning `docker compose up` inside the root directory of the project. The game will then be available in the browser at `localhost:8080`. The development docker compose file will mount the source code directories and will reload the client and server upon changes made in the source. One exception to the reloading is WASM files. To use changes to WASM files, you must `docker compose build` to rebuild the container WASM files, which will re-run `wasm-pack build` inside the container.
