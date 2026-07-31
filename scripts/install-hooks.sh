#!/usr/bin/env bash
# ABOUTME: Installs the repo-local pre-push hook that runs scripts/verify.sh, so nothing
# ABOUTME: reaches the remote without passing the local gate that stands in for hosted CI.
set -euo pipefail

root="$(git rev-parse --show-toplevel)"
hook="$root/.git/hooks/pre-push"

cat > "$hook" <<'HOOK'
#!/usr/bin/env bash
# Local gate. This repo runs no hosted CI, so the pre-push hook is the enforcement point.
# Bypass deliberately with --no-verify when you know why.
set -euo pipefail
exec "$(git rev-parse --show-toplevel)/scripts/verify.sh"
HOOK

chmod +x "$hook"
echo "Installed pre-push hook at .git/hooks/pre-push"
