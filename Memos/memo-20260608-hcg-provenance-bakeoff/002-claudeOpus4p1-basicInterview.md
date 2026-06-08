# Guide for Writing IT Procedures

## Purpose

This guide defines patterns for writing durable, executable procedures for IT operations. These procedures are designed to be executed infrequently by engineers unfamiliar with the specific task, requiring clarity over brevity and precision over flexibility.

## Core Principles

**Trust but Verify**: Assume competent operators who can research unfamiliar concepts, but provide clear success indicators for each step.

**Deterministic Execution**: Every step should clearly pass or fail. No optional steps within a procedure.

**Copy-Paste Safety**: All commands must be directly executable via copy-paste without modification except where explicitly marked.

## Terminal Management

### Naming Contexts

When procedures require multiple terminal sessions, name each context clearly:

```bash
# «LOCAL» - Your workstation
git clone https://github.com/company/repo.git
```

```bash
# «BASTION» - Jump host terminal
ssh user@internal-host.local
```

```bash
# «K8S-PROD» - Production cluster context
kubectl get pods -n production
```

### Environment Setup

Group configuration at the beginning using prefixed variables. Each prefix should be 2-5 characters uniquely identifying this procedure:

```bash
# AWSD = AWS Deployment procedure
export AWSD_REGION=«your-target-region»
export AWSD_BUCKET=«s3-bucket-name»
export AWSD_VERSION=1.3.5
```

```bash
# Verify setup
echo "Deploying version ${AWSD_VERSION} to ${AWSD_REGION}" && \
test -n "${AWSD_BUCKET}" && \
echo SUCCESS
```

## Command Patterns

### Multi-line Commands

Always chain with `&&` to ensure failure propagation:

```bash
aws s3 cp file.tar.gz s3://${AWSD_BUCKET}/ && \
aws s3api put-object-acl \
  --bucket ${AWSD_BUCKET} \
  --key file.tar.gz \
  --acl public-read && \
echo SUCCESS
```

### Validation Steps

Make success explicit:

```bash
# Create working directory
export MCHR_TEMP_DIR=$(mktemp -d) && \
test -d ${MCHR_TEMP_DIR} && \
echo SUCCESS
```

Expected output: `SUCCESS`

### Dangerous Operations

Mark irreversible operations clearly:

```bash
# ⚠️ WARNING: This permanently deletes the database
dropdb ${PGDS_DATABASE} && echo "Database deleted"
```

## Secret Management

Use guillemets to mark values requiring manual substitution:

```bash
export GHPR_PAT=«your-github-personal-access-token»
# Get this from: Settings → Developer Settings → Personal Access Tokens
# Or ask Sarah from DevOps for the team token
```

## Output Documentation

Use inline backticks for expected simple output:

```bash
terraform plan -out=plan.tfplan
```
Expect to see `Plan: 3 to add, 0 to change, 0 to destroy`

For complex output, describe characteristics rather than exact text:
"Verify the output shows three new security groups being created"

## Procedure Types

### Console Procedures

Standard terminal-based operations following all patterns above.

### Web UI Procedures

Document what you're getting and why, be flexible on exact navigation:

1. Navigate to: AWS Console → EC2 → Security Groups
2. Create new security group with:
   - Name: `production-web-sg`
   - Description: `Allow HTTPS traffic`
   - VPC: Select your production VPC

3. Add inbound rule:
   - Type: HTTPS
   - Source: 0.0.0.0/0

### Application Installation Procedures

For installer wizards, capture defaults for future reference:

1. Run installer: `./install.sh`
2. Installation directory: Accept default `/opt/application` 
3. Memory allocation: Change to `4096MB` (default was 2048MB)
4. Click Next through remaining screens with defaults
5. When prompted for license key, enter: `${INST_LICENSE_KEY}`
6. Installation takes approximately 5 minutes

## File Operations

Keep file modifications inline and atomic:

```bash
# Backup and modify configuration
cp /etc/app.conf /etc/app.conf.backup-$(date +%Y%m%d) && \
sed -i 's/debug=false/debug=true/' /etc/app.conf && \
grep "debug=true" /etc/app.conf && \
echo SUCCESS
```

## Cleanup

Always clean up immediately after temporary operations:

```bash
# Deploy files
cp ${DEPL_TEMP_DIR}/*.yaml /opt/configs/ && \
rm -rf ${DEPL_TEMP_DIR} && \
echo "Deployment complete, temp files cleaned"
```

## Cross-references

When depending on other procedures:

"Prerequisites: Complete the 'AWS VPC Setup' procedure first"

## Duration Guidance

Note expected durations for long operations:

```bash
# Build container (typically 3-5 minutes)
docker build -t ${DKBD_IMAGE}:${DKBD_VERSION} .
```

## Example Procedure Structure

```markdown
# Deploy Redis Cluster to Kubernetes

Deploy a three-node Redis cluster to the production Kubernetes environment.

## Prerequisites

- kubectl configured for production cluster
- Helm 3.x installed

## Setup

Open terminal «K8S-PROD» with production cluster context.

### Configuration

```bash
# RDSK = Redis on Kubernetes
export RDSK_NAMESPACE=redis-prod
export RDSK_VERSION=7.0.5
export RDSK_PASSWORD=«generate-strong-password»
```

### Steps

1. Create namespace:
```bash
kubectl create namespace ${RDSK_NAMESPACE} && \
kubectl get namespace ${RDSK_NAMESPACE} && \
echo SUCCESS
```

2. Add Helm repository:
```bash
helm repo add bitnami https://charts.bitnami.com/bitnami && \
helm repo update && \
echo SUCCESS
```

[Continue with remaining steps...]
```

## Anti-patterns to Avoid

- Never use `$` prompt indicators in code blocks
- Don't create optional steps - make separate procedures instead  
- Avoid bash functions - write proper scripts for complex logic
- Don't implement retry logic - fail fast and clearly
- Never leave `$` variables undefined without guillemets
- Don't use fenced blocks for command output
- Avoid platform-agnostic procedures - be specific


