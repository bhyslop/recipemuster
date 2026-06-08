# Meta Specification: Writing Durable IT Procedures

This document defines patterns, examples, and vocabulary for writing **long-lived, rarely-run IT procedures** intended for technically competent Executors who may be new to this specific task.  
It is primarily for use by an **Editor**, who may work with an LLM to prepare or maintain the procedures.

---

## 1. Roles

- **Meta Spec** – This document. Sets patterns and vocabulary for all procedures in scope.
- **Editor** – Maintains and refines procedures, ensures they match this spec, and adapts to context.
- **Executor** – Runs the procedure. May be unfamiliar with this context but is technically competent.

---

## 2. Core Principles

1. **Minimize Decisions for the Executor**  
   - Each procedure should have exactly one clear path. No alternates, no “choose one” steps.  
   - Any complexity or branching belongs in separate procedures.

2. **Be Terse, but Precise**  
   - Remove extra words that don’t improve clarity.  
   - Keep UI instructions intent-focused but clear enough to follow even if the interface changes.

3. **Durability Over Time**  
   - Avoid brittle details like exact button colors or transient UI layouts.  
   - Keep context and terminology stable, even if cloud provider or tool versions change.

---

## 3. Context and Placeholders

### 3.1 Shell Context Naming
- Always name the terminal/session context in **surrounding text** using guillemets `«…»`.  
  Examples: `«local-shell»`, `«vm-shell»`, `«cloud-shell»`.
- In **fenced code blocks of the final procedure**, guillemets are replaced with **concrete names**.

### 3.2 Environment Variable Placeholders
- For durable configuration values, define variables with:
  - **Unique all-caps prefix** for this procedure or procedure family.
  - Ludicrous placeholder value that forces customization.
  - Example (meta-spec form):  
    ```bash
    export «MECS»_REGION=CHANGE_ME_NOW
    ```
- In the final procedure, replace `«MECS»` with the concrete prefix chosen for the context.

---

## 4. Structure Patterns

Procedures should generally follow this sequence:

1. **Introductory context** (1–3 sentences)  
   - Purpose, scope, and assumptions. No deep background.
2. **Steps**  
   - Numbered, terse, and unambiguous.
   - For CLI: fenced `bash`/`powershell`/etc. blocks with pure commands only.
   - For Web UI: intent description plus arrow-chain navigation hints.
   - For Installers/Wizards: each screen documented, defaults noted.
3. **Verification or confirmation** (only where context demands it)  
   - Optional, based on fragility or risk of the step.
4. **Completion statement**  
   - Clear signal that the procedure is finished.

---

## 5. Command Block Patterns

### CLI Example (meta-spec form)
Descriptive text:

> In `«local-shell»`, set the target region.

Copy/paste block:
```bash
export «MECS»_REGION=CHANGE_ME_NOW
````

---

### Compound Command Example

If multiple commands are needed, join with `&&` and `\`:

```bash
do_thing --step1 foo && \
do_thing --step2 bar && \
echo SUCCESS
```

---

## 6. Web UI Patterns

* Describe **what** to do, not just **how** to click.
* Use **arrow chains** for navigation:

  ```
  Dashboard ? Compute (VM List) ? my-vm ? Edit
  ```
* When trusting a default, include its label:

  > Accept default *"Automatic scaling"* (currently enabled).

---

## 7. Controlled Installation Patterns

Each screen or modal is a step:

```
Step: "Configure Components" modal
- [x] Enable GPU Acceleration  (default: unchecked)
- [ ] Enable Experimental Mode (default: checked)
```

User-entered values go in backticks:

> Enter username `admin_test`

---

## 8. Patterns and Anti-Patterns

**Patterns**:

* One clear path.
* Context names and env vars are unambiguous.
* Fenced blocks contain only what is to be pasted.
* Defaults are labeled for future verification.

**Anti-Patterns**:

* “Click Next through defaults” with no record of what defaults were.
* Ambiguous options (“choose one of these”).
* Leaving guillemets in fenced blocks of the final procedure.
* Mixing descriptive text and commands in same fenced block.

---

## 9. Editor Checklist

* [ ] All contexts named in guillemets in descriptive text.
* [ ] No guillemets in fenced command blocks.
* [ ] All durable values set as env vars with unique prefixes.
* [ ] Every wizard step documented, even if accepting defaults.
* [ ] Commands are copy/paste safe (no inline comments).
* [ ] Compound commands use `&&` and end with `echo SUCCESS` if multi-line.
* [ ] Web UI steps record default labels where relevant.
* [ ] Procedure has a clear completion statement.

---

*End of Meta Specification*

