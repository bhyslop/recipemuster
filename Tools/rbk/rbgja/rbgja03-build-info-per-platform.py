#!/usr/bin/env python3
# RBGJAM Step 03: Generate per-platform mode-aware build_info.json
# Builder: gcr.io/cloud-builders/gcloud (switched from alpine for Python availability)
# Substitutions (GCB anchors — automapSubstitutions provides as env vars):
#   ${_RBGA_GAR_HOST} ${_RBGA_GAR_PATH} ${_RBGA_VESSEL}
#   ${_RBGA_CONSECRATION} ${_RBGA_VESSEL_MODE}
#   ${_RBGA_GIT_COMMIT} ${_RBGA_GIT_BRANCH} ${_RBGA_GIT_REPO}
#   ${_RBGA_BUILD_ID} ${_RBGA_INSCRIBE_TIMESTAMP}
#   ${_RBGA_BIND_SOURCE} ${_RBGA_GRAFT_SOURCE}
#   ${_RBGA_DOCKERFILE_CONTENT}
#
# Generates one build_info-{arch}{variant}.json per platform with mode-aware fields.
# Shared fields: vessel_mode, vessel_name, platform, image_digest, about_timestamp,
#                git_commit, git_branch, git_repo
# Conjure fields: build_id, inscribe_timestamp, qemu_used, slsa_*
# Bind fields: bind_source
# Graft fields: graft_source

import json
import os
import sys
from datetime import datetime, timezone


def die(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def require_env(name):
    val = os.environ.get(name, "")
    if not val:
        die(f"{name} missing")
    return val


def get_consecration():
    """Get consecration from env var (standalone about) or file (combined conjure)."""
    val = os.environ.get("_RBGA_CONSECRATION", "")
    if val:
        return val
    try:
        with open(".consecration") as f:
            return f.read().strip()
    except FileNotFoundError:
        die("_RBGA_CONSECRATION not set and .consecration file not found")


def get_build_id():
    """Get build ID from env var (standalone about) or GCB built-in (combined conjure)."""
    return os.environ.get("_RBGA_BUILD_ID", "") or os.environ.get("BUILD_ID", "")


def main():
    _            = require_env("_RBGA_GAR_HOST")
    _            = require_env("_RBGA_GAR_PATH")
    vessel       = require_env("_RBGA_VESSEL")
    consecration = get_consecration()
    vessel_mode  = require_env("_RBGA_VESSEL_MODE")

    for fname in ["platforms.txt", "platform_suffixes.txt", "platform_digests.txt"]:
        if not os.path.isfile(fname) or os.path.getsize(fname) == 0:
            die(f"{fname} not found (step 01)")

    # Write recipe.txt — prefer -diags extraction (step 01), fall back to substitution
    if os.path.isfile("recipe.txt"):
        print(f"recipe.txt present from -diags extraction ({os.path.getsize('recipe.txt')} bytes) — skipping substitution variable")
    elif os.environ.get("_RBGA_DOCKERFILE_CONTENT", ""):
        with open("recipe.txt", "w") as f:
            f.write(os.environ["_RBGA_DOCKERFILE_CONTENT"])
        print(f"recipe.txt written from substitution variable ({os.path.getsize('recipe.txt')} bytes)")
    else:
        print("No Dockerfile content provided — recipe.txt omitted")

    ts = datetime.now(timezone.utc).strftime("%Y-%m-%dT%H:%M:%SZ")
    host_platform = "linux/amd64"

    # Load per-platform digests
    digests = {}
    with open("platform_digests.txt") as f:
        for line in f:
            line = line.strip()
            if not line:
                continue
            suffix, digest = line.split(" ", 1)
            label = "".join(c for c in suffix.lstrip("-") if c.isalnum())
            digests[label] = digest

    with open("platforms.txt") as f:
        platforms_csv = f.read().strip()
    with open("platform_suffixes.txt") as f:
        suffixes_csv = f.read().strip()

    platforms = platforms_csv.split(",")
    suffixes = suffixes_csv.split(",")

    print("=== Generating per-platform build_info ===")

    for plat, suffix in zip(platforms, suffixes):
        label = "".join(c for c in suffix.lstrip("-") if c.isalnum())
        info_file = f"build_info-{label}.json"
        image_digest = digests.get(label, "")

        print(f"--- {plat} \u2192 {info_file} ---")

        info = {
            "consecration": consecration,
            "vessel_mode": vessel_mode,
            "vessel_name": vessel,
            "platform": plat,
            "image_digest": image_digest,
            "about_timestamp": ts,
            "git": {
                "repo": os.environ.get("_RBGA_GIT_REPO", ""),
                "branch": os.environ.get("_RBGA_GIT_BRANCH", ""),
                "commit": os.environ.get("_RBGA_GIT_COMMIT", ""),
            },
        }

        if vessel_mode == "conjure":
            build_id = get_build_id()
            info["build"] = {
                "build_id": build_id,
                "inscribe_timestamp": os.environ.get("_RBGA_INSCRIBE_TIMESTAMP", ""),
                "qemu_used": plat != host_platform,
            }
            info["slsa"] = {
                "build_level": 3,
                "build_invocation_id": build_id,
                "provenance_predicate_types": [
                    "https://slsa.dev/provenance/v0.1",
                    "https://slsa.dev/provenance/v1",
                ],
                "provenance_builder_id": "https://cloudbuild.googleapis.com/GoogleHostedWorker",
            }
        elif vessel_mode == "bind":
            info["bind"] = {"source": os.environ.get("_RBGA_BIND_SOURCE", "")}
        elif vessel_mode == "graft":
            info["graft"] = {"source": os.environ.get("_RBGA_GRAFT_SOURCE", "")}
        else:
            die(f"Unknown vessel mode: {vessel_mode}")

        with open(info_file, "w") as f:
            json.dump(info, f, indent=2)
            f.write("\n")

        if os.path.getsize(info_file) == 0:
            die(f"build_info output empty for {plat}")
        print(f"Generated: {info_file}")

    print("=== build_info generation complete ===")


if __name__ == "__main__":
    main()
