#!/bin/bash
# voa_arcanum.sh - VOK Arcanum: Two-repo install logic
#
# This script runs FROM the source repo and installs INTO a target repo.
#
# Usage:
#   ./Tools/vok/voa_arcanum.sh voa-i <target-repo-path>
#   ./Tools/vok/voa_arcanum.sh voa-u <target-repo-path>
#   ./Tools/vok/voa_arcanum.sh voa-c <target-repo-path>
#
# Operations:
#   voa-i  Install VVK + config into target repo
#   voa-u  Uninstall VVK from target repo
#   voa-c  Check VVK installation in target repo

set -euo pipefail

SCRIPT_DIR="$(cd "$(dirname "${BASH_SOURCE[0]}")" && pwd)"
ZVOA_INCLUDED=1

# -----------------------------------------------------------------------------
# Main dispatch
# -----------------------------------------------------------------------------

voa_main() {
    local cmd="${1:-}"
    local target="${2:-}"

    case "$cmd" in
        voa-i) voa_install "$target" ;;
        voa-u) voa_uninstall "$target" ;;
        voa-c) voa_check "$target" ;;
        *)
            echo "Usage: $0 {voa-i|voa-u|voa-c} <target-repo-path>" >&2
            exit 1
            ;;
    esac
}

# -----------------------------------------------------------------------------
# Install
# -----------------------------------------------------------------------------

voa_install() {
    local target="$1"

    if [[ -z "$target" ]]; then
        echo "Error: target repo path required" >&2
        exit 1
    fi

    # Verify release binaries exist
    if ! ls "$SCRIPT_DIR/release"/*/vvr >/dev/null 2>&1; then
        echo "Error: No release binaries found in $SCRIPT_DIR/release/" >&2
        echo "Run release preparation first." >&2
        exit 1
    fi

    echo "Installing VVK into $target..."

    # 1. Delete existing VVK if present
    if [[ -d "$target/Tools/vvk" ]]; then
        echo "  Removing existing Tools/vvk/"
        rm -rf "$target/Tools/vvk"
    fi

    # 2. Copy VVK template (wrapper + utilities)
    echo "  Copying Tools/vvk/"
    mkdir -p "$target/Tools"
    cp -r "$SCRIPT_DIR/../vvk" "$target/Tools/"

    # 3. Copy release binaries
    echo "  Copying release binaries"
    for platform_dir in "$SCRIPT_DIR/release"/*/; do
        platform=$(basename "$platform_dir")
        if [[ -f "$platform_dir/vvr" ]]; then
            cp "$platform_dir/vvr" "$target/Tools/vvk/bin/vvr-$platform"
        fi
    done

    # 4. Emit slash commands (TODO: implement emitters)
    echo "  Emitting slash commands (not yet implemented)"
    mkdir -p "$target/.claude/commands"

    # 5. Patch CLAUDE.md (TODO: implement)
    echo "  Patching CLAUDE.md (not yet implemented)"

    # 6. Record in manifest
    echo "  Recording in kit_manifest.json"
    mkdir -p "$target/.claude"
    # TODO: proper manifest management

    echo "Done."
}

# -----------------------------------------------------------------------------
# Uninstall
# -----------------------------------------------------------------------------

voa_uninstall() {
    local target="$1"

    if [[ -z "$target" ]]; then
        echo "Error: target repo path required" >&2
        exit 1
    fi

    echo "Uninstalling VVK from $target..."

    if [[ -d "$target/Tools/vvk" ]]; then
        rm -rf "$target/Tools/vvk"
        echo "  Removed Tools/vvk/"
    else
        echo "  Tools/vvk/ not found"
    fi

    # TODO: remove slash commands, unpatch CLAUDE.md, update manifest

    echo "Done."
}

# -----------------------------------------------------------------------------
# Check
# -----------------------------------------------------------------------------

voa_check() {
    local target="$1"

    if [[ -z "$target" ]]; then
        echo "Error: target repo path required" >&2
        exit 1
    fi

    echo "Checking VVK installation in $target..."

    local status=0

    if [[ -d "$target/Tools/vvk" ]]; then
        echo "  Tools/vvk/: present"
    else
        echo "  Tools/vvk/: MISSING"
        status=1
    fi

    if [[ -x "$target/Tools/vvk/bin/vvx" ]]; then
        echo "  bin/vvx: present"
    else
        echo "  bin/vvx: MISSING"
        status=1
    fi

    # Check for at least one binary
    if ls "$target/Tools/vvk/bin"/vvr-* >/dev/null 2>&1; then
        echo "  bin/vvr-*: present"
    else
        echo "  bin/vvr-*: MISSING"
        status=1
    fi

    if [[ $status -eq 0 ]]; then
        echo "Installation OK."
    else
        echo "Installation INCOMPLETE."
    fi

    return $status
}

# -----------------------------------------------------------------------------
# Run if executed directly
# -----------------------------------------------------------------------------

if [[ "${BASH_SOURCE[0]}" == "${0}" ]]; then
    voa_main "$@"
fi
