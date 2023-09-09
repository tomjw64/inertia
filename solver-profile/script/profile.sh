#!/usr/bin/env bash
set -e

ORIGINAL_DIR=$(pwd)
SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
PROFILE_DIR=$(dirname $SCRIPT_DIR)

cd $PROFILE_DIR
cargo build --release
if [[ $(cat /proc/sys/kernel/kptr_restrict) -ne 0 ]]; then
  echo 0 | sudo tee /proc/sys/kernel/kptr_restrict
fi
if [[ $(cat /proc/sys/kernel/perf_event_paranoid) -ne -1 ]]; then
  echo -1 | sudo tee /proc/sys/kernel/perf_event_paranoid
fi
time CARGO_PROFILE_RELEASE_DEBUG=true cargo flamegraph
cd $ORIGINAL_DIR
