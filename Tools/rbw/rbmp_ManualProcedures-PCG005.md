# Google Cloud Platform Container Infrastructure Setup

## Overview
Establish Google Artifact Registry and Cloud Build for container lifecycle management with three service accounts: GAR Admin (push/pull/delete), GCB Submitter (build initiation), and GAR Reader (pull only).

## Prerequisites
- Credit card for GCP account verification (won't be charged on free tier)
- Email address not already associated with GCP

## Procedure

### 1. Create GCP Account and Project

Open browser, navigate to https://cloud.google.com/free

1. Click "Get started for free"
2. Sign in with Google account or create new
3. Provide:
    - Country
    - Organization type: Individual
    - Credit card (verification only)
4. Accept terms → Start my free trial

After account creation, Google Cloud Console opens.

Create new project:
1. Top bar project dropdown → New Project
2. Configure:
    - Project name: `recipemuster-prod`
    - Leave organization as "No organization"
3. Create → Wait for notification
4. Select project from dropdown

### 2. Enable Required APIs

Open terminal ⟦LOCAL-SETUP⟧:

```bash
# GCIS = Google Cloud Infrastructure Setup
export GCIS_PROJECT_ID=recipemuster-prod
export GCIS_REGION=us-central1
export GCIS_BUCKET_NAME=recipemuster-builds
export GCIS_REPO_NAME=recipemuster
```

```bash
gcloud config set project ${GCIS_PROJECT_ID}

gcloud services enable \
  artifactregistry.googleapis.com \
  cloudbuild.googleapis.com \
  containerregistry.googleapis.com \
  storage-component.googleapis.com
```

### 3. Create Artifact Registry Repository

```bash
gcloud artifacts repositories create ${GCIS_REPO_NAME} \
  --repository-format=docker \
  --location=${GCIS_REGION} \
  --description="Recipe Bottle container images"
```

### 4. Create Cloud Build Staging Bucket

```bash
gsutil mb -l ${GCIS_REGION} gs://${GCIS_BUCKET_NAME}

# Grant Cloud Build service account access
export GCIS_BUILD_SA=$(gcloud projects describe ${GCIS_PROJECT_ID} \
  --format='value(projectNumber)')@cloudbuild.gserviceaccount.com

gsutil iam ch serviceAccount:${GCIS_BUILD_SA}:objectAdmin \
  gs://${GCIS_BUCKET_NAME}
```

### 5. Configure Cloud Build for GAR Access

```bash
# Grant Cloud Build permission to push to GAR
gcloud artifacts repositories add-iam-policy-binding ${GCIS_REPO_NAME} \
  --location=${GCIS_REGION} \
  --member=serviceAccount:${GCIS_BUILD_SA} \
  --role=roles/artifactregistry.writer
```

### 6. Create Service Accounts

```bash
# Create GAR Admin (full control)
gcloud iam service-accounts create rbra-gar-admin \
  --display-name="Recipe Bottle GAR Admin"

# Create GCB Submitter (build initiation)
gcloud iam service-accounts create rbra-gcb-submitter \
  --display-name="Recipe Bottle Build Submitter"

# Create GAR Reader (pull only)
gcloud iam service-accounts create rbra-gar-reader \
  --display-name="Recipe Bottle GAR Reader"
```

### 7. Assign IAM Roles

```bash
# GAR Admin - full repository control
gcloud artifacts repositories add-iam-policy-binding ${GCIS_REPO_NAME} \
  --location=${GCIS_REGION} \
  --member=serviceAccount:rbra-gar-admin@${GCIS_PROJECT_ID}.iam.gserviceaccount.com \
  --role=roles/artifactregistry.admin

# GCB Submitter - submit builds and access staging
gcloud projects add-iam-policy-binding ${GCIS_PROJECT_ID} \
  --member=serviceAccount:rbra-gcb-submitter@${GCIS_PROJECT_ID}.iam.gserviceaccount.com \
  --role=roles/cloudbuild.builds.editor

gsutil iam ch \
  serviceAccount:rbra-gcb-submitter@${GCIS_PROJECT_ID}.iam.gserviceaccount.com:objectAdmin \
  gs://${GCIS_BUCKET_NAME}

# GAR Reader - read only
gcloud artifacts repositories add-iam-policy-binding ${GCIS_REPO_NAME} \
  --location=${GCIS_REGION} \
  --member=serviceAccount:rbra-gar-reader@${GCIS_PROJECT_ID}.iam.gserviceaccount.com \
  --role=roles/artifactregistry.reader
```

### 8. Generate Service Account Keys

```bash
# Create keys directory
mkdir -p ../station-files/secrets
cd ../station-files/secrets

# Generate key files
gcloud iam service-accounts keys create rbra-gar-admin-key.json \
  --iam-account=rbra-gar-admin@${GCIS_PROJECT_ID}.iam.gserviceaccount.com

gcloud iam service-accounts keys create rbra-gcb-submitter-key.json \
  --iam-account=rbra-gcb-submitter@${GCIS_PROJECT_ID}.iam.gserviceaccount.com

gcloud iam service-accounts keys create rbra-gar-reader-key.json \
  --iam-account=rbra-gar-reader@${GCIS_PROJECT_ID}.iam.gserviceaccount.com
```

### 9. Create RBRA Environment Files

```bash
# GAR Admin (600s for maintenance operations)
cat > rbra-gar-admin.env << 'EOF'
RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/rbra-gar-admin-key.json
RBRA_TOKEN_LIFETIME_SEC=600
EOF

# GCB Submitter (600s for build operations)
cat > rbra-gcb-submitter.env << 'EOF'
RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/rbra-gcb-submitter-key.json
RBRA_TOKEN_LIFETIME_SEC=600
EOF

# GAR Reader (300s for quick pulls)
cat > rbra-gar-reader.env << 'EOF'
RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/rbra-gar-reader-key.json
RBRA_TOKEN_LIFETIME_SEC=300
EOF
```

### 10. Update RBRR Configuration

```bash
cd ../../recipe-bottle-regime

# Update configuration with actual values
sed -i.bak \
  -e "s/your-project-id/${GCIS_PROJECT_ID}/" \
  -e "s/your-build-staging-bucket/${GCIS_BUCKET_NAME}/" \
  rbrr_RecipeBottleRegimeRepo.sh

# Update service account file paths
sed -i \
  -e 's|rbrs-gar\.env|rbra-gar-admin.env|' \
  -e 's|rbrs-gcb\.env|rbra-gcb-submitter.env|' \
  rbrr_RecipeBottleRegimeRepo.sh
```

### 11. Verify Setup

```bash
# Test GAR access
export RBRA_SERVICE_ACCOUNT_KEY=../station-files/secrets/rbra-gar-reader-key.json
export RBRA_TOKEN_LIFETIME_SEC=300

gcloud auth activate-service-account --key-file=${RBRA_SERVICE_ACCOUNT_KEY}
gcloud artifacts repositories list --location=${GCIS_REGION}
```

Expected output shows `recipemuster` repository.

```bash
# Test Cloud Build (creates minimal test image)
cat > test.Dockerfile << 'EOF'
FROM alpine:3.19
RUN echo "Setup verified"
EOF

gcloud builds submit \
  --config=Tools/rbia_cloudbuild.yaml \
  --substitutions=RBIA_DOCKERFILE=test.Dockerfile,RBIA_TAG=setup-test,RBIA_MONIKER=verify,RBIA_PLATFORMS=linux/amd64,RBIA_GAR_LOCATION=${GCIS_REGION},RBIA_GAR_PROJECT=${GCIS_PROJECT_ID},RBIA_GAR_REPOSITORY=${GCIS_REPO_NAME},RBIA_GIT_COMMIT=test,RBIA_GIT_BRANCH=test,RBIA_GIT_REPO=test,RBIA_RECIPE_NAME=test \
  .

# Cleanup
rm test.Dockerfile
```

## Post-Setup Notes

Service accounts created:
- `rbra-gar-admin`: Full GAR control (push/pull/delete)
- `rbra-gcb-submitter`: Submit builds to Cloud Build
- `rbra-gar-reader`: Pull images only

Key files stored in `../station-files/secrets/`

⚠️ These JSON key files grant service access - protect them like passwords.
