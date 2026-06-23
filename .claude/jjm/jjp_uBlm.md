## Paddock: rbk-19-aeo-basic-steps

### Heat intent
Make Recipe Bottle discoverable to AI answer engines (ChatGPT, Claude, Perplexity, Gemini)
so that a developer asking an LLM "what's a good tool for secure container builds / Google Cloud build provenance"
can surface RB in the synthesized answer.
This is the AI-era analogue of SEO; the discipline is variously named GEO (Generative Engine Optimization),
AEO (Answer Engine Optimization), or LLM SEO.
Scope is deliberately "basic steps" — the high-leverage, low-effort moves, not a full marketing program.

### Timing — deferred, take on AFTER MVP release
This heat is staged for post-MVP customer acquisition, not active work.
Stabled on nomination; mount and break into paces when the MVP ships and there is a public surface worth optimizing.

### The core shift (why this differs from classic SEO)
Old SEO optimized to RANK A LINK.
GEO/AEO optimizes to be CITED INSIDE the AI's synthesized answer — citation is the unit of visibility, not the blue link.
A page can be invisible to AI even while ranking on Google, and vice versa.

### Techniques found (research dossier — durable, source-cited below)
- Be machine-readable: AI crawlers read raw HTML and do NOT execute JavaScript; client-side-only content is invisible.
  Allow AI crawlers in robots.txt; consider an `llms.txt` file as an AI-specific sitemap.
- Answer-first writing: put the core answer in the first 40-60 words under a question-shaped heading
  ("What is...", "How do I...", "X vs Y"); LLMs lift these blocks directly.
- Fact density + citations: pages with statistics, named sources, and quotes get pulled far more often
  (one widely-cited study claims up to a 40% lift for otherwise low-ranked pages).
- Structured data / schema markup (FAQPage, HowTo, Author/Person) turns a page into a ready-to-cite answer block.
- Third-party presence beats your own domain: brands reported ~6.5x more likely to be cited via third-party sources
  than their own site; presence on 4+ platforms ~2.8x more likely to surface.
  For a dev tool that means GitHub, Hacker News, Stack Overflow, Reddit, comparison/listicle posts, Wikipedia.
- Freshness: recently-updated content appears several times more often in AI answers.
- Video: YouTube reported as the single most-cited source in AI search; spoken transcript gets indexed.

### RB-specific reading (where the leverage actually is for THIS product)
RB is a niche, technical security/infra product — the consumer-brand playbook does not map cleanly.
Highest-leverage levers for RB:
- The "third-party citation" lever and "be where developers' questions get answered" lever dominate here.
- A clear public docs/glossary surface that an LLM can quote — the GitHub Pages site
  (https://scaleinv.github.io/recipebottle) is the natural home; the README-as-broadside instinct already in the
  project is the right shape.
- Open question to resolve at mount: who is the buyer — security-conscious dev teams, or individual developers?
  The answer steers which watering holes and which comparison framings matter.

### What done looks like (sketch — refine at mount)
A small, concrete checklist executed against the public surface:
robots.txt/llms.txt posture confirmed; key pages answer-first and server-rendered;
schema markup on the docs/glossary; a handful of third-party citations seeded;
and a way to spot-check whether the major engines now mention RB when prompted in-domain.

### Sources (verbatim, captured 2026-06-23)
- ChatGPT SEO & GEO 2026: 12 Tips To Get Cited In AI Answers — Yotpo: https://www.yotpo.com/blog/chatgpt-seo-geo-tips/
- Generative Engine Optimization (GEO): The 2026 Guide — LLMrefs: https://llmrefs.com/generative-engine-optimization
- 10-step framework for generative engine optimization — Profound: https://www.tryprofound.com/resources/articles/generative-engine-optimization-geo-guide-2025
- AI SEO/GEO/AEO: How to Get Shown in LLMs in 2026 — Edward Sturm: https://edwardsturm.com/articles/ai-seo-geo-aeo-get-shown-llms-2026/
- Answer Engine Optimization (AEO): 6 Best Practices for 2026 — Position Digital: https://www.position.digital/blog/answer-engine-optimization-best-practices/
- LLM SEO: The Complete Guide — LLMrefs: https://llmrefs.com/llm-seo
- Show Up in AI Search with Answer Engine Optimization — HubSpot: https://www.hubspot.com/products/marketing/aeo-guide