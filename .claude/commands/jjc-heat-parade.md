---
argument-hint: [firemark]
description: Display comprehensive heat status
---

Show full status of a heat including all paces and their states.

Arguments: $ARGUMENTS (optional Firemark; uses current context if omitted)

## Prerequisites

Requires gallops JSON at `.claude/jjm/jjg_gallops.json`.

## Step 1: Identify target heat

**If $ARGUMENTS contains a Firemark (e.g., `AB` or `₣AB`):**
- Use that Firemark directly

**If $ARGUMENTS is empty:**
- Use FIREMARK from current context
- If no context: Report "No heat context. Use `/jjc-heat-groom` or `/jjc-heat-mount` to establish one." and stop.

## Step 2: Get parade data

Run:
```bash
vvx jjx_parade <FIREMARK>
```

Parse JSON output:
```json
{
  "heat_silks": "...",
  "heat_created": "YYMMDD",
  "heat_status": "current|retired",
  "paddock_file": ".claude/jjm/jjp_XX.md",
  "paddock_content": "...",
  "paces": [
    {
      "coronet": "₢XXXXX",
      "silks": "...",
      "state": "rough|primed|complete|abandoned",
      "tack_text": "...",
      "tack_direction": "..."
    }
  ]
}
```

## Step 3: Display formatted status

### Heat: {heat_silks}
**Firemark:** ₣{firemark} | **Created:** {heat_created} | **Status:** {heat_status}

### Paddock
{Brief summary from paddock_content - first paragraph or key points}

### Paces

For each pace in order:

**{index}. {silks}** `{coronet}` — {state}
{For rough/primed: show tack_text summary}
{For primed: note direction}
{For complete/abandoned: just show state}

### Summary
- Total paces: N
- Complete: X
- Remaining: Y (Z rough, W primed)

## Step 4: Done

Display only. No suggestions or next actions.
