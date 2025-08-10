
# Provisioner Role Safety Recommendations (RBGO + curl)

This document outlines safeguards for using a **temporary high-privilege “provisioner” service account** when bootstrapping Recipe Bottle Workbench services via RBGO (JWT→OAuth) and direct Google Cloud REST API calls.

## 1. Scope & Isolation

* **Dedicated Project**
  Create a fresh GCP project solely for bootstrap operations. Do not reuse existing projects containing unrelated workloads.
* **Project-Scoped Owner**
  Grant `roles/owner` **only** at the project level (`projects/{projectId}`), never at organization or folder level.
* **Explicit Resource Targeting**
  All API calls must explicitly include the target `projectId` (and region/location where applicable). Never rely on defaults.

## 2. Credentials & Token Management

* **Short-Lived Tokens**
  Use minimal allowed token lifetimes (e.g., 1800 seconds) for the provisioner.
* **Key Lifecycle**

  * Create JSON key for the provisioner service account only when needed.
  * Store securely (`chmod 600`), never commit to version control, and delete after use.
  * Destroy both the key and the service account immediately after bootstrap.
* **Secret Storage**
  Store in a `.gitignore`d directory. Use restrictive file permissions and shred securely on teardown.

## 3. Preventing Accidental Impact

* **Resource Verification Before Action**

  * Confirm `projectId` and `projectNumber` match expectations before any mutating request.
  * Refuse to operate if mismatched.
* **Explicit Whitelisting**
  Allow mutations only if resource names (project, bucket, repo) match an approved pattern (e.g., `rbm-*`).
* **Blast Radius Containment**
  Never reference shared or global resources (e.g., default network) in API calls.

## 4. Least Privilege for Permanent Accounts

* **Role Narrowing**
  Use the provisioner only to create permanent, least-privilege service accounts (e.g., GAR Reader, GCB Submitter).
  Assign only the granular roles needed for each service.
* **Token Switching**
  Once permanent accounts exist, switch all operations to their credentials; never continue using the provisioner.

## 5. API Enablement & Service Use

* **Enable Minimal APIs**
  Through the Service Usage API, enable only the specific services required for bootstrap (IAM, Artifact Registry, Cloud Build, Storage if needed).
* **Avoid Unnecessary Services**
  Do not enable APIs not directly required by the bootstrap workflow.

## 6. Human-Factor Controls

* **Dry-Run Mode**
  Provide a mode that prints planned API calls without sending them.
* **Kill Switch for Mutations**
  Require an explicit environment flag (e.g., `RBM_FORCE=1`) before any write/delete request.
* **Redacted Logging**
  Ensure no logs contain tokens, key material, or full Authorization headers.
* **Clear Warnings**
  At the start of any bootstrap procedure, print a prominent notice:

  > “These operations will affect **only** project `{projectId}`. Abort if this is not correct.”

## 7. Teardown & Audit

* **Immediate Cleanup**
  After bootstrap:

  * Delete provisioner keys.
  * Delete the provisioner service account.
  * Confirm removal via IAM policy inspection.
* **Audit Readiness**
  Maintain a record of:

  * Project ID and number
  * Enabled APIs
  * Roles granted to permanent service accounts
  * Date/time of provisioner creation and deletion


