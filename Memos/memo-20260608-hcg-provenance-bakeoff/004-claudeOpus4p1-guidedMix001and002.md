# Meta Specification: Durable IT Procedures

## Purpose for Digital Mind

You are interpreting this specification to help an Editor create and maintain procedures that:
- Execute infrequently (quarterly, yearly, during incidents)
- Are run by smart engineers unfamiliar with the specific domain
- Remain valid despite environment changes
- Minimize cognitive load while maintaining precision

## Core Awareness

**Rich Health**: A procedure achieves rich health through minimalism. Every word either prevents failure or provides essential context. Trust that Executors are intelligent; don't patronize with redundancy.

**Durability**: These procedures survive time through intent-focus, not brittle UI details. When environments change, the procedure should guide repair rather than break entirely.

## The Guillemet Boundary

The `«...»` notation separates meta-layer from execution-layer:

### In descriptive text:
```markdown
Open a terminal «K8S-PROD» with production cluster context
```

### In execution blocks:
```bash
export RDSK_NAMESPACE=redis-prod
export RDSK_PASSWORD=«generate-strong-password»
```

Guillemets in descriptive text name contexts. Guillemets in code blocks mark values requiring substitution.

## Environment Configuration Pattern

Group configuration early with meaningful prefixes:

```bash
# RDSK = Redis on Kubernetes
export RDSK_NAMESPACE=redis-prod  
export RDSK_VERSION=7.0.5
export RDSK_PASSWORD=«generate-strong-password»
```

Prefixes should be 3-5 characters, memorable, and unique to this procedure.

## Command Patterns

### Baseline Execution
Simple commands need no verification:
```bash
kubectl get pods -n ${RDSK_NAMESPACE}
```

### Critical Operations
Chain with `&&` when failure must propagate:
```bash
aws s3 cp critical.tar.gz s3://${BUCKET}/ && \
aws s3api put-object-acl \
  --bucket ${BUCKET} \
  --key critical.tar.gz \
  --acl public-read && \
echo SUCCESS
```

### Dangerous Operations
Mark clearly when irreversible:
```bash
# ⚠️ WARNING: Permanently deletes production database
dropdb ${PGDS_DATABASE}
```

## Verification Philosophy

Include verification only when:
- Failure is silent but consequential
- Success criteria are non-obvious
- The next step depends on specific output

Avoid reflexive "echo SUCCESS" when the command's output is self-evident.

## Branching Guidance

When you detect branching emerging:
1. **Prune** if the branch handles an edge case
2. **Separate** into distinct procedures if both paths are common
3. **Document** the single blessed path if organizational policy exists

Never present "choose one" within a procedure.

## Web UI Patterns

Focus on intent and data, flex on navigation:

```markdown
1. Navigate to: AWS Console → EC2 → Security Groups
2. Create security group:
   - Name: `production-web-sg`
   - VPC: Select production VPC (not default)
3. Add inbound rule: HTTPS from 0.0.0.0/0
```

Record defaults only when they matter for troubleshooting.

## Duration Awareness

Note time for operations over 30 seconds:
```bash
# Build container (typically 3-5 minutes)
docker build -t ${IMG}:${VER} .
```

## Output Documentation

For simple validation:
```bash
terraform plan
# Expect: "Plan: 3 to add, 0 to change, 0 to destroy"
```

For complex output, describe characteristics:
"Verify three security groups appear in the list"

## Secret Management

Always mark secrets for substitution:
```bash
export GHPR_PAT=«github-personal-access-token»
# Get from: Settings → Developer → Personal Access Tokens
```

## Digital Mind Heuristics

When reviewing a procedure:

1. **Every line must earn its presence** - Does this prevent failure or provide essential context?
2. **Failure modes over success theater** - Focus on what breaks, not celebrations
3. **Intent over mechanics** - "Create security group" not "Click blue button"
4. **Test the unhappy path mentally** - What happens when this step fails?

When uncertain, prefer minimalism. The Editor will request additions if needed.

## Patterns and Anti-patterns

**Embrace:**
- Single blessed path
- Meaningful variable prefixes
- Intent-focused UI instructions
- Explicit dangerous operations
- Natural success indicators

**Avoid:**
- Optional steps
- Undefined variables without guillemets
- Brittle UI details
- Excessive verification
- Success theater
- Prompt indicators ($) in code blocks

## Example Transformation

Meta-spec guidance:
> "Configure Redis with appropriate memory for production load"

Poor procedure:
```bash
$ redis-cli CONFIG SET maxmemory 2gb
OK
$ echo "Success!"
```

Rich procedure:
```bash
# Configure Redis memory limit based on node capacity
redis-cli CONFIG SET maxmemory ${RDSK_MEMORY_LIMIT}
redis-cli CONFIG GET maxmemory  # Should show your configured value
```

## For Edge Cases

When this specification doesn't address a situation:
1. Consider similar patterns in existing procedures
2. Prefer minimalism over completeness
3. Surface genuine ambiguity to the Editor

Remember: You're helping curate procedures toward rich health, not enforcing rigid rules. Trust your understanding of what makes procedures durable and executable under stress.
