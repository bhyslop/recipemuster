# Google Cloud Platform Container Infrastructure Setup

## Overview
Bootstrap GCP infrastructure by creating a temporary provisioner service account with Project Owner privileges.
The provisioner will automate the creation of operational service accounts and infrastructure configuration.

## Prerequisites
- Credit card for GCP account verification (won't be charged on free tier)
- Email address not already associated with GCP

---

## Phase 1: Manual Provisioner Setup (Required First)

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
3. Create → Wait for notification "Creating project..." to complete
4. Select project from dropdown when ready

### 2. Create Provisioner Service Account

Navigate to IAM & Admin section:
1. Left sidebar → IAM & Admin → Service Accounts
2. If prompted about APIs, click "Enable API"
3. Wait for "Identity and Access Management (IAM) API" to enable

Create the provisioner:
1. Click "+ CREATE SERVICE ACCOUNT" at top
2. Service account details:
   - Service account name: `rbra-provisioner`
   - Service account ID: (auto-fills as `rbra-provisioner`)
   - Description: `Temporary provisioner for infrastructure setup - DELETE AFTER USE`
3. Click "CREATE AND CONTINUE"

### 3. Assign Project Owner Role

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

---

ELIDED