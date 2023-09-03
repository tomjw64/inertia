#!/usr/bin/env bash
set -e

ORIGINAL_DIR=$(pwd)
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
INERTIA_CORE_DIR=$(dirname $SCRIPT_DIR)

cd $INERTIA_CORE_DIR
typeshare . --lang=typescript --output-file=./pkg/inertia_core.ts
cd $ORIGINAL_DIR
