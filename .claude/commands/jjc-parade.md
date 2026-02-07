---
argument-hint: [target or full]
description: Display heat or pace info
---

Display heat or pace information with smart defaults.

Arguments: $ARGUMENTS (optional target or keyword)

## Prerequisites

## Step 1: Parse arguments

**If $ARGUMENTS is empty:**
- Check context for FIREMARK
- If FIREMARK exists: use it as target (heat list mode)
- Otherwise: Error "No heat context. Use `/jjc-heat-mount` first or provide a target."

**If $ARGUMENTS is "detail":**
- Check context for FIREMARK
- If FIREMARK exists: use it as target with --detail flag (heat detail mode)
- Otherwise: Error "No heat context. Use `/jjc-heat-mount` first or provide a firemark."

**If $ARGUMENTS starts with "detail " (followed by identifier):**
- Extract identifier after "detail "
- Parse as firemark (heat detail mode with explicit target)
- Use --detail flag

**If $ARGUMENTS is a 2-char identifier (e.g., `AB` or `₣AB`):**
- Parse as firemark (heat list mode)

**If $ARGUMENTS is a 5-char identifier (e.g., `ABCDE` or `₢ABCDE`):**
- Parse as coronet (pace detail mode)

**Otherwise:**
- Error "Invalid argument. Use: empty (context), 'detail', 'detail <firemark>', <firemark>, or <coronet>"

## Step 2: Run parade

Based on determined mode:

**Heat list (firemark, no --detail):**
```bash
./tt/vvw-r.RunVVX.sh jjx_show <FIREMARK>
```

**Heat detail (firemark with --detail):**
```bash
./tt/vvw-r.RunVVX.sh jjx_show <FIREMARK> --detail
```

**Pace detail (coronet):**
```bash
./tt/vvw-r.RunVVX.sh jjx_show <CORONET>
```

## Step 3: Display output

Echo the vvx output directly in your response text. Do not use code blocks or markdown tables - just output the plain text lines so they display compactly.

## Examples

- `/jjc-parade` — List paces in current heat (requires context)
- `/jjc-parade detail` — Show detailed current heat with paddock (requires context)
- `/jjc-parade AB` — List paces in heat ₣AB
- `/jjc-parade detail AB` — Show detailed heat ₣AB with paddock
- `/jjc-parade ABCDE` — Show detail for pace ₢ABCDE
