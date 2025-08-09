# Google Cloud Platform Container Infrastructure Setup

## Overview
Bootstrap GCP infrastructure by creating a temporary provisioner service account with Project Owner privileges.
The provisioner will automate the creation of operational service accounts and infrastructure configuration.

## Prerequisites
- Credit card for GCP account verification (won't be charged on free tier)
- Email address not already associated with GCP

---

## Manual Provisioner Setup Procedure

Recipe Bottle setup requires a manual bootstrap procedure to enable enough control 

Open a web browser to https://cloud.google.com/free

1. **Establish Account**
   1. Click "Get started for free"
   1. Sign in with Google account or create new
   1. Provide:
      - Country
      - Organization type: Individual
      - Credit card (verification only)
   1. Accept terms → Start my free trial
   1. Expect Google Cloud Console to open.
1. **Create New Project**
   1. Top bar project dropdown → New Project
   1. Configure:
      - Project name: `recipemuster-prod`
      - Leave organization as "No organization"
   1. Create → Wait for notification "Creating project..." to complete
   1. Select project from dropdown when ready
1. **Create Provisioner Service Account**
   1. Navigate to IAM & Admin section
   1. Left sidebar → IAM & Admin → Service Accounts
   1. If prompted about APIs, click "Enable API"
   1. Wait for "Identity and Access Management (IAM) API" to enable
1. **Create the Provisioner**
   1. Click "+ CREATE SERVICE ACCOUNT" at top
   1. Service account details:
      - Service account name: `rbra-provisioner`
      - Service account ID: (auto-fills as `rbra-provisioner`)
      - Description: `Temporary provisioner for infrastructure setup - DELETE AFTER USE`
   1. Click "CREATE AND CONTINUE"
1. Assign Project Owner Role:

⚠️ **CRITICAL SECURITY WARNING**: This grants complete project control. Delete immediately after setup.

Grant access section:
1. Click "Select a role" dropdown
2. In filter box, type: `owner`
3. Select: **Basic → Owner**
4. Click "CONTINUE"
5. Grant users access section: Skip (click "DONE")

Service account list now shows `rbra-provisioner@recipemuster-prod.iam.gserviceaccount.com`

### 4. Generate Service Account Key

From service accounts list:
1. Click on `rbra-provisioner@recipemuster-prod.iam.gserviceaccount.com`
2. Top tabs → KEYS
3. Click "ADD KEY" → "Create new key"
4. Key type: **JSON** (should be selected)
5. Click "CREATE"

Browser downloads: `recipemuster-prod-[random].json`

6. Click "CLOSE" on download confirmation

### 5. Configure Local Environment

Open terminal ⟦LOCAL-SETUP⟧:

```bash
# Create secrets directory structure
mkdir -p ../station-files/secrets
cd ../station-files/secrets

# Move downloaded key (adjust path to your Downloads folder)
mv ~/Downloads/recipemuster-prod-*.json rbra-provisioner-key.json

# Verify key structure
jq -r '.type' rbra-provisioner-key.json
# Should output: service_account

# Create RBRA environment file
cat > rbra-provisioner.env << 'EOF'
RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/rbra-provisioner-key.json
RBRA_TOKEN_LIFETIME_SEC=1800
EOF

# Set restrictive permissions
chmod 600 rbra-provisioner-key.json
chmod 600 rbra-provisioner.env
```

### 6. Verify Provisioner Access

```bash
# Test authentication
export RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/rbra-provisioner-key.json
gcloud auth activate-service-account --key-file=${RBRA_SERVICE_ACCOUNT_KEY}

# Verify project access
gcloud projects describe recipemuster-prod --format="value(projectId)"
# Should output: recipemuster-prod
```

### ⚠️ POST-SETUP SECURITY REQUIREMENT

After completing Phase 2 infrastructure setup:

1. Return to Console → IAM & Admin → Service Accounts
2. Select checkbox for `rbra-provisioner`
3. Click trash icon → DELETE
4. Type `rbra-provisioner` to confirm
5. Click "DELETE"
6. Remove local key files:
```bash
rm ../station-files/secrets/rbra-provisioner-key.json
rm ../station-files/secrets/rbra-provisioner.env
```

---

## Phase 2: Automated Infrastructure Setup (Pseudocode)

*These steps will be automated using RBGO OAuth with the provisioner token*

### 1. Initialize Configuration
```
# Set project configuration
PROJECT_ID = recipemuster-prod
REGION = us-central1
BUCKET_NAME = recipemuster-builds
REPO_NAME = recipemuster
```

### 2. Enable Required APIs
```
# Using provisioner token via RBGO
Enable APIs:
  - artifactregistry.googleapis.com
  - cloudbuild.googleapis.com  
  - containerregistry.googleapis.com
  - storage-component.googleapis.com
```

### 3. Create Artifact Registry Repository
```
Create Docker repository:
  - Name: ${REPO_NAME}
  - Location: ${REGION}
  - Format: Docker
  - Description: "Recipe Bottle container images"
```

### 4. Create Cloud Build Staging Bucket
```
Create GCS bucket:
  - Name: ${BUCKET_NAME}
  - Location: ${REGION}
  - Grant Cloud Build service account objectAdmin
```

### 5. Configure Cloud Build Permissions
```
Grant Cloud Build service account:
  - Role: artifactregistry.writer
  - Repository: ${REPO_NAME}
```

### 6. Create Operational Service Accounts
```
Create service accounts:
  - rbra-gar-admin (Recipe Bottle GAR Admin)
  - rbra-gcb-submitter (Recipe Bottle Build Submitter)
  - rbra-gar-reader (Recipe Bottle GAR Reader)
```

### 7. Assign IAM Roles
```
rbra-gar-admin:
  - artifactregistry.admin on ${REPO_NAME}

rbra-gcb-submitter:
  - cloudbuild.builds.editor on project
  - storage.objectAdmin on ${BUCKET_NAME}

rbra-gar-reader:
  - artifactregistry.reader on ${REPO_NAME}
```

### 8. Generate Service Account Keys
```
For each service account:
  - Generate JSON key
  - Save to ../station-files/secrets/
  - Create corresponding RBRA env file
```

### 9. Create RBRA Environment Files
```
rbra-gar-admin.env:
  - Key path: rbra-gar-admin-key.json
  - Lifetime: 600 seconds

rbra-gcb-submitter.env:
  - Key path: rbra-gcb-submitter-key.json  
  - Lifetime: 600 seconds

rbra-gar-reader.env:
  - Key path: rbra-gar-reader-key.json
  - Lifetime: 300 seconds
```

### 10. Update RBRR Configuration
```
Update rbrr_RecipeBottleRegimeRepo.sh:
  - Set PROJECT_ID
  - Set BUCKET_NAME
  - Update service account file references
```

### 11. Verification Tests
```
Test each service account:
  - GAR Reader: List repositories
  - GAR Admin: Push test image
  - GCB Submitter: Submit test build
```

### 12. Cleanup
```
Delete provisioner:
  - Remove service account from project
  - Delete local key files
  - Remove RBRA environment file
```

---

## Final Infrastructure State

**Operational Service Accounts:**
- `rbra-gar-admin`: Full GAR control (push/pull/delete)
- `rbra-gcb-submitter`: Submit builds to Cloud Build
- `rbra-gar-reader`: Pull images only

**Storage:**
- Artifact Registry: `${REGION}-docker.pkg.dev/${PROJECT_ID}/${REPO_NAME}`
- Build Staging: `gs://${BUCKET_NAME}`

**Security:**
- Provisioner deleted
- Operational accounts use principle of least privilege
- Keys stored in `../station-files/secrets/`
