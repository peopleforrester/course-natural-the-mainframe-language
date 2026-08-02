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

step 'Prose style (learner-facing and authored text)'
# Covers markdown AND the lesson content, which is the prose a learner actually reads.
# Two kinds of file are excluded because their punctuation is not ours to change:
# vendored third-party code, and the verbatim copies under reference/ that were taken
# from other repos and must stay faithful to their originals.
prose_files() {
    git ls-files -z '*.md' '*.js' '*.html' '*.css' \
        ':!:web/vendor/*' ':!:web/pkg/*' ':!:web/fonts/*' ':!:reference/*'
}

prose_hits=$(prose_files | xargs -0 grep -ln '—\|–' 2>/dev/null || true)
if [[ -n "${prose_hits}" ]]; then
    printf '      FAILED: em-dash or en-dash found in:\n'
    printf '        %s\n' ${prose_hits}
    failed+=('prose')
else
    # Curly quotes and ellipsis characters are the other common paste artifacts, and they
    # break the fixed-width terminal font when they land in lesson text.
    smart_hits=$(prose_files | xargs -0 grep -ln '[‘’“”…]' 2>/dev/null || true)
    if [[ -n "${smart_hits}" ]]; then
        printf '      FAILED: smart quotes or ellipsis found in:\n'
        printf '        %s\n' ${smart_hits}
        failed+=('prose')
    else
        printf '      ok\n'
    fi
fi

printf '\n────────────────────────────────────\n'
if [ ${#failed[@]} -eq 0 ]; then
    printf 'PASS: all %d stages green.\n' "$STAGES"
    exit 0
fi
printf 'FAIL: %s\n' "${failed[*]}"
exit 1
