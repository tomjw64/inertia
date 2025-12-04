#!/usr/bin/env bash
set -euo pipefail

usage() {
  cat <<'USAGE'
Usage: benchmark_branches.sh --branch-a <name> --branch-b <name> [--runs <N>] [-- <solver-profile args>]

  -a, --branch-a   Git branch for the first benchmark (required)
  -b, --branch-b   Git branch for the second benchmark (required)
  -r, --runs       Number of hyperfine runs per branch (default: 10)
  -h, --help       Show this message

Any arguments after "--" are forwarded to the solver-profile binary.

The script checks out each branch, runs `cargo build --release` in the solver-profile crate,
captures the resulting binaries, and finally runs hyperfine to compare them.
USAGE
}

sanitize_branch() {
  echo "$1" | sed 's#[^A-Za-z0-9._-]#_#g'
}

join_command() {
  local -n _arr=$1
  local cmd=""
  for arg in "${_arr[@]}"; do
    cmd+="$(printf '%q ' "$arg")"
  done
  echo "${cmd% }"
}

ensure_clean_worktree() {
  if [[ -n "$(git status --porcelain --untracked-files=no)" ]]; then
    echo "Error: working tree is not clean. Please commit, stash, or discard changes before running this script." >&2
    exit 1
  fi
}

build_branch_binary() {
  local branch=$1 dest=$2
  echo "Checking out ${branch}"
  git checkout --quiet "$branch"
  echo "Building solver-profile in release mode (branch: ${branch})"
  pushd "$SOLVER_PROFILE_DIR" >/dev/null
  cargo build --release --bin solver-profile >/dev/null
  cp target/release/solver-profile "$dest"
  popd >/dev/null
}

BRANCH_A=""
BRANCH_B=""
RUNS=10
PASSTHROUGH_ARGS=()

while [[ $# -gt 0 ]]; do
  case "$1" in
    -a|--branch-a)
      BRANCH_A=${2:-}
      shift 2
      ;;
    -b|--branch-b)
      BRANCH_B=${2:-}
      shift 2
      ;;
    -r|--runs)
      RUNS=${2:-}
      shift 2
      ;;
    -h|--help)
      usage
      exit 0
      ;;
    --)
      shift
      PASSTHROUGH_ARGS=("$@")
      break
      ;;
    *)
      echo "Unknown argument: $1" >&2
      usage
      exit 1
      ;;
  esac
 done

if [[ -z "$BRANCH_A" || -z "$BRANCH_B" ]]; then
  echo "Error: both --branch-a and --branch-b must be provided." >&2
  usage
  exit 1
fi

if ! [[ "$RUNS" =~ ^[0-9]+$ && "$RUNS" -gt 0 ]]; then
  echo "Error: --runs must be a positive integer." >&2
  exit 1
fi

SCRIPT_DIR=$( cd -- "$( dirname -- "${BASH_SOURCE[0]}" )" &> /dev/null && pwd )
SOLVER_PROFILE_DIR=$(dirname "$SCRIPT_DIR")
REPO_ROOT=$(cd "$SOLVER_PROFILE_DIR/.." && pwd)

pushd "$REPO_ROOT" >/dev/null
ensure_clean_worktree
ORIGINAL_REF=$(git rev-parse --abbrev-ref HEAD 2>/dev/null || git rev-parse HEAD)

cleanup() {
  git checkout --quiet "$ORIGINAL_REF" >/dev/null 2>&1 || true
  popd >/dev/null 2>&1 || true
}
trap cleanup EXIT

TMP_BIN_DIR="$SOLVER_PROFILE_DIR/target/branch-benchmark"
mkdir -p "$TMP_BIN_DIR"

BIN_A="$TMP_BIN_DIR/solver-profile_$(sanitize_branch "$BRANCH_A")"
BIN_B="$TMP_BIN_DIR/solver-profile_$(sanitize_branch "$BRANCH_B")"

build_branch_binary "$BRANCH_A" "$BIN_A"
build_branch_binary "$BRANCH_B" "$BIN_B"

git checkout --quiet "$ORIGINAL_REF"

CMD_A=("$BIN_A" "${PASSTHROUGH_ARGS[@]}")
CMD_B=("$BIN_B" "${PASSTHROUGH_ARGS[@]}")

CMD_A_STR=$(join_command CMD_A)
CMD_B_STR=$(join_command CMD_B)

echo "Running hyperfine with ${RUNS} run(s) per branch"
hyperfine --runs "$RUNS" -n "$BRANCH_A" "$CMD_A_STR" -n "$BRANCH_B" "$CMD_B_STR"
