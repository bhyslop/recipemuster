---
description: Apply whitespace normalization to concept model documents
argument-hint: [file-path | all]
---

Invoke the cmsa-normalizer subagent to apply MCM whitespace normalization.

**Target:** $ARGUMENTS

Use the Task tool with:
- `subagent_type`: "cmsa-normalizer"
- `prompt`: Include the target from arguments and configuration below

**Configuration to pass:**
- Lenses directory: lenses/
- Kit directory: Tools/cmk/
- Kit path: Tools/cmk/concept-model-kit.md

**File Resolution** (include in prompt):
When a filename is provided (not "all"):
1. If it's a full path that exists → use it
2. If it matches a file in lenses directory → use that
3. If it matches a file in kit directory → use that
4. Common aliases: "MCM" → mcm-MCM-MetaConceptModel.adoc, "AXL" → axl-AXLA-Lexicon.adoc
