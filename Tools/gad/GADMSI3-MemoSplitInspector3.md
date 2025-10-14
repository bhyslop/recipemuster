# Cut Plan (three files, one HTML)

**Targets**

* `gadib_base.js` (**GADIB**) — transport + logging + hashing + factory shipper.
* `gadie_engine.js` (**GADIE**) — full 9-phase diff (DOM surgery, coalescing, deletion placement) → returns `{html, metrics, debugArtifacts}`.&#x20;
* `gadiu_user.js` (**GADIU**) — manifest/rails/UI state + event wiring + rendering into `#renderedPane`.&#x20;

We also have `gadic_cascade.css` in place of the prior `gadc.css` file.

---

# Surgical Moves

## 1) Extract **Base (GADIB)**

**Move out of HTML now:**

* WebSocket trace/shipper (`WebSocketTraceHandler` + `sendTrace/sendDebugOutput`) and the tiny `logger`. Wire `GADIB.logger.*` to ship over WS when connected.  &#x20;
* SHA-256/hash helper and any generic helpers (no DOM reads/writes).
* A tiny `factory.ship({type, content, metadata})` that wraps the debug sender.

**Acceptance checks**

* With only `manifest.json` available, WS connects *after* manifest success (not before). No fatal if WS absent.&#x20;
* Console still logs; WS sends when open.&#x20;

## 2) Isolate **UI (GADIU)**

**Keep here:**

* Manifest fetch, rails, popover, status ribbon, URL/hash state, swap button, selection logic. Render container is `#renderedPane`. &#x20;
* The only place that calls the engine: `const {html, metrics, debugArtifacts} = GADIE.diff(fromHtml, toHtml, opts)` then inject `html` into `#renderedPane`.&#x20;

**Expel from UI:**

* Any DOM mutation related to coalescing/placement/DFK routes—those live in **GADIE**.

**Acceptance checks**

* Rails populate purely from `manifest.commits` (positions H, -1, -2…), no hash text by default.&#x20;
* Defaults only if no hash params (From=-1, To=H).&#x20;
* `popstate` uses hash state only (no `?search`).&#x20;

## 3) Consolidate **Engine (GADIE)**

**Own end-to-end 9-phase algorithm**: classification → assembly → placement → serialize. Single file for blame and fixes; returns rendered HTML + metrics + debug artifacts (for Factory).&#x20;

**Dependency rule**

* GADIE can call **GADIB** (for `hash`, `logger`, `factory.ship`) but **never** touch UI elements or window history. Keep routes/DFK/DOM ops internal.

**Acceptance checks**

* No “approximate/fallback” DOM route recovery; fail loud with route in message.&#x20;
* Debug artifacts (phase snapshots) are *optional*: emitted via Base only if WS open.&#x20;

---

# Wiring (HTML)

* Load order: `gadib_base.js` → `gadie_engine.js` → `gadiu_user.js`.
* Keep ESM import for `diff-dom` in HTML; dispatch `diff-dom-ready` as now.&#x20;

---

# Contracts (don’t break these)

* **GADIE API:** `diff(fromHtml, toHtml, opts?) -> { html, metrics, debugArtifacts }`.&#x20;
* **GADIB API (minimal):**

  * `logger.d/p/e(msg)`
  * `hash(payload) -> "sha256:..."`
  * `factory.ship(type, content, metadata)` (no-ops if WS closed)

---

# Quality Gates (fast to verify)

1. **Single-file blame:** Engine bugs never require opening UI/Base; UI bugs never require Engine; transport bugs never require UI/Engine. (Design principle).&#x20;
2. **No Renderer:** Phases stay integrated—no fourth module.&#x20;
3. **WS after manifest:** connection attempt only after successful fetch.&#x20;
4. **Hash-only in Base; DOM surgery only in Engine; DOM injection only in UI.** (Memo responsibilities).  &#x20;

---

# Migration Steps (tight)

1. **Create `gadib_base.js`** and move WS/Logger/Factory/Hash. Replace in HTML with `GADIB.*` calls.
2. **Create `gadiu_user.js`**: lift constructor/DOM element discovery/event listeners/manifest/rails/url state; replace direct engine calls with `GADIE.diff`. &#x20;
3. **Create `gadie_engine.js`**: move every function that computes, mutates, or serializes the diffed DOM; export `diff`.
4. **HTML**: load three modules; delete inlined logger/WS/classes now living in Base/UI/Engine.
5. **Run gates** above; ship phase artifacts to Factory to validate snapshots match pre-split.

---

# Anti-patterns to nuke (if you see them)

* UI reaching into Engine internals (routes, node IDs).
* Engine doing `document.getElementById` or touching window history.
* New “Renderer” abstractions—don’t.


