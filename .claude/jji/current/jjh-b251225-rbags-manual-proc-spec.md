# Heat: RBAGS Manual Procedure Specification Alignment

## Context

RBAGS (lenses/rbw-RBAGS-AdminGoogleSpec.adoc) specifies Recipe Bottle's Google Cloud operations. Implementation exists in Tools/rbw/ but spec sections are incomplete or misaligned.

Goal: Complete and align specification for the Director-triggered remote build flow.

### Reference Files

**Specification:**
- `lenses/rbw-RBAGS-AdminGoogleSpec.adoc` - master spec

**Implementation:**
- `Tools/rbw/rbgm_ManualProcedures.sh` - payor establish/refresh display
- `Tools/rbw/rbgg_Governor.sh` - director/retriever creation
- `Tools/rbw/rbf_Foundry.sh` - trigger build, image delete
- `Tools/rbw/rbgo_OAuth.sh` - JWT exchange
- `Tools/rbw/rbgu_Utility.sh` - RBRA load, API enable

**Regime Configuration:**
- `rbrr_RecipeBottleRegimeRepo.sh` - master regime
- `rbrp.env` - payor config

**Legacy (to delete):**
- `Tools/rbw/rbmp_ManualProcedures-PCG005.md`

## Done

## Current

## Remaining
- Define specification completeness criteria for RBAGS operations
- Delete legacy rbmp_ManualProcedures-PCG005.md
- Specify rbtgo_director_create (expand stub to full procedure)
- Specify rbtgo_trigger_build (document build submission flow)
- Specify rbtgo_image_delete (document deletion flow)
- Normalize and validate RBAGS

## Itches
