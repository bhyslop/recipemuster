# Memo: Git hosting landscape for proprietary source-code custody

**Date:** 2026-05-14
**Subject:** Strategic environment, commercial / legal / safety considerations, and an evaluation framework for choosing a git host for proprietary source code. Operational recipes (deployment configs, backup commands, hardening checklists) are out of scope by design — once the considerations are clear, implementation is straightforward.

---

## Strategic environment

### The evaluation question

For a single developer holding proprietary source code, host choice is not "which forge is safest?" It is:

> **Which combination of legal contract, business incentives, technical custody model, and exit path leaves the fewest parties able or motivated to misuse the code?**

Five questions any candidate host must answer:

1. Can the provider legally or contractually use, share, or disclose my code?
2. Can its employees or systems technically access my code?
3. Can I leave quickly with complete history?
4. Will the provider still exist, and still be in the same business, in ten years?
5. Does using this host create client, employer, export-control, or insurance problems?

### The structural truth about SaaS git

**No mainstream SaaS git host offers zero-knowledge / customer-held-key repository storage.** Every operator can read every repo on every standard tier. BYOK / CMEK exists only on enterprise / dedicated tiers, and even then usually applies to infrastructure-layer encryption rather than git object readability.

This is a market fact, not a configuration detail. The consequence: provider choice is mostly a choice among legal jurisdiction, business durability, platform drift, account-control posture, and operator trust — *not* a choice among cryptographic custody models. Cryptographic custody is a separate layer the developer can apply on top; it appears here only as a category of consideration.

### Where the major providers are heading (2024-2026)

The trajectory across major commercial providers is consistent and worth naming explicitly.

| Provider | Recent move | Direction signal |
|---|---|---|
| **GitHub** | April 2026 ToS expanded AI-training rights over private-repo content provided as Input to AI features; opt-out for Free/Pro/Pro+, not Enterprise | Expanding data-utilization rights; Enterprise carve-outs reinforce two-tier customer treatment |
| **Bitbucket / Atlassian** | August 17, 2026 clause uses metadata + in-app data for cross-customer product/AI improvement; metadata opt-out is Enterprise-only. Atlassian Server EOL March 2024; Data Center products forced to Cloud by March 2029 | AI-data utilization + aggressive cloud-only consolidation |
| **GitLab** | Explicit "no inputs or outputs used to train GitLab models"; Duo AI opt-in | Counter-positioning on AI data use; transparent legal handbook; cleanest mainstream SaaS posture |
| **Gitee** (China) | May 2022 enforced manual review of all public repos | State alignment via MIIT designation |
| **NotABug.org** | "Down due to AI scrapers" state in 2026 | Smaller hosts being overwhelmed by indirect AI-induced load |

The commercial-SaaS direction is more data utilization, not less. The consolidation pressure narrows "buy without AI" options each year. A host that changes data-use defaults is telling you its business incentives shifted — that signal is more decisive than headline pricing.

### Regulatory wave hitting 2026-2027

| Date | Event | Implication for code custody |
|---|---|---|
| **July 10, 2023** | EU-US DPF entered into force | Current legal pathway for transatlantic transfers including code |
| **September 2025** | EU General Court upheld DPF against first challenge | DPF survives, but NOYB signaled broader CJEU challenge ("Schrems III") |
| **September 11, 2026** | EU Cyber Resilience Act reporting obligations begin | Mandatory vulnerability / incident reporting for "products with digital elements" |
| **2026 (rolling)** | EU member states transposing NIS2 | Supply-chain security obligations propagate to software vendors |
| **December 11, 2027** | Full CRA obligations apply | Lifecycle cybersecurity requirements for software products |

For a solo developer the CRA / NIS2 implication is not "choose an EU host." It is:

> Can I produce evidence of secure development, vulnerability handling, provenance, access control, and incident response if my software becomes regulated?

Host posture on those evidence trails matters more than host location.

### ITAR adjacency — a separate universe

If actual ITAR-controlled content ever enters scope, the entire economic and operational model changes. Cost components:

| Item | Magnitude |
|---|---|
| DDTC registration | $3-4K/yr |
| Export-control counsel (year 1) | $10-50K |
| CMMC Level 2 readiness + assessment | $70-150K (year 1) |
| GitLab Dedicated for Government, GHES on GovCloud | $50-200K/yr quote-dependent |
| Continuous compliance | $20-50K/yr |
| **3-year TCO** | **~$135K-470K** |

ITAR is structurally a "have an ITAR-paying contract" path — speculative compliance is uneconomic at solo-developer scale.

**The "ITAR-ready learning" posture**: use GitLab Self-Managed CE or GitHub Enterprise Server on commercial AWS — same binaries that elevate to GovCloud — and layer NIST 800-171 disciplines (SSO + hardware MFA, audit log streaming, IaC, SSP drafting, POA&M, restore drills) regardless of certification status. ~$500-1,500/yr. The disciplines transfer when the day comes; the compliance program is bought separately.

### Seven landscape conclusions

1. **GitHub** is operationally strongest but strategically AI / platform-driven.
2. **GitLab** is the cleanest mainstream SaaS posture today — explicit no-training stance plus a self-managed exit path.
3. **Bitbucket / Atlassian** is commercially durable but has poor drift signals for a minimalist IP-custody use case.
4. **Codeberg** and **SourceHut** are values-aligned but not enterprise-custody substitutes; Codeberg is explicitly mission-mismatched for proprietary private repos.
5. **Forgejo / Gitea self-hosting** gives control, not automatic safety.
6. **Managed Gitea / Forgejo** and **Radicle** deserve consideration as alternate custody models — each shifts risk rather than eliminating it.
7. **No mainstream SaaS git host should be assumed zero-knowledge.**

---

## Provider landscape snapshot

| Provider / category | Commercial signal | Legal signal | Security / compliance signal | Landscape note |
|---|---|---|---|---|
| **GitHub** | Very durable; Microsoft-owned; broad free → enterprise ladder | US / Microsoft legal nexus; strong transparency; AI policy requires close reading by account type | Strong compliance artifacts; excellent enterprise controls; historical private-repo (2016) and SSH-host-key (2023) incidents disclosed | Best ecosystem; strongest strategic drift toward AI / dev platform |
| **GitLab.com / GitLab Dedicated** | Public company; paid tiers; self-managed exit path | US company; strong no-AI-training messaging; transparent legal handbook | SOC 2 Type II / ISO 27001 / 27017 / 27018; Dedicated for Government FedRAMP Moderate (May 2025); 2017 data-loss postmortem unusually transparent | Best mainstream posture if SaaS is required |
| **Bitbucket Cloud / Atlassian** | Large corporate durability; Atlassian cloud consolidation | Australia / US global footprint; AI / data-policy drift | Trust center exists; Government Cloud FedRAMP Moderate (~25% premium over Cloud Enterprise); Bitbucket-specific audit coverage less prominent | Strategic direction is Atlassian Cloud + Rovo / AI, not minimalist git custody |
| **Codeberg** | Nonprofit, donation-funded, community | EU / Germany nonprofit; mission favors FOSS, not proprietary custody | Forgejo-based; no formal enterprise assurance | Values-aligned for FOSS; their own FAQ steers proprietary private repos elsewhere |
| **SourceHut** | Paid subscription, independent, no ads / tracking | Netherlands presence; strong anti-data-sale values; formal transparency thinner | Minimalist, privately owned hardware, no AI features; few compliance artifacts | Ethically strong, assurance-light |
| **Self-hosted Forgejo / Gitea** | Lowest license cost; highest admin burden | Jurisdiction depends on where / how hosted | Posture is yours; Gitea Enterprise has SOC 2 / SOC 3 claims (the project itself does not confer them on your deployment) | Best control, worst if neglected |
| **Managed Gitea / Forgejo** | Mid-cost niche | Depends heavily on provider contract and jurisdiction | Varies widely; ask for SOC / ISO, access policy, backups, subprocessors | Underexplored middle ground |
| **Radicle** | Emerging / nontraditional | Less central operator exposure; more peer / key-management questions | Cryptographic identities; not standard enterprise compliance | Interesting for resilience and independence; not a private-host substitute |
| **Gitee / Yandex / other state-aligned** | State-aligned by default | Subject to domestic state demands; data localization | Compelled disclosure structurally available to home state | Unacceptable for crown-jewel commercial code |

---

## A. Commercial considerations

### Pricing tier shape

Order-of-magnitude only.

| Segment | Typical providers | Cost shape (per dev) | What it usually buys | Hidden catch |
|---|---|---|---|---|
| Free individual SaaS | GitHub Free, GitLab Free, Bitbucket Free, Codeberg | $0 | Private repos, basic git, limited controls | Consumer ToS; least leverage over data terms |
| Small-team commercial SaaS | GitHub Team, GitLab Premium, Bitbucket Standard / Premium, Gitea Cloud | ~$4-30/user/month | Branch protection, support, some audit / security | "One developer" often still needs org / enterprise tier for the controls that matter |
| Enterprise SaaS | GitHub Enterprise Cloud, GitLab Ultimate, Atlassian Cloud Enterprise | ~$20-100+/user/month | SAML / SSO, advanced audit, data residency, compliance artifacts | Seat minimums, annual contracts, support gating, add-on security products |
| Dedicated / single-tenant | GitLab Dedicated, Atlassian Isolated Cloud, Gitea Enterprise hosted | Quote-based; mid-four to low-six figures / yr | Isolation, compliance posture, custom region, stronger support | Usually uneconomic for a single developer |
| Government / FedRAMP / ITAR-ready | GitHub Enterprise Cloud FedRAMP, GitLab Dedicated for Government, Atlassian Government Cloud | Enterprise + surcharge / quote | FedRAMP packages, controlled support, region / personnel constraints | May still not be ITAR by default; export-controlled code triggers further restrictions |
| Self-hosted open source | Forgejo, Gitea CE, bare git | $0 license + hardware / admin time | Control and portability | You buy your own reliability, patching, backups, legal-process handling |
| Managed open-source forge | Managed Gitea / Forgejo hosts, small MSPs | ~$10-100+/month | Private instance without full admin burden | Inherit both SaaS operator trust *and* self-host operational ambiguity |

### Funding model as longevity predictor

| Model | Examples | Longevity signal | Risk |
|---|---|---|---|
| Corporate-owned platform | GitHub / Microsoft, Bitbucket / Atlassian | Deep balance sheet; strong operational durability | Strategic drift, AI / data-reuse pressure, product consolidation |
| Public SaaS company | GitLab | Commercially durable; transparent reporting; self-managed exit path | Pricing pressure; features migrate up-tier |
| Nonprofit / donation / community | Codeberg / Forgejo ecosystem | Mission alignment, low extractive pressure | Runway, abuse handling, commercial-fit limits |
| Paid-subscription independent | SourceHut | Incentive alignment: users pay, not ads / data | Smaller operator risk; thinner enterprise assurance |
| Venture-backed AI-pivoting platform | Various | Can grow quickly | "Your code becomes model fuel"; AI / enterprise bundling drift |

### Strategic-drift signals to watch

| Signal | Why it matters |
|---|---|
| AI policy changes | A host that changes data-use defaults is telling you its incentives shifted. GitHub's March 2026 change separates Enterprise / org-account treatment from individual / Copilot data treatment — implying two customer tiers under different rules. |
| Cloud-only migrations / product retirements | Atlassian ended Server already; Data Center forced to Cloud by March 28, 2029 (extended maintenance by exception only). Strong lock-in / drift signal. |
| Compliance SKU proliferation | "The safe version" may exist only in expensive Enterprise / Gov / Dedicated tiers |
| Pricing reorganization | Seat minimums, storage add-ons, advanced-security add-ons, support gating often matter more than headline price |
| Executive turnover / acquisition rumors | Less relevant for Microsoft / Atlassian; more relevant for small independent hosts |
| Community-to-commercial fork tensions | Gitea / Forgejo split illustrates governance drift risk. Forgejo now positions as community / nonprofit free software; Gitea has commercial Cloud / Enterprise (SOC 2 / SOC 3 claims) |

### Portability and lock-in

| Portable | Semi-portable | Usually non-portable |
|---|---|---|
| Git commit graph, branches, tags, submodules | Pull requests, code-review comments, issues, releases, protected-branch rules | Actions / CI pipelines, package registries, security alerts, secret-scanning history, audit logs, SSO policies, enterprise identity configuration |

The single-developer custody view: **git itself is portable; platform state is not.** The more you use CI, packages, issues, PR review, branch rules, code scanning, AI review, and deployment tokens, the more the "git host" becomes a platform-custody problem with no clean exit.

### Hidden costs people miss

- **Minimum seat / annual contract** — enterprise controls often unavailable to a solo dev without org-level billing
- **Support-tier dependency** — incident response, legal escalations, compliance documents gated by paid plans
- **Audit-log retention** — "has audit logs" may mean short retention or enterprise-only export
- **Data-residency add-ons** — region choice may cost extra or omit telemetry / support / AI / billing data
- **Advanced-security add-ons** — secret scanning, code scanning, dependency review, push protection often separately priced
- **Egress / storage** — less about clones, more about LFS, packages, artifacts, CI caches
- **Identity-provider cost** — SAML / SSO usually requires both provider enterprise tier and IdP subscription
- **Legal review cost** — client contracts, DPAs, BAAs, export-control clauses, insurance riders can dwarf hosting cost

---

## B. Legal considerations

### Jurisdiction in practice (not just acronyms)

| Regime | Practical meaning for source-code custody |
|---|---|
| **GDPR / EU data protection** | Protects personal data, not trade secrets as such. Source code can carry personal data in comments, fixtures, logs, datasets, commit metadata, or secrets. EU hosts may simplify EU-client DPAs, but GDPR does not make code confidential from the processor's authorized staff. |
| **BDSG (German overlay)** | Stronger employee / personal-data discipline layered on GDPR. Not a magic IP shield. |
| **CCPA / CPRA** | California consumer-privacy rights. Relevant if account / user metadata contains personal info; not a source-code confidentiality regime. |
| **EU-US Data Privacy Framework** | Current EU-to-US transfer mechanism (July 10, 2023). Upheld by EU General Court September 2025; further legal-challenge risk remains. |
| **Standard Contractual Clauses (SCCs) + Schrems II analysis** | Contractual basis for transfers to non-adequate countries; still requires assessing foreign surveillance and supplementary measures. New SCCs adopted June 2021. |
| **CLOUD Act** | US providers can be compelled under US law to produce covered data even when stored outside the US. Data residency does not eliminate US legal reach. |
| **FISA 702 / NSLs** | US national-security process can compel US providers for foreign-intelligence collection; public reporting is delayed or aggregated. |
| **UK Investigatory Powers Act** | Technical Capability Notices and related can impose engineered-access obligations, often gag-ordered. |
| **MLATs** | Slower treaty-based legal-assistance routes between states; "foreign host" does not mean "unreachable." |
| **Russia / China / state-aligned** | The issue is not just formal law — it is state influence over domestic providers, weak transparency, and practical inability to resist national-security demands. |
| **Israel** | Strong tech sector, security-law environment; not automatically bad, but defense / intelligence proximity matters if the adversary model includes state-aligned access. |

**Critical consideration**: jurisdiction matters most when the provider's *parent company* is incorporated there, not just where servers physically sit. The CLOUD Act follows incorporation, not the data center.

### EU-US data transfers (DPF status, May 2026)

- DPF adopted July 2023 via US Executive Order 14086
- First judicial challenge dismissed by EU General Court September 2025
- NOYB (Max Schrems) signaled intent to bring a broader CJEU challenge — a "Schrems III" is anticipated but not filed as of this writing
- Practical effect: data flow from EU to DPF-certified US companies is currently legal under EU law; this could change on 6-12 months' notice if CJEU strikes down DPF

**Consideration**: relying on DPF stability for code-custody plan is the same bet GDPR-regulated companies are making for their entire EU operation. Not crazy, but not safe.

### Compelled-disclosure regimes

| Regime | What it can compel | Provider gagged? |
|---|---|---|
| US National Security Letter | Subscriber records, transactional data | Yes |
| US FISA 702 directive | Communications content for foreign targets | Yes |
| US CLOUD Act warrant | Content regardless of data location | No (normal warrant) |
| UK IPA technical capability notice | Engineered capability to provide access | Yes |
| EU MLAT (state-to-state) | Routed via treaty channels | Variable |
| China Cybersecurity Law | Direct access to data and infrastructure | Yes |
| Russia FSB SORM | Real-time communications access | Yes |

| Provider nexus | Practical risk |
|---|---|
| US provider or US parent | CLOUD Act + subpoenas / warrants + NSLs + FISA 702 + export / sanctions demands |
| EU provider | EU / local law + MLAT + GDPR processor obligations + national-security exceptions |
| UK provider | IPA notices + UK legal process |
| Small nonprofit / community host | Less legal staff to push back; may publish fewer statistics |
| Self-hosted on VPS | VPS provider can be compelled for infrastructure-level data |
| Self-hosted on owned hardware | Legal pressure hits you directly; fewer intermediaries can silently disclose |

### ToS surface area beyond AI training

Clauses worth reviewing on any candidate host:

| Clause | Why it matters |
|---|---|
| Ownership preservation | Most providers say you retain ownership — necessary but not sufficient |
| License to host / process | Watch for "sublicensable," "perpetual," "irrevocable," or "for improvement of services" |
| Affiliate sharing | Microsoft / GitHub / Atlassian-style structures route processing through affiliates |
| AI / product-improvement clauses | Explicit no-training is materially better than opt-out default. GitLab: "no inputs or outputs from Duo used to train GitLab models; vendors prohibited from using prompts/outputs associated with GitLab customer IDs for their own purposes." |
| Mandatory arbitration / class-action waiver | Reduces leverage on systemic data-use or breach issues |
| Unilateral terms changes | Consumer / small-business tiers often permit notice-and-continue-use changes |
| Indemnity asymmetry | You may indemnify host for your content while host liability for disclosure is capped |
| Suspension rights | Abuse, sanctions, DMCA, export-control, payment, or "risk to service" clauses can freeze access |
| Support access to content | Support workflows may authorize employee access unless strictly gated |

### Transparency-report quality

| Provider | Signal |
|---|---|
| GitHub | Strong public Transparency Center: government takedowns, user-info requests, DMCA data, structured repository. Notifies affected users when legally permitted. |
| Microsoft (GitHub parent) | Semiannual law-enforcement reports; H2 2025: 5,587 US legal demands for consumer data; 115 warrants seeking content stored outside the US |
| GitLab | Publishes transparency-report links and law-enforcement guidelines |
| Atlassian | Trust / legal pages exist; Bitbucket-specific transparency less prominent — aggregation across Atlassian products is a negative signal for git-custody-specific visibility |
| SourceHut / Codeberg / community | Strong values statements ("you cannot have our users' data"), weaker formal statistics |

**Warrant canaries**: do not assume one exists for mainstream git hosts unless you can find a current, signed, regularly updated statement. Silence is signal. No major git host operates a moved / triggered warrant canary as of May 2026.

### Employer / client IP and contractor-agreement risks

A frequently missed dimension. If code is written for a client or employer:

| Risk | Example |
|---|---|
| Unauthorized third-party disclosure | Client agreement may prohibit storing source outside approved systems |
| AI-training-clause exposure | Even opt-out default may breach a "no model training / no reuse" clause |
| Foreign access | Support staff in other countries may create export-control or confidentiality problems |
| Repo metadata leakage | Commit authors, branch names, issue titles, filenames can reveal client projects even with private code |
| Contractor ownership disputes | If repo account is personal not client / org-owned, chain-of-title becomes messy |

### Insurance considerations

Cyber liability, E&O, and tech professional-liability policies increasingly care about:

- MFA enforcement on source-code stores
- Independent backups
- Regulated data / test data in repos
- Vendor SOC 2 / ISO 27001 status
- Contractual DPAs / BAAs in place
- AI / data-processing clauses vs. client representations
- Export-controlled work and unauthorized foreign access

The question for insurance is not "is this host secure?" — it's **"does this host satisfy the controls represented in my policy application?"**

---

## C. Safety / security considerations (posture, not config)

### What attestations prove (and don't)

| Attestation | Proves | Does not prove |
|---|---|---|
| SOC 2 Type II | Controls operated over a period (security / availability / confidentiality) | Zero breaches, zero employee access, source-code confidentiality by design |
| SOC 1 | Financial-reporting control relevance | Usually irrelevant to solo source-code custody |
| ISO 27001 | ISMS exists and is audited | Technical architecture or absence of insider access |
| ISO 27017 / 27018 | Cloud-security / cloud-privacy control extensions | Cryptographic custody |
| PCI DSS / AoC | Cardholder-data environment controls | Mostly irrelevant unless repos include payment artifacts |
| FedRAMP | US government cloud authorization, continuous monitoring, control baseline | Does not equal ITAR, zero knowledge, or immunity to US legal process |
| CSA STAR / CAIQ | Cloud-control transparency questionnaire | Self-assessment may be weaker than audit |

Provider state of attestations (2026):

| Provider | SOC 2 Type II | ISO 27001 | FedRAMP | Other |
|---|---|---|---|---|
| GitHub | Yes | 27001:2022 | LI-SaaS (low) | PCI AoC, CSA STAR |
| GitLab.com / Dedicated | Yes | 27001 / 27017 / 27018 | Moderate (Dedicated for Government, May 2025) | PCI AoC, CSA STAR |
| Bitbucket Cloud | Yes | Yes | Government Cloud — Moderate | CSA STAR |
| Atlassian Government Cloud | Yes | Yes | Moderate | ~25% premium over Cloud Enterprise |
| SourceHut | None published | None | No | None |
| Codeberg | None | None | No | None |
| Gitea Cloud / Enterprise | Claimed (12-month observation) | Not verified | No | SOC 3 claimed |
| Forgejo / Gitea community | N/A — software, not service | N/A | N/A | N/A |

### Encryption posture compared

- **Encryption in transit**: TLS / SSH everywhere on mainstream hosts. Operational history matters — GitHub rotated its RSA SSH host key in 2023 after exposure in a public repository.
- **Encryption at rest**: platform-managed AES-256 on all mainstream hosts. GitLab.com uses GCP-managed at-rest encryption.
- **BYOK / CMEK**: available on enterprise / dedicated tiers (GitLab Dedicated, some Atlassian Cloud Premium); rarely applies to git object readability — usually only infrastructure layers.
- **Customer-held application-layer keys (zero knowledge)**: not offered by any mainstream SaaS git host in 2026. This is the structural truth that reshapes the rest of the analysis.

### Employee-access architecture

| Provider | Posture |
|---|---|
| GitHub | Access permitted for security, support, service integrity, legal compliance, user-enabled features |
| GitLab | Consent-gated for support; documented "files needed only, scope-limited, cloned repos deleted after issue resolution" |
| Bitbucket | RBAC / need-to-know; audit-log coverage explicitly incomplete |
| SourceHut | Operator and small team have root |
| Codeberg | Volunteer admins have root; smaller team |

None of these are zero-access architectures. Customer support touching customer data is the norm. **If a provider is silent on this, assume broader access than marketing suggests.**

### Incident history with custody lessons

| Incident | Custody lesson |
|---|---|
| **GitHub 2016 private-repo disclosure** | A bug exposed a small amount of private repo data via git pulls / clones; 156 private repositories affected; users notified. **Direct evidence that "private repo" is not a cryptographic boundary.** |
| **GitLab.com 2017 database incident** | Lost production database from a 6-hour window. Git / wiki repos not affected, but metadata (issues, MRs, comments, snippets, users) was. ~5,000 projects, 5,000 comments, 700 user accounts affected. Unusually transparent postmortem. |
| **2019 git ransom campaign** | GitHub, GitLab, Bitbucket jointly reported no platform compromise; 392+ repos hit through leaked credentials / tokens. **Credential discipline matters more than provider choice for the most likely attack.** |
| **GitHub 2023 SSH host-key exposure** | RSA SSH host key exposed in a public repository; GitHub rotated. Relevant to SSH trust hygiene and incident-response speed. |
| **Ongoing secret leakage across forges** | Large-scale secret exposure documented across GitHub / GitLab / Bitbucket / Common Crawl. **Repository custody and secret hygiene are separate problems.** |
| **2024-2025 trend** | GitHub-published reports note ~21% YoY increase in incidents affecting users — largely supply-chain and credential-based, not platform compromise |

### Account-takeover protections

| Provider | TOTP MFA | Hardware key (FIDO2) | SSO / SAML | IP allowlist | Audit log API |
|---|---|---|---|---|---|
| GitHub | Yes | Yes | Enterprise | Enterprise | Enterprise |
| GitLab | Yes | Yes | Premium+ | Ultimate | Premium+ |
| Bitbucket | Yes | Yes | Atlassian Access | Premium | Premium (incomplete coverage) |
| SourceHut | Yes (TOTP) | No | No | No | Account activity log |
| Codeberg | Yes | Yes (Forgejo) | No | No | Limited |

---

## D. Regulatory framework considerations

### GDPR / BDSG: what a small commercial operation inherits

Choosing an EU host can simplify EU-client expectations but pulls the developer into processor / controller paperwork:

- DPA with the host may be required
- Subprocessors must be disclosed
- Cross-border transfers require DPF, SCCs, adequacy, or other legal basis
- Personal data in repos counts: test fixtures, logs, customer names, emails, commit metadata
- Deletion requests conflict with git immutability if personal data lands in history

### HIPAA / PCI / SOX — when code custody starts to matter

| Framework | Trigger |
|---|---|
| **HIPAA** | Repos contain PHI, production logs, test fixtures derived from patient data, deployment scripts giving PHI access, or secrets to HIPAA systems. BAA required; many hosts won't sign one on ordinary tiers. |
| **PCI DSS** | Repos contain cardholder data, PAN samples, payment secrets, or code materially affecting cardholder-data environments. Comments / scripts referencing real PANs trigger scope. |
| **SOX** | Public-company financial-reporting systems: source-control access, change approval, audit trails become ITGC evidence. Less about secrecy, more about change-control integrity. |

### Export control: ITAR / EAR

Source code itself can be controlled technical data or technology. Practical implications:

- Foreign support access can constitute a "deemed export" violation
- Public / private SaaS classification is not the controlling axis — personnel nationality and access controls are
- FedRAMP does not automatically mean ITAR-ready
- ITAR-ready usually implies US persons + US region + contractual controls + auditability + export-control-specific commitments

See the **Strategic environment / ITAR adjacency** section above for the cost magnitudes.

### CMMC for defense work

CMMC matters when handling Federal Contract Information or Controlled Unclassified Information for US defense contracts. Git hosting becomes relevant if source, build scripts, issues, logs, or secrets are CUI / FCI-adjacent. Dominant considerations:

- Is the host inside the assessed boundary?
- Does it support required access-control / audit / incident controls?
- Does it have FedRAMP Moderate equivalence where required?
- Are support personnel and subprocessors acceptable?

For a solo developer, CMMC can convert "cheap git host" into "must use approved enterprise / government cloud or client-controlled system."

### Sector overlays

| Sector | Source-code custody consideration |
|---|---|
| FinCEN / financial services | Vendor risk, audit logs, change control, insider access, sanctions exposure |
| FDA software-as-medical-device | Traceability, design history, change control, defect records, validation evidence — git host artifacts may become regulated records |
| Automotive ISO 26262 | Safety lifecycle traceability; source-control evidence and tool qualification |
| Aviation / aerospace (DO-178C) | Export control + safety traceability + supplier access + retention |
| Critical infrastructure | NIS2 / CRA pressure in EU; supply-chain security evidence |

### EU CRA / NIS2

- **CRA** targets products with digital elements: lifecycle cybersecurity obligations, vulnerability handling, incident reporting
- **NIS2** establishes a cybersecurity framework across 18 critical sectors in the EU

For a solo developer the git-hosting implication is not "choose an EU host." It is: **can I produce evidence of secure development, vulnerability handling, provenance, access control, and incident response if my software becomes regulated?** Host posture on those evidence trails is the consideration; geography of the host is secondary.

---

## E. Considerations a paranoid solo developer may not have thought to ask

### 1. Source-available / managed self-hosted middle ground

Between "trust a closed SaaS" and "run a box yourself":

| Category | Examples | Why it matters |
|---|---|---|
| Managed Gitea / Forgejo | Gitea Cloud, third-party managed instances | More control surface than shared SaaS; less operational burden than self-hosting. Provider still has infrastructure custody. |
| Enterprise source-available / self-managed support | GitLab Self-Managed, Gitea Enterprise | Better contractual support and compliance artifacts; licensing and feature gates matter |
| Dedicated managed | GitLab Dedicated, Gitea Enterprise-hosted | Stronger isolation; expensive |
| Federated / P2P | Radicle | Custody model is peer replication rather than provider — not a conventional compliance answer. Radicle nodes are Ed25519-key-identified peers seeding repositories. |

**Consideration**: source-available hosted means "I could in principle audit the code that holds my code, and I could escape to self-hosted on the same software." Structurally different trust position than closed-source SaaS.

### 2. Developer death / incapacitation

Provider policies often assume the user is alive to recover an account. A solo developer should ask:

- Can an executor access the account?
- Can a business continue using the repo if I die?
- Are private keys, recovery codes, billing credentials recoverable?
- Does the provider have a deceased-user or corporate succession process?
- Are client NDAs assignable to an executor or company?

Provider state:

| Provider | Mechanism |
|---|---|
| GitHub | Built-in **Successor Settings** — designate another GitHub user; on death certificate (7-day wait) or obituary (21-day wait), successor can archive / transfer **public** repos. Private repos require Support engagement. Successor cannot log in. |
| GitLab | No formal successor feature; Support handles requests with documentation; less documented policy |
| Bitbucket / Atlassian | Estate handled via Support; no built-in successor mechanism |
| SourceHut | No documented policy |
| Codeberg | No documented policy; community / admin discretion |

This is a business-continuity issue, not just a backup issue. **None of the above handles private code well.**

### 3. Nation-state-aligned hosts

Avoiding US providers does not automatically improve the threat model if the alternative is state-aligned or opaque. Hosts tied to Russia, China, or other high-state-control environments may be unacceptable when:

- Code has commercial strategic value
- Customers are in defense, critical infrastructure, finance, or security
- Sanctions / export control matters
- Source disclosure to that jurisdiction would be reputationally fatal

Also relevant: **third-party dependencies** hosted on these platforms entering your build chain (the Yandex fast-glob example illustrates the supply-chain dimension).

### 4. Reproducible builds / provenance as custody

A host can protect confidentiality yet fail integrity. Increasingly relevant questions:

- Can I prove this source produced that artifact?
- Can I prove no one silently rewrote history?
- Can I preserve signed tags and release provenance across hosts?
- Does the host preserve, expose, or mutate signatures?
- Does the host integrate with SLSA / Sigstore / gitsign without forcing code into AI / security scanners?

SLSA is build-integrity, not repository-secrecy, but it becomes part of custody once source is valuable and artifacts are relied upon by others.

### 5. Cross-border support and subprocessors

Data residency often covers storage, not support. The real question:

> Who can see my code during support, abuse handling, legal review, incident response, AI processing, malware scanning, or backup restoration?

If the provider is silent, assume broader access than marketing suggests.

### 6. Repo metadata as leakage

Even when source files are private, the following can leak sensitive facts:

- Repository names
- Branch names
- Commit messages
- Author emails
- Timestamps
- File paths
- Issue titles and PR discussions
- CI logs
- Package names
- Secret-scanning alerts
- Support tickets
- Billing entity
- IP addresses and clone times

A provider can disclose or analyze metadata even if it does not train on file contents.

### 7. Personal account vs. legal entity

A solo developer often starts on a personal account. For crown jewels:

- Should the repo belong to an LLC / C-corp / org account?
- Who owns billing?
- Who can prove chain-of-title?
- Does a client contract require client-controlled repos?
- Does personal-account ToS differ from organization / enterprise ToS? (It almost always does.)

### 8. Community-host mission mismatch

Codeberg is the canonical example: attractive on values, but its own FAQ explicitly frames the service around free-software use; community discussions repeatedly note proprietary private repos are **not** the intended use. The risk is not just possible deletion — it is mission mismatch between operator and user.

### 9. "Security features" as additional data-processing surfaces

Secret scanning, code search, code intelligence, dependency analysis, AI code review, vulnerability scanning — all require deeper processing of code. Net-positive for security in most cases, but for custody they are extra processing surfaces and ToS-clause exposures.

### 10. Free plan as adverse signal

Free private repos are convenient, but if the code is truly valuable, the provider's incentive model matters. A paid plan does not guarantee confidentiality, but it usually gives better contractual posture, support path, audit / control features, and standing to negotiate changes.

### 11. EU sovereignty push

EU sovereignty initiatives (Gaia-X, EuroStack) are creating political pressure for EU-resident git hosting. Codeberg, EU-resident GitLab partners, and others may see institutional adoption that improves their durability over the next 2-5 years. Worth watching, not yet decisive.

### 12. CRA scope-creep

CRA reporting obligations starting September 2026 apply to "products with digital elements." OSS projects published from EU repos and their hosters may inherit obligations. Full scope is still being worked out — a near-term landscape change to watch.

---

## Considerations checklist (priority order)

### Priority 1 — disqualifiers

1. Does the provider explicitly reserve rights to use private code, prompts, or repo content for AI training or product improvement?
2. Is there an opt-out, and is it account-wide, organization-wide, contractual, and durable?
3. Does the host receive plaintext source, or only encrypted / opaque artifacts?
4. Which legal entity contracts with me, in what jurisdiction?
5. Can US / UK / EU / other state process reach the provider or its parent?
6. Does my client / employer contract allow this provider?
7. Does the provider support mandatory MFA / hardware keys on the plan I can actually buy?
8. Can I export the full git history without platform-specific dependencies?
9. What happens if the account is suspended, billing fails, or I die?
10. Is this provider mission-compatible with proprietary private code?

### Priority 2 — legal / commercial diligence

11. Do I retain ownership, and is the provider license narrowly limited to operating the service?
12. Are there sublicensing, affiliate-sharing, perpetual-license, or unilateral-change clauses?
13. Mandatory arbitration or class-action waiver?
14. Is there a DPA? Are subprocessors listed?
15. Does data residency cover support, telemetry, backups, AI, and abuse / security processing?
16. Does the provider publish law-enforcement statistics, DMCA notices, and government takedown data?
17. Warrant canary? Current, signed, meaningful?
18. Seat minimums or enterprise-tier gates for the controls I actually need?
19. Is the product strategically core to the provider, or legacy / peripheral?
20. Has the provider recently changed pricing, AI policy, or product availability?

### Priority 3 — security posture

21. Encryption at rest: provider-managed, BYOK / CMEK, or true customer-held-key?
22. Are employee / support accesses JIT-approved and audited, or merely "need to know"?
23. Are git read events auditable, or only admin / config events?
24. Audit-log retention duration and export capability?
25. Are SSH host keys documented and rotation incidents disclosed?
26. SOC 2 Type II, ISO 27001 / 27017 / 27018, CSA STAR, FedRAMP, or equivalent?
27. What's actually in scope for those attestations?
28. Has the provider had private-repo disclosure, data-loss, or credential-wiper incidents?
29. Incident-disclosure behavior: fast, specific, user-notifying, postmortem-quality?
30. Are secret scanning, code search, AI features optional and contractually constrained?

### Priority 4 — regulatory fit

31. Could any repo content be PHI, PCI data, CUI, export-controlled technical data, or regulated test data?
32. Does the host sign BAAs, DPAs, government addenda, or export-control commitments if needed?
33. Are foreign nationals or non-approved support regions able to access content?
34. Would this host satisfy cyber / E&O insurance representations?
35. Would source-control audit trails satisfy SOX, FDA, automotive, defense, or financial-sector evidence needs?
36. Does the provider's supply-chain / provenance posture help or hinder later SLSA / SBOM requirements?
37. Do CRA / NIS2-style obligations require evidence the host can preserve?

### Priority 5 — continuity

38. Can I leave in one day with complete git history?
39. Can I preserve issues / PRs / releases if they become business records?
40. Is there an independent backup / mirror path outside the provider?
41. Can I prove remote history was not silently rewritten?
42. Can an executor, business partner, or client recover access if I am incapacitated?
43. What is my plan if the provider is acquired, pivots to AI, sunsets the product, or changes ToS?
44. What is my plan if the provider receives a takedown or legal demand?
45. What is the maximum tolerable outage or lockout?

---

## Open questions / things to revisit

- DPF stability — re-check after each major CJEU decision
- CRA scope clarification — particularly OSS / hoster obligations as 2027 approaches
- GitHub AI clause direction — Free / Pro / Pro+ vs Enterprise treatment may diverge further
- Atlassian's Aug 2026 AI / metadata clause — track whether opt-out coverage expands beyond Enterprise
- Radicle maturity — annual re-evaluation as mirror-availability layer
- EU sovereignty institutional adoption — Codeberg's durability profile may improve if hosted EU-public-sector workloads land there

---

## Sources

### Provider documentation and policy

- [GitHub Terms of Service](https://docs.github.com/en/site-policy/github-terms/github-terms-of-service)
- [GitHub Privacy / AI training update — March 2026](https://github.blog/changelog/2026-03-25-updates-to-our-privacy-statement-and-terms-of-service-how-we-use-your-data/)
- [GitHub Transparency Center](https://transparencycenter.github.com/)
- [GitHub DMCA Takedowns](https://transparencycenter.github.com/dmca/)
- [GitHub Deceased User Policy](https://docs.github.com/en/site-policy/other-site-policies/github-deceased-user-policy)
- [GitHub Secret Leakage Risks](https://docs.github.com/en/code-security/concepts/secret-security/secret-leakage-risks)
- [GitHub Pricing](https://github.com/pricing)
- [GitHub Enterprise Cloud Compliance Reports](https://docs.github.com/enterprise-cloud@latest/organizations/keeping-your-organization-secure/managing-security-settings-for-your-organization/accessing-compliance-reports-for-your-organization)
- [GitHub 2016 Private Repo Disclosure Incident](https://github.blog/news-insights/incident-report-inadvertent-private-repository-disclosure/)
- [GitHub on AWS GovCloud](https://government.github.com/aws-govcloud)
- [GitHub and Trade Controls](https://docs.github.com/en/site-policy/other-site-policies/github-and-trade-controls)
- [GitHub Copilot data residency + FedRAMP — April 2026](https://github.blog/changelog/2026-04-13-copilot-data-residency-in-us-eu-and-fedramp-compliance-now-available/)
- [GitLab Privacy Statement](https://about.gitlab.com/privacy/)
- [GitLab Security FAQ](https://about.gitlab.com/security/faq/)
- [GitLab AI Transparency Center](https://about.gitlab.com/ai-transparency-center/)
- [GitLab Transparency Reports](https://handbook.gitlab.com/handbook/legal/privacy/transparency-reports/)
- [GitLab AI Functionality Terms V4](https://handbook.gitlab.com/handbook/legal/ai-functionality-terms-v4/)
- [GitLab Content Removal Guidelines](https://handbook.gitlab.com/handbook/legal/dmca/)
- [GitLab Dedicated Encryption](https://docs.gitlab.com/administration/dedicated/encryption/)
- [GitLab Dedicated for Government](https://docs.gitlab.com/subscriptions/gitlab_dedicated_for_government/)
- [GitLab.com 2017 Database Incident Postmortem](https://about.gitlab.com/blog/gitlab-dot-com-database-incident/)
- [GitLab FedRAMP page](https://about.gitlab.com/solutions/public-sector/fedramp/)
- [Atlassian Customer Agreement](https://www.atlassian.com/legal/atlassian-customer-agreement)
- [Atlassian Transparency Report](https://www.atlassian.com/trust/privacy/transparency-report)
- [Atlassian Law Enforcement Guidelines](https://www.atlassian.com/trust/privacy/guidelines-for-law-enforcement)
- [Atlassian Cloud Security](https://www.atlassian.com/software/bitbucket/features/cloud-security)
- [Atlassian Data Center End of Life](https://www.atlassian.com/licensing/data-center-end-of-life)
- [GitLab opt-out vs Atlassian AI](https://about.gitlab.com/blog/atlassian-will-train-on-your-data-opt-out-with-gitlab/)
- [Codeberg FAQ](https://docs.codeberg.org/getting-started/faq/)
- [Forgejo](https://forgejo.org/)
- [Forgejo Governance](https://forgejo.org/docs/next/contributor/governance/)
- [Gitea](https://about.gitea.com/)
- [Gitea MFA Documentation](https://docs.gitea.com/usage/multi-factor-authentication)
- [SourceHut](https://sourcehut.org/)
- [SourceHut Pricing](https://sourcehut.org/pricing)
- [SourceHut "You cannot have our users' data"](https://sourcehut.org/blog/2025-04-15-you-cannot-have-our-users-data/)
- [Radicle](https://radicle.xyz/)
- [Radicle 1.7.0 release](https://radicle.xyz/2026/03/18/radicle-1.7.0)
- [Gitee — state designation 2020 (Slashdot)](https://developers.slashdot.org/story/20/08/24/1441210/chinas-ministry-of-it-picks-gitee-to-build-independent-open-source-code-hosting-platform-for-the-country-as-tension-with-the-us-escalates)
- [Gitee 2022 mandatory review (The Register)](https://www.theregister.com/2022/05/20/gitee_code_review/)
- [NotABug.org current state](https://notabug.org/)

### Incidents and reporting

- [2019 Git Ransom Campaign Incident Report (joint)](https://github.blog/open-source/git/git-ransom-campaign-incident-report/)
- [GitHub 2023 RSA SSH Key Exposure (Dark Reading)](https://www.darkreading.com/application-security/github-private-rsa-ssh-key-mistakenly-exposed-public-repository)
- [Microsoft Government Requests for Customer Data](https://www.microsoft.com/en-us/corporate-responsibility/reports/government-requests/customer-data)

### Legal frameworks

- [EU-US Data Privacy Framework — Program Overview](https://www.dataprivacyframework.gov/Program-Overview)
- [EU-US DPF Survives First Challenge (Freshfields)](https://www.freshfields.com/en/our-thinking/blogs/technology-quotient/eu-us-data-privacy-framework-survives-its-first-judicial-challenge-but-more-are-102l4m1)
- [EU New Standard Contractual Clauses](https://commission.europa.eu/law/law-topic/data-protection/international-dimension-data-protection/new-standard-contractual-clauses-questions-and-answers-overview_en)
- [US DOJ CLOUD Act Resources](https://www.justice.gov/criminal/cloud-act-resources)
- [AWS CLOUD Act Explainer](https://aws.amazon.com/compliance/cloud-act/)
- [FISA Section 702 Booklet (US IC)](https://www.intel.gov/assets/documents/702-documents/702_Booklet-FINAL.pdf)
- [UK Investigatory Powers Act — Notices Regime Code of Practice](https://www.gov.uk/government/publications/notices-regime-code-of-practice/notices-regime-code-of-practice-accessible)

### Regulatory frameworks (EU)

- [EU Cyber Resilience Act (digital-strategy.ec.europa.eu)](https://digital-strategy.ec.europa.eu/en/policies/cyber-resilience-act)
- [EU NIS2 Directive (digital-strategy.ec.europa.eu)](https://digital-strategy.ec.europa.eu/en/policies/nis2-directive)

### Government / FedRAMP / ITAR

- [GitLab Dedicated for Government — FedRAMP Marketplace](https://www.fedramp.gov/marketplace/products/FR2411959145/)
- [AWS GovCloud Compliance](https://docs.aws.amazon.com/govcloud-us/latest/UserGuide/govcloud-compliance.html)
- [Azure Government ITAR overview](https://github.com/MicrosoftDocs/azure-docs/blob/main/articles/azure-government/documentation-government-overview-itar.md)
- [Vanderbilt — Software & Export Controls primer](https://cdn.vanderbilt.edu/vu-URL/wp-content/uploads/sites/381/2022/07/20030228/EC-Software.pdf)
- [DDTC ITAR Registration Fees (cmmccompliance.us)](https://cmmccompliance.us/understanding-the-new-itar-registration-payment-requirements-for-2025/)
- [CMMC Level 2 Cost — 2026 Budget Guide](https://ibsscorp.com/cmmc-level-2-cost-complete-2026-budget-guide-for-defense-contractors/)
- [CMMC Certification Cost Breakdown (Secureframe)](https://secureframe.com/hub/cmmc/certification-cost)

### Supply-chain / provenance

- [SLSA Threats & Mitigations](https://slsa.dev/threats)
