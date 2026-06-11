# ₣BH Fable Recommendation — Pin the Delete Builder by Digest

Date: 2026-06-10

Status: Recommendation from the Fable review of the cloud-dispatch delete architecture
(commit 4f8a5c703). Not a blocker for the landed pace; priority hardening.
TRIAGED 2026-06-10: folded into the cloud-delete-hardening pace (₢BHAAh, wrapped) — ZRBFC_DELETE_BUILDER digest-pinned, RBSCB posture updated.

## The correctable behavior

`ZRBFC_DELETE_BUILDER="gcr.io/cloud-builders/gcloud:latest"` — minted in `zrbfc_kindle`
(`Tools/rbk/rbfc0_FoundryCore.sh`) — is a floating, unpinned builder image, and the delete
build runs **as the Director SA** (`zrbld_cloud_delete_dispatch`, `Tools/rbk/rbldd_Delete.sh`),
the most GAR-privileged identity in the system (`repoAdmin`, delete over the whole repository).

Every prior floating-builder use (underpin/wsl gcrane bootstrap) runs as **Mason, writer-only**
— that asymmetry was the design: the identity that executes unpinned or untrusted bytes never
holds delete authority. The delete build breaks the asymmetry: a poisoned `:latest` executes
with delete authority, and the in-pool metadata server hands it Director access tokens
(`metadata_token()` in `Tools/rbk/rbgjl/rbgjl06-package-delete.py` demonstrates the fetch).

The comment on the constant already cites the bootstrap-pin itch (RBS0 `rbsk_pinning_boundary`)
as accepted. The Director-identity pairing escalates that itch from hygiene to priority **for
this builder specifically**.

## Recommended repair

Pin `ZRBFC_DELETE_BUILDER` by digest:
`gcr.io/cloud-builders/gcloud@sha256:<digest>`, resolved once by hand (or by a small
`gcrane digest` gesture) and recorded in the constant with a dated comment.

- Preserves the reliquary-less property the delete build requires (a digest pin needs no
  reliquary; the delete of the last reliquary still works).
- The step's needs are frozen (python3 + urllib + json), so staleness cost is near zero;
  refresh the digest deliberately, as a reviewed one-line change, rather than riding `:latest`.
- Update the RBSCB "Cloud-Dispatched Tool-Plane Deletes" posture section to record the pin
  when done (it currently inherits the floating-bootstrap framing).
