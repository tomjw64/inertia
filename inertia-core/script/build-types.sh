#!/usr/bin/env bash
set -e

ORIGINAL_DIR=$(pwd)
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
INERTIA_CORE_DIR=$(dirname $SCRIPT_DIR)

cd $INERTIA_CORE_DIR
rm -rf ./pkg
mkdir ./pkg
cat > ./pkg/package.json <<- PACKAGE
{
  "name": "inertia-core",
  "collaborators": [
    "tomjw64 <tom.jw64@gmail.com>"
  ],
  "version": "0.1.0",
  "files": [
    "inertia_core.ts"
  ],
  "module": "inertia_core.ts",
  "types": "inertia_core.ts"
}
PACKAGE
typeshare . --lang=typescript --output-file=./pkg/inertia_core.ts
cd $ORIGINAL_DIR
