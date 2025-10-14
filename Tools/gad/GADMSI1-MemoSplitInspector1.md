# GADI Architecture Memo

## Module Roles

### **GADIB – Base**

* **Responsibilities:**

    * Provides all shared utilities: logging, hashing, time, UUID, file/Factory shipping.
    * Owns and maintains **WebSocket connections**.
    * Manages **per-user contexts** keyed by `userKey`.
* **API (examples):**

    * `GADIB.registerUser(userKey, { sinks, level, factoryUrl })`
    * `GADIB.logger(userKey).d/i/w/e/p(msg, meta?)`
    * `GADIB.factory(userKey).ship({ type, name, content, sources })`
* **Key Point:** All transports (console, WS, file shipping) live here, not scattered.

---

### **GADIU – User (Controller)**

* **Responsibilities:**

    * Orchestrates the user’s experience.
    * Handles UI events (clicks, keypress, popstate) and Factory events.
    * Manages URL/hash state, manifest fetch, rail selection, swap, status ribbon.
    * Creates a `userKey`, registers it with GADIB, and passes it into engine calls.
    * Renders engine output into the DOM.

---

### **GADIE – Engine (Model)**

* **Responsibilities:**

    * Implements the 9‑phase diff pipeline.
    * Pure transform of `(fromHtml, toHtml)` → `{ html, metrics }`.
    * Calls into GADIB using `userKey` for logging and shipping (no direct WS/DOM).
* **Key Point:** Engine is algorithmic core but empowered by GADIB utilities.

---

### **GADIH – HTML (View)**

* Provides markup container and module bootstrap.
* Imports CSS, loads diff‑dom, and triggers `GADIU.bootstrap(...)`.
* Contains no business logic.

---

### **GADIC – CSS (View)**

* Provides all styling, semantic diff classes, and visual consistency.
* No JS or logic.

---

## Flow Overview

1. **HTML (GADIH)** loads, CSS attached, diff‑dom ready.
2. **User (GADIU)** bootstraps: creates `userKey`, registers with **Base (GADIB)**.
3. **User (GADIU)** handles selection/swaps, fetches HTML pair, calls **Engine (GADIE.diff)** with `(fromHtml, toHtml, userKey)`.
4. **Engine (GADIE)** runs pipeline, logs/ships via **Base (GADIB)**, returns `{ html, metrics }`.
5. **User (GADIU)** renders result into `#renderedPane`, updates UI/URL state, manages ribbons and popovers.

---

## Guardrails

* **Single Source of IO:** GADIB owns all logging and Factory/WebSocket shipping.
* **Per‑User Key:** Every call path includes `userKey` so multiple concurrent users are supported cleanly.
* **Separation of Concerns:**

    * GADIB = shared utilities + transports
    * GADIU = orchestration + user experience
    * GADIE = diff algorithms
    * GADIH = markup shell
    * GADIC = styling

