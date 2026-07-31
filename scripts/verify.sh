#!/usr/bin/env bash
# ABOUTME: The local verification gate for this repo. Stands in for hosted CI, which is
# ABOUTME: deliberately not used here. Runs format, lint, tests, and the wasm target build.
set -uo pipefail

cd "$(git rev-parse --show-toplevel)" || exit 2

STAGES=5
stage=0
failed=()

step() {
    stage=$((stage + 1))
    printf '\n[%d/%d] %s\n' "$stage" "$STAGES" "$1"
}

run() {
    local label="$1"
    shift
    local start elapsed
    start=$SECONDS
    if "$@" >/tmp/verify-out.$$ 2>&1; then
        elapsed=$((SECONDS - start))
        printf '      ok (%ds)\n' "$elapsed"
        rm -f /tmp/verify-out.$$
        return 0
    fi
    elapsed=$((SECONDS - start))
    printf '      FAILED (%ds)\n\n' "$elapsed"
    tail -30 /tmp/verify-out.$$
    rm -f /tmp/verify-out.$$
    failed+=("$label")
    return 1
}

printf 'Local verification gate (no hosted CI by design)\n'
printf 'Repo: %s\n' "$(basename "$PWD")"

step 'Formatting (cargo fmt --check)'
run 'fmt' cargo fmt --all --check

step 'Lint (cargo clippy, warnings denied)'
run 'clippy' cargo clippy --workspace --all-targets -- -D warnings

step 'Tests (cargo test --workspace)'
run 'test' cargo test --workspace

step 'Wasm target build (wasm32-unknown-unknown)'
run 'wasm' cargo build -p natural-core --target wasm32-unknown-unknown

step 'Prose style (no em-dashes in tracked markdown)'
if git ls-files '*.md' -z | xargs -0 grep -l '—' 2>/dev/null; then
    printf '      FAILED: em-dashes found in the files above\n'
    failed+=('prose')
else
    printf '      ok\n'
fi

printf '\n────────────────────────────────────\n'
if [ ${#failed[@]} -eq 0 ]; then
    printf 'PASS: all %d stages green.\n' "$STAGES"
    exit 0
fi
printf 'FAIL: %s\n' "${failed[*]}"
exit 1
