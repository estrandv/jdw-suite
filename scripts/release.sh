#!/usr/bin/env bash
# release.sh — create a new jdw-suite release.
#
# Prompts for version (suggests next patch bump), then:
#   1. Verifies tests pass
#   2. Builds release binary
#   3. Tags, pushes
#   4. Creates GitHub Release with binary + assets
#
# Usage: ./scripts/release.sh

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "$0")" && pwd)"
SUITE_DIR="$(dirname "$SCRIPT_DIR")"
BILLBOARD_DIR="$HOME/programming/jdw-billboarding-backend"

cd "$SUITE_DIR"

# ── Current version ──────────────────────────────────────────────
current=$(git describe --tags --abbrev=0 2>/dev/null || echo "v0.0.0")
current_num="${current#v}"

echo "Current version: $current"

# Suggest next patch bump
IFS=. read -r major minor patch <<< "$current_num"
suggested="$major.$minor.$((patch + 1))"

# ── Prompt ────────────────────────────────────────────────────────
read -p "New version [$suggested]: " version
version="${version:-$suggested}"
tag="v$version"

echo ""
echo "Release: $current  →  $tag"
echo ""

# ── Verify ────────────────────────────────────────────────────────
echo "=== Running tests ==="
(cd "$BILLBOARD_DIR" && cargo test) || { echo "Tests failed!"; exit 1; }

echo ""
echo "=== Building release ==="
cargo build --release || { echo "Build failed!"; exit 1; }

# ── Confirm ───────────────────────────────────────────────────────
echo ""
echo "Ready to release $tag. Assets:"
echo "  - target/release/jdw"
echo "  - assets/hello.bbd"
echo "  - assets/synthdefs.scd"
echo "  - assets/example.jdw.toml"
echo "  - assets/install.sh"
read -p "Proceed? [Y/n] " confirm
if [[ ! "$confirm" =~ ^[Yy]?$ ]]; then
    echo "Aborted."
    exit 1
fi

# ── Tag + push ────────────────────────────────────────────────────
echo ""
echo "=== Tagging $tag ==="
git tag -d "$tag" 2>/dev/null || true
git tag "$tag"
git push origin "$tag"

# ── Create release ────────────────────────────────────────────────
echo ""
echo "=== Creating GitHub Release ==="
gh release create "$tag" \
    --title "jdw $tag" \
    --notes "Release $tag." \
    target/release/jdw \
    assets/hello.bbd \
    assets/synthdefs.scd \
    assets/example.jdw.toml \
    assets/install.sh

echo ""
echo "Release: https://github.com/estrandv/jdw-suite/releases/tag/$tag"
echo "CI:      https://github.com/estrandv/jdw-suite/actions"
echo ""
echo "CI will attach platform .zip artifacts automatically."
