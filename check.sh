#!/usr/bin/env bash
set -euo pipefail
cd "$(dirname "$0")"
if [[ $# -eq 0 ]]; then
  /bin/sh .chrono-harness/ci/install.sh
  export PATH="$PWD/.chrono-harness/bin:$PATH"
  export CHRONO_CHECK_SOURCE=${CHRONO_CHECK_SOURCE:-local}
  exec .chrono-harness/bin/chrono-harness check
fi
if [[ $# == 1 && $1 == --native ]]; then
  exec chrono-host-bootstrap native-scratch run /bin/bash ./check.sh --native-in-directory
fi
if [[ $# -ne 1 || $1 != --native-in-directory ]]; then
  echo 'usage: ./check.sh [--native]' >&2
  exit 2
fi

scratch=${CHRONO_NATIVE_SCRATCH:?missing native check directory}
export TMPDIR="$scratch"
export CARGO_TARGET_DIR="${CARGO_TARGET_DIR:-$scratch/cargo}"
cargo fmt --check --manifest-path crates/test-git/Cargo.toml
product=crates/test-git/Cargo.toml
cargo build --locked --message-format=json-render-diagnostics --manifest-path "$product" > "$scratch/build.jsonl"
cargo fmt --check --manifest-path crates/test-git-tests/Cargo.toml
export CHRONO_TEST_GIT_BIN="$CARGO_TARGET_DIR/debug/chrono-test-git"
export CHRONO_TEST_GIT_PROFILE_BIN="$CARGO_TARGET_DIR/debug/chrono-test-git-profile"
cargo test --locked --manifest-path crates/test-git-tests/Cargo.toml --color never -- --format pretty | tee "$scratch/tests.log"
for consumer in \
  known_git_command_independent_boundaries \
  dispatch_rejects_unknown_commands_before_opening_a_repository \
  profile_terminal_failures_preserve_process_responses_and_delegation \
  environment_errors_precede_trace_creation \
  registered_environments_preserve_raw_inputs_and_first_error \
  registration_tree_failure_preserves_bytes_exit_and_arguments \
  git_status_io::selected_git_mock_status_skips_ignored_subtrees_but_checks_tracked_files \
  git_status_io::selected_git_mock_status_keeps_visible_and_explicit_ignored_io_errors; do
  grep -Fxq "test $consumer ... ok" "$scratch/tests.log" || {
    echo "E_VERUS_COVERAGE: required test-git consumer did not pass: $consumer" >&2
    exit 1
  }
done

coverage_failure() {
  echo "E_VERUS_COVERAGE: $*" >&2
  exit 1
}

# Read the exact library selected by Cargo in the build of the tested executable.
metadata=$(jq -ers --arg manifest "$(pwd -P)/$product" '
  [.[] | select(.reason == "compiler-artifact" and .manifest_path == $manifest
                 and .target.kind == ["lib"]) | .filenames[] | select(endswith(".rmeta"))]
  | if length == 1 then .[0] else error("expected the product library") end
' "$scratch/build.jsonl") || coverage_failure "native library was not identified"
filename=${metadata##*/}
stem=${filename#lib}
stem=${metadata%/*}/${stem%.rmeta}
chrono-host-bootstrap check-compiler-sources native "$stem" src/lib.rs src/profile.rs src/profile_failure.rs ||
  coverage_failure "native library differs from required sources"
