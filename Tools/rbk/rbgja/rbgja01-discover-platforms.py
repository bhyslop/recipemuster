#!/usr/bin/env python3
# RBGJAM Step 01: Discover platforms from -image manifest in GAR
# Builder: gcr.io/cloud-builders/gcloud
# Substitutions: _RBGA_GAR_HOST, _RBGA_GAR_PATH, _RBGA_VESSEL,
#                _RBGA_CONSECRATION, _RBGA_VESSEL_MODE,
#                _RBGA_ARK_SUFFIX_IMAGE, _RBGA_ARK_SUFFIX_DIAGS
#
# Queries the -image manifest to discover what platforms are present.
# Handles OCI image index (multi-platform) vs single OCI/Docker manifest.
# Writes platform info to /workspace for subsequent steps:
#   platforms.txt          - comma-separated platform list (linux/amd64,linux/arm64)
#   platform_suffixes.txt  - comma-separated suffix list (-amd64,-arm64)
#   platform_count.txt     - number of platforms
#   platform_digests.txt   - suffix-to-digest mapping (one line per platform: -amd64 sha256:...)

import json
import os
import subprocess
import sys
import tarfile
import urllib.error
import urllib.request
from io import BytesIO

ACCEPT_ALL = ",".join([
    "application/vnd.oci.image.index.v1+json",
    "application/vnd.docker.distribution.manifest.list.v2+json",
    "application/vnd.oci.image.manifest.v1+json",
    "application/vnd.docker.distribution.manifest.v2+json",
])

DIAGS_ACCEPT = ",".join([
    "application/vnd.oci.image.manifest.v1+json",
    "application/vnd.docker.distribution.manifest.v2+json",
])


def die(msg):
    print(msg, file=sys.stderr)
    sys.exit(1)


def require_env(name):
    val = os.environ.get(name, "")
    if not val:
        die(f"{name} missing")
    return val


def gar_fetch(url, token, accept, method="GET"):
    headers = {"Authorization": f"Bearer {token}", "Accept": accept}
    req = urllib.request.Request(url, headers=headers, method=method)
    return urllib.request.urlopen(req)


def gar_json(url, token, accept):
    resp = gar_fetch(url, token, accept)
    return json.loads(resp.read())


def main():
    gar_host       = require_env("_RBGA_GAR_HOST")
    gar_path       = require_env("_RBGA_GAR_PATH")
    vessel         = require_env("_RBGA_VESSEL")
    consecration   = require_env("_RBGA_CONSECRATION")
    _              = require_env("_RBGA_VESSEL_MODE")
    ark_suffix_img = require_env("_RBGA_ARK_SUFFIX_IMAGE")
    ark_suffix_dia = require_env("_RBGA_ARK_SUFFIX_DIAGS")

    image_tag = f"{consecration}{ark_suffix_img}"
    registry_base = f"https://{gar_host}/v2/{gar_path}/{vessel}"

    print("Fetching OAuth2 token via gcloud")
    result = subprocess.run(
        ["gcloud", "auth", "print-access-token"],
        capture_output=True, text=True, check=True,
    )
    token = result.stdout.strip()

    print(f"=== Discovering platforms for {vessel}:{image_tag} ===")

    try:
        manifest = gar_json(f"{registry_base}/manifests/{image_tag}", token, ACCEPT_ALL)
    except Exception as e:
        die(f"Failed to fetch manifest for {image_tag}: {e}")

    media_type = manifest.get("mediaType", "")
    print(f"Manifest media type: {media_type}")

    is_index = "manifest.list" in media_type or "image.index" in media_type

    if is_index:
        _discover_index(manifest, registry_base, token)
    else:
        _discover_single(manifest, registry_base, token, image_tag)

    with open("platforms.txt") as f:
        print(f"Platforms: {f.read()}")
    with open("platform_suffixes.txt") as f:
        print(f"Suffixes: {f.read()}")
    with open("platform_count.txt") as f:
        print(f"Count: {f.read()}")
    print("=== Platform discovery complete ===")

    _extract_diags(registry_base, token, consecration, ark_suffix_dia)


def _discover_index(manifest, registry_base, token):
    all_entries = manifest.get("manifests", [])
    print(f"Multi-platform manifest detected ({len(all_entries)} entries in index)")

    # Filter out attestation manifests (platform unknown/unknown)
    # BuildKit stores SLSA provenance and SBOM attestations as manifest entries
    # with platform unknown/unknown to prevent runtimes from pulling them as images.
    entries = []
    attestation_count = 0
    for m in all_entries:
        plat = m.get("platform")
        if not plat:
            continue
        os_name = plat.get("os", "")
        arch = plat.get("architecture", "")
        if os_name == "unknown" and arch == "unknown":
            attestation_count += 1
            continue
        variant = plat.get("variant", "")
        platform_str = f"{os_name}/{arch}"
        suffix = f"-{arch}"
        if variant:
            platform_str += f"/{variant}"
            suffix += variant
        entries.append((platform_str, m["digest"], suffix))

    if attestation_count > 0:
        print(f"Filtered {attestation_count} attestation manifest(s) (platform unknown/unknown)")

    if not entries:
        die("No runnable platforms found after filtering attestation manifests")

    with open("platforms.txt", "w") as f:
        f.write(",".join(e[0] for e in entries))
    with open("platform_suffixes.txt", "w") as f:
        f.write(",".join(e[2] for e in entries))
    with open("platform_count.txt", "w") as f:
        f.write(str(len(entries)))
    with open("platform_digests.txt", "w") as f:
        for _, digest, suffix in entries:
            f.write(f"{suffix} {digest}\n")


def _discover_single(manifest, registry_base, token, image_tag):
    print("Single manifest detected")

    config_digest = manifest.get("config", {}).get("digest", "")
    if not config_digest:
        die("No config digest in manifest")

    try:
        config = gar_json(f"{registry_base}/blobs/{config_digest}", token, ACCEPT_ALL)
    except Exception as e:
        die(f"Failed to fetch config blob: {e}")

    os_name = config.get("os", "")
    arch    = config.get("architecture", "")
    variant = config.get("variant", "")

    if variant:
        platform = f"{os_name}/{arch}/{variant}"
        suffix = f"-{arch}{variant}"
    else:
        platform = f"{os_name}/{arch}"
        suffix = f"-{arch}"

    # Get manifest digest from HEAD response
    try:
        resp = gar_fetch(f"{registry_base}/manifests/{image_tag}", token, ACCEPT_ALL, method="HEAD")
        manifest_digest = resp.headers.get("Docker-Content-Digest", "")
    except Exception as e:
        die(f"Failed HEAD for manifest digest: {e}")
    if not manifest_digest:
        die("No Docker-Content-Digest in HEAD response")

    with open("platforms.txt", "w") as f:
        f.write(platform)
    with open("platform_suffixes.txt", "w") as f:
        f.write(suffix)
    with open("platform_count.txt", "w") as f:
        f.write("1")
    with open("platform_digests.txt", "w") as f:
        f.write(f"{suffix} {manifest_digest}\n")


def _extract_diags(registry_base, token, consecration, ark_suffix_diags):
    diags_tag = f"{consecration}{ark_suffix_diags}"
    print()
    print(f"=== Checking for -diags artifact: {diags_tag} ===")

    try:
        diags_manifest = gar_json(
            f"{registry_base}/manifests/{diags_tag}", token, DIAGS_ACCEPT,
        )
    except urllib.error.HTTPError as e:
        print(f"No -diags artifact found (HTTP {e.code}) — expected for bind/graft")
        print("=== -diags check complete ===")
        return
    except Exception:
        print("No -diags artifact found (HTTP 000) — expected for bind/graft")
        print("=== -diags check complete ===")
        return

    print("Found -diags artifact, extracting...")

    for layer in diags_manifest.get("layers", []):
        layer_digest = layer["digest"]
        print(f"Extracting -diags layer: {layer_digest}")
        try:
            resp = gar_fetch(f"{registry_base}/blobs/{layer_digest}", token, DIAGS_ACCEPT)
            data = resp.read()
            with tarfile.open(fileobj=BytesIO(data), mode="r:gz") as tar:
                # filter="data" available in 3.12+; gcloud image is 3.10
                if sys.version_info >= (3, 12):
                    tar.extractall(filter="data")
                else:
                    tar.extractall()
        except Exception:
            print(f"WARN: Failed to extract -diags layer {layer_digest}", file=sys.stderr)

    for dfile in ["buildkit_metadata.json", "cache_before.json", "cache_after.json", "recipe.txt"]:
        if os.path.isfile(dfile):
            print(f"Extracted from -diags: {dfile} ({os.path.getsize(dfile)} bytes)")

    print("=== -diags check complete ===")


if __name__ == "__main__":
    main()
