#!/usr/bin/env bash
set -e

ORIGINAL_DIR=$(pwd)
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
INERTIA_CORE_DIR=$(dirname $SCRIPT_DIR)

cd $INERTIA_CORE_DIR
wasm-pack build --features web
cd $ORIGINAL_DIR
