#!/bin/bash
#
# Copyright 2025 Scale Invariant, Inc.
#
# Licensed under the Apache License, Version 2.0 (the "License");
# you may not use this file except in compliance with the License.
# You may obtain a copy of the License at
#
#     http://www.apache.org/licenses/LICENSE-2.0
#
# Unless required by applicable law or agreed to in writing, software
# distributed under the License is distributed on an "AS IS" BASIS,
# WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
# See the License for the specific language governing permissions and
# limitations under the License.
#
# Author: Brad Hyslop <bhyslop@scaleinvariant.org>
#
# Recipe Bottle GCP Constants - Implementation (no printf required)

set -euo pipefail

# Multiple inclusion detection
# (Module state remains ZRBGC_* per BCG; external constants use RBGC_*)
test -z "${ZRBGC_SOURCED:-}" || buc_die "Module rbgc multiply sourced - check sourcing hierarchy"
ZRBGC_SOURCED=1

# Tinder constants (pure string literals, no variable expansion — available at source time)
# Depot project ID infix between RBRD_CLOUD_PREFIX and RBRD_DEPOT_MONIKER, consumed
# by rbdc_DerivedConstants.sh's RBDC_DEPOT_PROJECT_ID derivation.
RBGC_depot_project_infix="d-"

######################################################################
# Internal Functions (zrbgc_*)

zrbgc_kindle() {
  test -z "${ZRBGC_KINDLED:-}" || buc_die "Module rbgc already kindled"

  # Global Resource Naming (Google Cloud global namespace)
  # These resources compete in globally-unique namespaces across all of GCP
  # Pattern: {prefix}-{type}-{name}-{timestamp} where timestamp is YYMMDDHHMMSS
  readonly RBGC_GLOBAL_PREFIX="rbwg"
  readonly RBGC_GLOBAL_TYPE_PAYOR="p"
  readonly RBGC_GLOBAL_TYPE_DEPOT="d"
  readonly RBGC_GLOBAL_TYPE_BUCKET="b"
  readonly RBGC_GLOBAL_TIMESTAMP_FORMAT="+%y%m%d%H%M%S"
  readonly RBGC_GLOBAL_TIMESTAMP_LEN=12
  readonly RBGC_GLOBAL_TIMESTAMP_REGEX="[0-9]{${RBGC_GLOBAL_TIMESTAMP_LEN}}"

  # Global resource validation patterns
  # Payor:  rbwg-p-YYMMDDHHMMSS  (timestamp survives — payor is installation-scoped, not depot-scoped)
  readonly RBGC_GLOBAL_PAYOR_REGEX="^${RBGC_GLOBAL_PREFIX}-${RBGC_GLOBAL_TYPE_PAYOR}-${RBGC_GLOBAL_TIMESTAMP_REGEX}$"

  # Basic Configuration
  readonly RBGC_ADMIN_ROLE="rbw-admin"
  readonly RBGC_PAYOR_ROLE="rbw-payor"
  readonly RBGC_PAYOR_APP_NAME="Recipe Bottle Payor"

  # Timeouts
  readonly RBGC_MAX_CONSISTENCY_SEC=90
  readonly RBGC_EVENTUAL_CONSISTENCY_SEC=3
  # Consecutive 404s rbuh_poll_until_gone requires before declaring a deleted
  # resource durably gone. GCP IAM's SA read path is multi-replica eventually-
  # consistent: a post-DELETE GET flaps 200<->404 for seconds as replicas
  # converge, so a single 404 is not durable proof — a same-name recreate can
  # still race a lagging replica's existence preflight. The streak debounces
  # that flap; any intervening 200 resets it. Bounded by RBGC_MAX_CONSISTENCY_SEC.
  readonly RBGC_GONE_CONFIRM_STREAK=3
  readonly RBGC_SA_KEY_CREATE_RETRY_MAX=7
  readonly RBGC_SA_KEY_CREATE_RETRY_DELAY_SEC=10

  # JWT-bearer consumer-side retry — every RBRA consumer absorbs the post-write
  # race where Google's OAuth backend has not yet accepted a freshly-minted SA
  # or its key, surfaced as `invalid_grant` paired with either
  # `Invalid JWT Signature.` (fresh-key lag) or `Invalid grant: account not
  # found` (fresh-SA lag). 90s budget locked by ₣BB pristine-tier contract;
  # cadence below.
  readonly RBGC_SA_KEY_CONSUMER_RETRY_BUDGET_SEC=90
  readonly RBGC_SA_KEY_CONSUMER_RETRY_INITIAL_DELAY_SEC=2
  readonly RBGC_SA_KEY_CONSUMER_RETRY_MAX_DELAY_SEC=15

  # IAM-grant propagation retry — exponential-backoff budget shared by every
  # get-modify-set IAM grant site in rbgi_IAM.sh plus the inline GAR retry in
  # rbgg_invest_director. Recognizes three propagation classes against the
  # same time budget: (A) forward member-visibility (HTTP 400 "does not
  # exist"), (B) backward member-visibility (HTTP 400 "is not deleted"),
  # (C) caller-recently-empowered (HTTP 403 from resource-scope IAM caches).
  # Class C is time-bounded only — real propagation succeeds within budget,
  # real denial waits the budget and fails cleanly. RBSCIP locks the profile.
  readonly RBGC_PROPAGATION_INITIAL_DELAY_SEC=3
  readonly RBGC_PROPAGATION_MAX_DELAY_SEC=20
  readonly RBGC_PROPAGATION_DEADLINE_SEC=420

  # HTTP transient-failure retry — bounded retry on curl-network blips
  # (connection refused 7, timeout 28, TLS handshake 35, recv failure 56).
  # Shared by rbuh_json and the OAuth token-mint POST. Other curl exits
  # are configuration-deterministic and fail fast.
  readonly RBGC_HTTP_TRANSIENT_RETRY_ATTEMPTS=3
  readonly RBGC_HTTP_TRANSIENT_RETRY_SLEEP_SEC=3

  # docker login daemon->registry transient — the moby/moby#44350 signature.
  # docker login's registry-auth client carries a hardcoded, non-configurable
  # 15s timeout (moby/registry/auth.go); against a healthy-but-slow GAR auth
  # backend it fires prematurely, emitting this Go net/http stdlib string.
  # Alone among login/pull/push, login carries no internal retry, so callers
  # wrap it (rbgo_docker_login, zrbndb_docker_login) reusing the HTTP retry
  # budget above. The string is a Go standard-library invariant, stable across
  # docker versions; this is a surveyed-signature allowlist, NOT a catch-all —
  # real auth failures emit "unauthorized" and fail fast.
  readonly RBGC_DOCKER_LOGIN_TRANSIENT_SIGNATURE='Client.Timeout exceeded while awaiting headers'

  # docker login credential-persist failure under headless Cygwin. Docker
  # Desktop's docker-credential-wincred backend cannot reach the Windows
  # Credential Manager from an sshd session that owns no interactive logon
  # (cmdkey /list is empty over SSH), emitting this Win32 string (rc=1) AFTER
  # auth has already succeeded — the token mint and the push path are sound;
  # only the credential STORE fails. Windows docker also ignores an empty
  # credsStore (the CLI still detects wincred), so config alone cannot divert
  # the store to the file store. At this Palisade (docker's own credential store,
  # source we cannot edit) rbgo_docker_login bends ONCE on this exact signature:
  # since auth already succeeded it writes the credential into the base64 file
  # store itself (the config.json `auths` map docker push reads directly — the
  # form WSL uses natively) and treats login as done. A real auth failure emits
  # "unauthorized" and never matches. Stable Win32 system-error message;
  # corroborated by docker/cli#4353 and #1263. REMOVE the bend when the
  # uncontrolled-Cygwin host gains an interactive Windows logon / working vault.
  readonly RBGC_DOCKER_WINCRED_HEADLESS_SIGNATURE='A specified logon session does not exist'

  # URL Roots & Well-known Endpoints
  readonly RBGC_OAUTH_TOKEN_URL="https://oauth2.googleapis.com/token"
  readonly RBGC_OAUTH_AUTHORIZE_URL="https://accounts.google.com/o/oauth2/v2/auth"
  readonly RBGC_OAUTH_USERINFO_URL="https://www.googleapis.com/oauth2/v3/userinfo"
  readonly RBGC_API_ROOT_IAM="https://iam.googleapis.com"
  readonly RBGC_API_ROOT_CRM="https://cloudresourcemanager.googleapis.com"
  readonly RBGC_API_ROOT_SERVICEUSAGE="https://serviceusage.googleapis.com"
  readonly RBGC_API_ROOT_ARTIFACTREGISTRY="https://artifactregistry.googleapis.com"
  readonly RBGC_API_ROOT_CLOUDBUILD="https://cloudbuild.googleapis.com"
  readonly RBGC_API_ROOT_CLOUDBILLING="https://cloudbilling.googleapis.com"
  readonly RBGC_API_ROOT_STORAGE="https://storage.googleapis.com"
  readonly RBGC_API_ROOT_SECRETMANAGER="https://secretmanager.googleapis.com"
  readonly RBGC_CONSOLE_URL="https://console.cloud.google.com/"
  readonly RBGC_SIGNUP_URL="https://cloud.google.com/free"

  # OAuth Scopes
  readonly RBGC_SCOPE_CLOUD_PLATFORM="https://www.googleapis.com/auth/cloud-platform"

  # Service Usage Service Identifiers
  readonly RBGC_SERVICE_IAM="iam.googleapis.com"
  readonly RBGC_SERVICE_CRM="cloudresourcemanager.googleapis.com"
  readonly RBGC_SERVICE_ARTIFACTREGISTRY="artifactregistry.googleapis.com"

  # Email/Domain Assembly
  readonly RBGC_SA_EMAIL_DOMAIN="iam.gserviceaccount.com"

  # API Version Paths
  readonly RBGC_IAM_V1="/v1"
  readonly RBGC_CRM_V1="/v1"
  readonly RBGC_CRM_V3="/v3"
  readonly RBGC_SERVICEUSAGE_V1="/v1"
  readonly RBGC_SERVICEUSAGE_V1BETA1="/v1beta1"
  readonly RBGC_ARTIFACTREGISTRY_V1="/v1"
  readonly RBGC_CLOUDBUILD_V1="/v1"
  readonly RBGC_CLOUDBILLING_V1="/v1"
  readonly RBGC_STORAGE_JSON_V1="/storage/v1"
  readonly RBGC_STORAGE_JSON_UPLOAD="/upload/storage/v1"
  readonly RBGC_SECRETMANAGER_V1="/v1"

  # REST Path Fragments
  readonly RBGC_PATH_PROJECTS="/projects"
  readonly RBGC_PATH_LOCATIONS="/locations"
  readonly RBGC_PATH_REPOSITORIES="/repositories"
  readonly RBGC_PATH_SERVICE_ACCOUNTS="/serviceAccounts"
  readonly RBGC_PATH_KEYS="/keys"

  # REST Operation Suffixes
  readonly RBGC_CRM_GET_IAM_POLICY_SUFFIX=":getIamPolicy"
  readonly RBGC_CRM_SET_IAM_POLICY_SUFFIX=":setIamPolicy"
  readonly RBGC_SERVICEUSAGE_ENABLE_SUFFIX=":enable"
  readonly RBGC_SERVICEUSAGE_PATH_SERVICES="/services"

  # Operation Prefixes
  readonly RBGC_OP_PREFIX_GLOBAL="operations/"

  # Ark Artifact Basenames (₢A_AAK layout)
  # Each ark type is a plain basename sibling under rbi_hm/<hallmark>/.
  readonly RBGC_ARK_BASENAME_IMAGE="image"
  readonly RBGC_ARK_BASENAME_ABOUT="about"
  readonly RBGC_ARK_BASENAME_VOUCH="vouch"
  readonly RBGC_ARK_BASENAME_DIAGS="diags"
  readonly RBGC_ARK_BASENAME_ATTEST="attest"
  readonly RBGC_ARK_BASENAME_POUCH="pouch"

  # Hallmark Prefix Letters
  # Encode artifact provenance in the leading character of a hallmark stamp.
  # Kludge hallmarks are local-only; the other three originate in GAR.
  readonly RBGC_HALLMARK_PREFIX_CONJURE="c"
  readonly RBGC_HALLMARK_PREFIX_KLUDGE="k"
  readonly RBGC_HALLMARK_PREFIX_BIND="b"
  readonly RBGC_HALLMARK_PREFIX_GRAFT="g"

  # GAR Categorical Namespaces (₢A_AAK layout)
  # Top-level namespaces under which arks are stored. Consumed by rbgl_GarLayout.sh.
  # rbi_hm holds Director-authored image families; rbi_df holds
  # Payor-authored depot-scoped OCI artifacts produced during depot lifetime.
  readonly RBGC_GAR_CATEGORY_HALLMARKS="rbi_hm"
  readonly RBGC_GAR_CATEGORY_DEPOT_FACTS="rbi_df"
  readonly RBGC_GAR_CATEGORY_LODES="rbi_ld"

  # GAR Lode Layout (fetched-side universal capture — see RBSL)
  # One Lode = one GAR package named rbi_ld/<kind-letter><stamp>; that package
  # IS the atomic-delete unit (single `packages delete`). Members and provenance
  # ride as TAGS within the one package, never as /-path-segments (GAR has no
  # subtree delete). Stamp matches the hallmark second-granular form YYMMDDHHMMSS.
  #
  # Kind letters (one per capture kind; podvm carries two for its quay families):
  #   b  bole | r  reliquary | w  wsl | vw  podvm-wsl | vn  podvm-native
  # Five kinds, not six: the single-tool kind ('t') was dropped as non-load-
  # bearing — it split from reliquary on cardinality alone, has no consumer
  # (tool images are only ever consumed as a co-versioned cohort), and the
  # package layer already carries 1-vs-N for free (RBSL "Why five kinds, not four").
  readonly RBGC_LODE_KIND_BOLE="b"
  readonly RBGC_LODE_KIND_RELIQUARY="r"
  readonly RBGC_LODE_KIND_WSL="w"
  readonly RBGC_LODE_KIND_PODVM_WSL="vw"
  readonly RBGC_LODE_KIND_PODVM_NATIVE="vn"

  # Kind-brand enum — the touchmark's kind carried in the host-side single-form
  # chaining fact a derived-pull election reads to resolve the member tag. It is
  # read as its own fact, NOT parsed from the touchmark's kind-letter prefix
  # (the chaining channel is single-form: opaque values, never parsed). Each kind
  # adds its brand here with its vertical; the brand string is also the envelope's
  # `kind` field, and — for podvm — the operator-typed `immure` family argument.
  readonly RBGC_LODE_BRAND_BOLE="bole"
  readonly RBGC_LODE_BRAND_RELIQUARY="reliquary"
  readonly RBGC_LODE_BRAND_WSL="wsl"
  readonly RBGC_LODE_BRAND_PODVM_WSL="podvm-wsl"
  readonly RBGC_LODE_BRAND_PODVM_NATIVE="podvm-native"

  # Member/provenance tags. The rbi_ sprue marks strings from RB's domain:
  # RB's authored lexicon (bole, vouch) and RB-measured-from-content values
  # (the digest). It does NOT mark foreign-cued strings — the sanitized-origin
  # tag is UNSPRUED (origin is a vessel cue), computed at capture, not a constant.
  readonly RBGC_LODE_TAG_SPRUE="rbi_"               # RB reserved tag prefix; member tags compose as <sprue><name>
  readonly RBGC_LODE_TAG_BOLE="rbi_bole"            # uniform greppable handle (bole singleton)
  readonly RBGC_LODE_TAG_VOUCH="rbi_vouch"          # one-per-Lode provenance envelope
  readonly RBGC_LODE_TAG_DIGEST_PREFIX="rbi_sha256-"  # canonical OCI digest tag: rbi_sha256-<full-hex>
  # reliquary cohort members carry the clean scheme :<sprue><tool> (e.g. rbi_gcrane)
  # — no digest/fingerprint layer; the tool name is RB-authored lexicon, so sprued.
  readonly RBGC_LODE_TAG_ROOTFS="rbi_rootfs"        # wsl singleton: the opaque rootfs blob member (RB-authored, sprued)

  # Provenance envelope (:rbi_vouch) — two honest trust grades, declared per Lode.
  # bole captures the durable-upstream grade; podvm-* carries the recorded grade.
  readonly RBGC_LODE_TRUST_VERIFIED="verified-against-published"
  readonly RBGC_LODE_TRUST_RECORDED="recorded-at-acquisition"
  # rbld-vouch-2: the rblv_ sprue migration (ACGm_108, first application).
  # rbld-vouch-3: rblv_git_commit added — the dispatching HEAD commit, stamped at
  #   the shared vouch-push step (rbgjl02) from the spine-injected substitution.
  # Pre-MVP: no back-compat; every author writes rblv_ keys, augur reads rblv_ ONLY.
  readonly RBGC_LODE_VOUCH_SCHEMA="rbld-vouch-3"    # unsigned, schema-versioned, rblv_ sprue

  # wsl-kind acquisition convention — NOT a resolved coordinate (see RBSLU). Per
  # the no-FQIN premise, intent stays declarative and the pipeline computes the
  # coordinate: underpin takes the substrate version as ARGUMENTS (release + point,
  # e.g. `24.04 4`); the host _capture function assembles the tarball URL from this
  # path-convention template, and the cloud step DISCOVERS the checksum at capture
  # — it fetches Canonical's published, GPG-signed SHA256SUMS, verifies the
  # signature against the pinned signing-key fingerprint below, then verifies the
  # rootfs bytes. No full URL and no digest are pinned here; advancing the version
  # is a different argument, not a constant edit. The template bets on Canonical's
  # cdimage path scheme staying stable — a fail-loud bet (a 404 / missing sums-line
  # dies clean). printf args: (release, release.point, arch).
  #
  # Palisade note (RBSLU): the paddock named cloud-images.ubuntu.com/wsl/, but that
  # path retired its checksummed-tarball publication — noble/ now ships only
  # .manifest files and a Store-delivered .wsl. Ubuntu Base (cdimage) is the
  # genuinely-checksummed, GPG-signed, wsl --import-shaped Canonical equivalent.
  # Acquisition-only this heat, so the distro flavor is not load-bearing; the
  # WSL-specific seed is a consumption-time re-pin (wsl --import deferred — see
  # RBSLU, paddock Heat nature). The signing fingerprint is per-source: re-pin it
  # with any source change. Retire the note if the wsl/ tarball publication returns.
  readonly RBGC_LODE_WSL_URL_TEMPLATE="https://cdimage.ubuntu.com/ubuntu-base/releases/%s/release/ubuntu-base-%s-base-%s.tar.gz"
  readonly RBGC_LODE_WSL_ARCH_DEFAULT="amd64"
  # Ubuntu CD Image Automatic Signing Key (2012, RSA4096), cdimage@ubuntu.com —
  # signs cdimage SHA256SUMS.gpg. The trust anchor: tiny, stable, auditable. The
  # cloud step fetches the key BY this fingerprint into a clean keyring, so the
  # keyserver is never trusted — only a signature from exactly this key passes.
  readonly RBGC_LODE_WSL_SIGNING_FPR="843938DF228D22F7B3742BC0D94AA3F0EFE21092"

  # podvm-kind acquisition convention — NOT resolved coordinates (see RBSLI). Per
  # the no-FQIN premise, intent stays declarative and the pipeline resolves leaf
  # digests at capture: immure takes the quay FAMILY and the podman VERSION as
  # arguments (e.g. `podvm-wsl 5.6`); the cloud select step reads the family's
  # multi-arch OCI index at that version and picks the curated {disktype × arch}
  # leaves below from the index child DESCRIPTOR's platform.architecture +
  # annotations.disktype — never the layer filename, which is unreliable (the 5.6
  # wsl x86_64 leaf is titled `5.0-rootfs-amd64.tar.zst`; memo-20260608 §3.4). No
  # digest is pinned; advancing the version is a different argument, not a constant
  # edit. Trust grade is recorded-at-acquisition — quay rotates podvm out within
  # days and publishes no durable checksum, so RB attests only the digest captured.
  #
  # Two quay families, one verb (immure spans both via the family argument). The
  # disktype leaves carry the ALT arch spelling (x86_64/aarch64), not the OCI
  # amd64/arm64 the plain container children use — selection keys on that spelling.
  readonly RBGC_LODE_PODVM_FAMILY_WSL="quay.io/podman/machine-os-wsl"
  readonly RBGC_LODE_PODVM_FAMILY_NATIVE="quay.io/podman/machine-os"
  # Curated leaf selection per family — declarative `disktype:arch` rows the select
  # step matches against index child descriptors (alt arch spelling: x86_64/aarch64).
  # Member tag composes as :<sprue><disktype>-<arch> (e.g. rbi_wsl-x86_64);
  # disktype+arch are RB-selected from the index, so sprued.
  # WSL family: 2-leaf set (both wsl-disktype leaves). The podvm-wsl fixture proves
  # this end-to-end with the multi-member machinery and per-member-jettison path.
  readonly RBGC_LODE_PODVM_WSL_SELECTION="wsl:x86_64 wsl:aarch64"
  # Native family: full 8-leaf curation — {applehv, hyperv, qemu, wsl} × {x86_64, aarch64}.
  # Sourced from memo-20260608 §5 (the machine-os index carries exactly these 8 disktype
  # children at the 5.6 observation point; the 2 plain-container children are EXCLUDED
  # by disktype-key selection). This pace (lode-podvm-platform-fanout) lands full
  # curation; the "FOLLOWING pace" deferral is retired. Native full-curation is gated
  # one-time (not a recurring service fixture) — see the podvm-lifecycle fixture comment.
  readonly RBGC_LODE_PODVM_NATIVE_SELECTION="applehv:x86_64 applehv:aarch64 hyperv:x86_64 hyperv:aarch64 qemu:x86_64 qemu:aarch64 wsl:x86_64 wsl:aarch64"

  # rbi_df layout: flat namespace. No subdirs. Each filename names one
  # depot-scoped artifact; tag varies by artifact role.
  #
  # Current artifacts:
  #   probe-tether:probe   — tether pool levy-time capability probe
  #                          (cloud-pushed marker image, FROM scratch)
  #   probe-airgap:probe   — airgap pool levy-time capability probe
  #                          (cloud-pushed marker image, FROM scratch)
  #   rbrd:tripwire        — depot regime tripwire (FROM-scratch image
  #                          carrying rbmm_moorings/rbrd.env). Host-inscribed by
  #                          Payor at end of levy; pulled + byte-diffed
  #                          by every cloud-submitting command. See
  #                          Tools/rbk/rbndb_base.sh.
  #
  # The depot-time-immutable identity and pool settings (CLOUD_PREFIX,
  # DEPOT_MONIKER, GCP_REGION, GCB_MACHINE_TYPE) live in the RBRD regime
  # and are also inscribed into rbi_df at the rbrd:tripwire tag so post-
  # levy drift can be detected at every subsequent cloud submission.
  #
  # Enumerators (rbw-iah / rbw-iar) ignore rbi_df by design — its
  # contents are operational, not part of the hallmark/reliquary
  # image catalogue.

  # Reliquary Tool Basenames (cohort seeds for the conclave Lode)
  # Canonical tool names; the resolver composes RBGC_LODE_TAG_SPRUE onto each to
  # address the :rbi_<tool> member tags on the one rbi_ld/<touchmark> package.
  # Authoritative cohort manifest lives in rbgjl/rbgjl03-conclave-capture.sh.
  readonly RBGC_RELIQUARY_TOOL_GCLOUD="gcloud"
  readonly RBGC_RELIQUARY_TOOL_DOCKER="docker"
  readonly RBGC_RELIQUARY_TOOL_ALPINE="alpine"
  readonly RBGC_RELIQUARY_TOOL_SYFT="syft"
  readonly RBGC_RELIQUARY_TOOL_BINFMT="binfmt"
  # gcrane is in the cohort for two reasons: (1) sealed-reliquary-consuming captures
  # (bole/wsl) resolve a PINNED gcrane builder from the reliquary, never the floating
  # gcr.io bootstrap (supply-chain pinning boundary — RBS0 rbsk_pinning_boundary,
  # RBSCB); (2) the bind mirror step (rbgjm01) uses gcrane cp for registry-to-registry
  # copy, authenticating GAR ambiently via google.Keychain. Mirrored as the :debug
  # variant (busybox shell) so the resolved builder carries the orchestration shell
  # its steps need.
  readonly RBGC_RELIQUARY_TOOL_GCRANE="gcrane"

  # Fact-file filenames (written to BURD_OUTPUT_DIR by producers, read by tests)
  readonly RBF_FACT_HALLMARK="rbf_fact_hallmark"
  readonly RBF_FACT_BUILD_ID="rbf_fact_build_id"
  readonly RBF_FACT_GAR_ROOT="rbf_fact_gar_root"
  readonly RBF_FACT_ARK_STEM="rbf_fact_ark_stem"
  readonly RBF_FACT_ARK_YIELD="rbf_fact_ark_yield"
  readonly RBF_FACT_RELIQUARY="rbf_fact_reliquary"

  # Lode capture chaining facts (single-form, fixed filenames). Ensconce is
  # capture-pure and writes no consumer config; it hands the bole touchmark to a
  # later derived-pull election (the conjure ANCHOR populator) through these two
  # bare facts via the depth-1 cross-tabtarget chain. The provenance envelope
  # lives only in GAR (:rbi_vouch), never host-side. TOUCHMARK carries the Lode
  # stamp (e.g. b260602120000); BRAND carries the kind enum (RBGC_LODE_BRAND_*).
  readonly RBF_FACT_LODE_TOUCHMARK="rbf_fact_lode_touchmark"
  readonly RBF_FACT_LODE_BRAND="rbf_fact_lode_brand"

  # Payor fact-file filenames (governor identifying values)
  readonly RBGP_FACT_GOVERNOR_SA_EMAIL="rbgp_fact_governor_sa_email"

  # Depot lifecycle-state vocabulary. Fact-file extensions live in RBCC.
  # rbgp_depot_list emits one fact file per known depot at
  # "<cloud_prefix>/<moniker>.${RBCC_fact_ext_depot}" with content equal to
  # one of the values below. The cloud_prefix subdir prevents collisions
  # between same-moniker depots under different cloud_prefixes.
  readonly RBGP_DEPOT_STATE_COMPLETE="COMPLETE"
  readonly RBGP_DEPOT_STATE_DELETE_REQUESTED="DELETE_REQUESTED"

  # DisplayName anchor used across depot-creation sites (depot project,
  # Mason SA, Governor SA). Search backend filters CRM v3 projects:search
  # by displayName starting with this anchor. Distinct, unmistakable string
  # ensures no collision with non-depot projects in the operator's account.
  readonly RBGC_DEPOT_DISPLAY_PREFIX="RBGC-DEPOT"

  # Artifact Registry (GAR) Composition
  readonly RBGC_GAR_HOST_SUFFIX="-docker.pkg.dev"

  # GAR Cleanup Policy (applied at depot levy — see RBSDE "Create Container Repository").
  # Reaps untagged manifests on GAR's daily cleanup cadence; underwrites the V2-DELETE-by-tag
  # contract documented in RBSIJ for multi-platform orphan children.
  readonly RBGC_GAR_CLEANUP_POLICY_ID="rb-delete-untagged"
  readonly RBGC_GAR_CLEANUP_OLDER_THAN_SEC="86400s"


  # Canonical Role IDs
  readonly RBGC_ROLE_ARTIFACTREGISTRY_READER="roles/artifactregistry.reader"
  readonly RBGC_ROLE_ARTIFACTREGISTRY_WRITER="roles/artifactregistry.writer"
  readonly RBGC_ROLE_ARTIFACTREGISTRY_ADMIN="roles/artifactregistry.admin"
  readonly RBGC_ROLE_CONTAINERANALYSIS_OCCURRENCES_VIEWER="roles/containeranalysis.occurrences.viewer"
  readonly RBGC_ROLE_CLOUDBUILD_BUILDS_EDITOR="roles/cloudbuild.builds.editor"

  # Common API Base Paths (project-independent)
  readonly RBGC_API_BASE_GCS="${RBGC_API_ROOT_STORAGE}${RBGC_STORAGE_JSON_V1}"

  # Cloud Resource Manager - Liens API
  readonly RBGC_API_CRM_LIST_LIENS="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens"
  readonly RBGC_API_CRM_DELETE_LIEN="${RBGC_API_ROOT_CRM}${RBGC_CRM_V1}/liens"

  # Google Cloud Storage (GCS) APIs (project-independent)
  readonly RBGC_API_GCS_BUCKETS="${RBGC_API_BASE_GCS}/b"

  readonly RBGC_BUILD_RUNNER_PLATFORM="linux/amd64"


  # Worker pool infrastructure (dual pools: tether + airgap)
  readonly RBGC_POOL_SUFFIX_TETHER="-tether"
  readonly RBGC_POOL_SUFFIX_AIRGAP="-airgap"
  readonly RBGC_WORKER_POOL_SUFFIX="-pool"
  readonly RBGC_PATH_WORKER_POOLS="/workerPools"

  readonly ZRBGC_KINDLED=1
}

zrbgc_sentinel() {
  test "${ZRBGC_KINDLED:-}" = "1" || buc_die "Module rbgc not kindled - call zrbgc_kindle first"
}

# eof

