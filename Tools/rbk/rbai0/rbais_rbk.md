### RBK Core Context

What each file and family of the Recipe Bottle Kit is and does, and the map of the moorings tree.
The acronym index `rbaip_rbk.md` lists the same rows bare.

#### Acronym Notes

Annotations for the acronym homes indexed in `Tools/rbk/rbai0/rbaip_rbk.md` — the per-row descriptions and family topology the index does not carry.

- **RBAU** → `Tools/rbk/rbau_auth.sh` (Auth — loads the payor's credential and mints a mantle's token; holds the sederunt lifecycle: avow reuses a standing sederunt only while enough runway remains, and `rbau_novate` opens a fresh one. Its CLI partner `rbau_cli.sh` serves novate (`rbw-aN`) and espy (`rbw-as`), which reports from the local cache alone whether a sederunt stands and how much runway it has left)
- **RBDC** → `Tools/rbk/rbdc_derived.sh`
- **RBDG** → `diagrams/rbdg*` (Recipe Bottle diagrams — PlantUML `.puml` sources with rendered `-light`/`-dark` `.svg` pairs for theme-aware `<picture>` embedding; `rbdg` itself names no file. Members: `rbdgl_` federation login, `rbdgs_` federation setup, `rbdgm_` federation seam. The pluml crucible case `rbtccc_pluml_render_diagrams` renders every `diagrams/*.puml` in both modes, so a new diagram is one new `rbdgX_*.puml` file there.)
- **RBF**  → Foundry prefix (names no file; children rbfb, rbfc, rbfd, rbfh, rbfk, rbfl, rbfr, rbfv)
- **RBFB** → `Tools/rbk/rbfb_beckon.sh` (Foundry Beckon — after an operation records a fact, prints the tabtargets that read that fact next. One emitter per fact; today only `rbfb_hallmark`, naming the summon, plumb and rekon readers and the anoint and mitre writers of a new hallmark. Sourced by ordain and kludge.)
- **RBFC** → `Tools/rbk/rbfc*` (Foundry Core family — names no file; its CLI is `rbfc0_cli.sh`.)
  - **RBFCV** → `Tools/rbk/rbfcv_*.sh` (Foundry Core vessel-resolution)
  - **RBFCB** → `Tools/rbk/rbfcb_*.sh` (Foundry Core build-host primitives — waiting on a build, reading git metadata, writing a step script body, native paths, and splicing shared snippets into cloud steps; sourced by the family entry `rbfc0_core.sh`, and directly by the theurge fixtures that test it.)
  - **RBFCA** → `Tools/rbk/rbfca_*.sh` (Foundry Core step-assembly)
  - **RBFCG** → `Tools/rbk/rbfcg_*.sh` (Foundry Core Artifact Registry REST calls)
  - **RBFCP** → `Tools/rbk/rbfcp_*.sh` (Foundry Core plumb — the hallmark provenance display)
  - **RBFC0** → `Tools/rbk/rbfc0_core.sh` (Foundry Core entry — the family's one inclusion guard and module setup, and the tool-image globals `z_rbfc_tool_*`. CLI partner `rbfc0_cli.sh`.)
- **RBFD** → `Tools/rbk/rbfd*` (Foundry Director family — names no file. Director-side Cloud Build orchestration: `rbfdo_ordain`, `rbfdb_build`, `rbfdm_mirror` and `rbfdg_graft`, with the reliquary, quota and registry preflights. Its CLI is `rbfd0_cli.sh`.)
  - **RBFDP** → `Tools/rbk/rbfdp_preflight.sh` (Foundry Director preflight — the registry preflight every operation runs, `zrbfdp_registry_preflight`, and beneath it `zrbfdp_preflight_reliquary`, which checks the reliquary's tool cohort)
  - **RBFDB** → `Tools/rbk/rbfdb_build.sh` (Foundry Director conjure — `rbfdb_build` submits a direct Cloud Build, with its quota preflight, the build JSON stitcher `zrbfdb_stitch_build_json` and the pouch push `zrbfdb_push_build_context`)
  - **RBFDM** → `Tools/rbk/rbfdm_mirror.sh` (Foundry Director mirror — `rbfdm_mirror` copies an annex vessel's upstream image into the registry through one Cloud Build, with its submit helper `zrbfdm_mirror_submit`)
  - **RBFDG** → `Tools/rbk/rbfdg_graft.sh` (Foundry Director graft — `rbfdg_graft` local image tag-and-push into GAR under a hallmark minted from the image's own creation timestamp)
  - **RBFDO** → `Tools/rbk/rbfdo_ordain.sh` (Foundry Director ordain — dispatches on the vessel's mode, then submits the vouch or the graft metadata through rbfv and prints the new hallmark's next tabtargets)
  - **RBFD0** → `Tools/rbk/rbfd0_director.sh` (Foundry Director entry — the family's one inclusion guard and module setup. CLI partner `rbfd0_cli.sh`.)
- **RBFH** → `Tools/rbk/rbfh_hygiene.sh` (Foundry Hygiene — Dockerfile FROM-line constraint shared by kludge and conjure; thin CLI partner `Tools/rbk/rbfh_cli.sh` surfaces the contract via `rbw-fhc`/`rbw-fhv`)
- **RBFK** → `Tools/rbk/rbfk_kludge.sh` (Foundry Kludge — local vessel image build for development (`rbfk_kludge`); CLI partner `rbfk_cli.sh`, colophon `rbw-fk`)
- **RBFL** → `Tools/rbk/rbfl*` (Foundry Ledger family — names no file; its CLI is `rbfl0_cli.sh`.)
  - **RBFLY** → `Tools/rbk/rbfly_*.sh` (Foundry Ledger yoke — reliquary-touchmark yoke into vessel regimes)
  - **RBFLA** → `Tools/rbk/rbfla_anoint.sh` (Foundry Ledger anoint — rewrites `RBRV_GRAFT_IMAGE` in one graft vessel from the facts a graft recorded; a chain-link verb on the vessel's config)
  - **RBFLF** → `Tools/rbk/rbflf_feoff.sh` (Foundry Ledger feoff — elects one conjure vessel's base anchor `RBRV_IMAGE_n_ANCHOR` from a base-image Lode's touchmark, given as an argument or read from the fact the last ensconce recorded; a chain-link verb, colophon `rbw-rvf`. It refuses a broken chain or a Lode of another kind.)
  - **RBFLS** → `Tools/rbk/rbfls_seise.sh` (Foundry Ledger seise — elects the repository's substrate reliquary `RBRR_SUBSTRATE_RELIQUARY` from a reliquary Lode's touchmark, given as an argument or read from the fact the last conclave recorded; underpin and immure take their build tools from it. A chain-link verb, colophon `rbw-rrs`; it refuses a broken chain or a Lode of another kind.)
  - **RBFLD** → `Tools/rbk/rbfld_*.sh` (Foundry Ledger delete — reliquary / ledger removal gesture)
  - **RBFLN** → `Tools/rbk/rbfln_*.sh` (Foundry Ledger inventory — ledger listing / enumeration gesture)
  - **RBFLW** → `Tools/rbk/rbflw_*.sh` (Foundry Ledger wrest — image wrest from registry)
  - **RBFL0** → `Tools/rbk/rbfl0_ledger.sh` (Foundry Ledger entry — the family's one inclusion guard and module setup. CLI partner `rbfl0_cli.sh`.)
- **RBFR** → `Tools/rbk/rbfr_retriever.sh` (Foundry Retriever — retriever-credentialed hallmark summon (`rbfr_summon`); CLI partner `rbfr_cli.sh`)
- **RBFV** → `Tools/rbk/rbfv*` (Foundry Verify family — names no file. Hallmark provenance verification: `rbfvv_vouch`, `rbfva_about`, `rbfvg_vouch_gate` and `rbfvb_batch_vouch`, plus the combined about-and-vouch submission graft mode takes from rbfd's ordain. Its CLI is `rbfv0_cli.sh`.)
  - **RBFVG** → `Tools/rbk/rbfvg_gate.sh` (Foundry Verify gate — `rbfvg_vouch_gate`, the read-only admission check: it reads a hallmark's vouch manifest in the registry and refuses an image carrying none, submitting no Cloud Build)
  - **RBFVA** → `Tools/rbk/rbfva_about.sh` (Foundry Verify about — `rbfva_about`, the standalone about-metadata operation, and its Cloud Build submission `zrbfva_about_submit`)
  - **RBFVM** → `Tools/rbk/rbfvm_metadata.sh` (Foundry Verify graft metadata — `zrbfvm_graft_metadata_submit`, the one Cloud Build that writes a graft's about and vouch together, so a graft never stands with one and not the other; its only caller is rbfd's ordain)
  - **RBFVV** → `Tools/rbk/rbfvv_vouch.sh` (Foundry Verify vouch — `rbfvv_vouch`, the mode-aware vouch, and its Cloud Build submission `zrbfvv_vouch_submit`)
  - **RBFVB** → `Tools/rbk/rbfvb_batch.sh` (Foundry Verify batch — `rbfvb_batch_vouch`, the two-pass driver that rolls every pending hallmark (image and about present, vouch absent) and calls `rbfvv_vouch` on each)
  - **RBFV0** → `Tools/rbk/rbfv0_verify.sh` (Foundry Verify entry — the family's one inclusion guard and module setup. CLI partner `rbfv0_cli.sh`. Both that CLI and `rbfd0_director.sh` source it, which is why the guard stands here.)
- **RBGA** → `Tools/rbk/rbga_registry.sh`
- **RBGB** → `Tools/rbk/rbgb_buckets.sh`
- **RBGC** → `Tools/rbk/rbgc_constants.sh`
- **RBGE** → `Tools/rbk/rbge_rest.sh` (Google REST — LRO polling + API-enable patterns over rbuh)
- **RBGFT** → `Tools/rbk/rbgft_terrier.sh` (Federation Terrier — the muniment data layer: `rbgft_engross`, `rbgft_expunge` and `rbgft_peruse`, and the manor-wide read `rbgft_peruse_manor`, composed by the payor module's admission verbs. The caller supplies the token. Muniment wire keys begin `rbgft_`.)
- **RBGG** → `Tools/rbk/rbgg_governor.sh`
- **RBGI** → `Tools/rbk/rbgi_iam.sh`
- **RBGO** → `Tools/rbk/rbgo_oauth.sh` (also owns the stateless `rbgo_curl_status_is_transient_predicate` transient-curl-exit classifier)
- **RBGP** → `Tools/rbk/rbgp_payor.sh` (Payor — also the admission verbs `rbgp_brevet`, `rbgp_unseat`, `rbgp_attaint` and `rbgp_rehearse`, dispatched by `rbgp_cli` and wielded as a donned governor mantle (`rbau_avow`, then `rbau_don_capture governor`), colophons `rbw-pB`/`rbw-pU`/`rbw-pA`/`rbw-pr`; and the payor's own `rbgp_gird` (colophon `rbw-mG`), which seats a depot's first governor, the one admission made outside governor wielding.)
- **RBGV** → `Tools/rbk/rbgv_probe.sh` (Google Verification — JWT SA and Payor OAuth access probes)
- **RBGW** → `Tools/rbk/rbgw_capabilities.sh` (Capability sets — the one home of the three per-role grant lists, governor, director and retriever, applied alike to an enrobed service account and to the mantle service accounts at levy; a library with no CLI, sourced by `rbgg_cli` and `rbgp_cli`)
- **RBGJS** → `Tools/rbk/rbgjs/` (shared cloud-step snippets — each `rbgj«family»/` directory holds one family's in-pool step scripts (`rbgja` about, `rbgjb` build, `rbgjl` lode, `rbgjm` mirror, `rbgjr` reliquary, `rbgjv` verify), and `rbgjs` holds the fragments shared across families, spliced into a step at `#@rbgjs_include «name»` markers by `zrbfcb_expand_includes` in `rbfcb_host.sh`. It names no bare file.)
- **RBH0** → `Tools/rbk/rbh0/` (Handbook directory — one function per file)
  - `rbh*` is the Handbook family (human-facing procedures), beside `rbg*` (Google machinery). Two groups: `RBHO` onboarding, `RBHP` payor. Onboarding's colophons are `rbw-o` (its entry) and `rbw-O*` (its tracks); the payor's guided procedures run as `rbw-gP*`.
  - **RBHO0** → `Tools/rbk/rbh0/rbho0_*.sh` (Onboarding — CLI `rbho0_cli.sh` and the group's entry module `rbho0_onboarding.sh`, holding its probes and shared helpers)
  - **RBHP0** → `Tools/rbk/rbh0/rbhp0_*.sh` (Payor — CLI `rbhp0_cli.sh`, with full regime, OAuth and IAM dependencies, and the group's entry module `rbhp0_payor.sh`)
- **RBID** → `rbmm_moorings/rbmv_vessels/common-ifrit-context/` (Ifrit — in-bottle attack binary for crucible testing; shared source across tether/airgap variants)
- **RBJ**  → Jailer prefix (names no file; children rbjs, rbjp, rbje, rbjh)
- **RBJS** → `rbmm_moorings/rbmv_vessels/common-sentry-context/rbjs_sentry.sh` (Jailer Sentry - container security setup: iptables, dnsmasq, enclave network; ships in the sentry vessel build context, not Tools/rbk/)
- **RBLD** → `Tools/rbk/rbld*` (Lode capture family — capture on the fetching side, with the in-pool cloud steps in `rbgjl/`; registry namespace `rbi_ld`; colophons `rbw-l*`. `rbld` names no file. Its children: CLI `rbld0_cli`, entry `rbld0_lode`, lifecycle `rbldl_`, capture-assembly spine `rblds_`, delete `rbldd_`, kind decoder `rbldk_`, and the per-kind bodies `rbldb_`/`rbldr_`/`rbldw_`/`rbldv_`.)
  - **RBLD0** → `Tools/rbk/rbld0_*.sh` (Lode CLI `rbld0_cli.sh`, dispatching across the rbld children, and the family's entry module `rbld0_lode.sh`.)
  - **RBLDS** → `Tools/rbk/rblds_spine.sh` (Lode capture-assembly spine — the data-driven Cloud Build composer shared by every Lode capture kind.)
  - **RBLDB** → `Tools/rbk/rbldb_*.sh` (Base-image body — ensconce, riding the spine.)
  - **RBLDL** → `Tools/rbk/rbldl_*.sh` (Lode lifecycle REST — `divine`/`banish`, direct GAR-REST host ops, distinct from the assembly spine.)
  - **RBLDR** → `Tools/rbk/rbldr_reliquary.sh` (Reliquary body — `conclave`, the build-tool date-cohort capture kind riding the spine)
  - **RBLDW** → `Tools/rbk/rbldw_underpin.sh` (Wsl body — `underpin`, the vendor WSL rootfs capture kind riding the spine)
  - **RBLDV** → `Tools/rbk/rbldv_immure.sh` (Podvm body — `immure`, the podman-machine disk capture kind riding the spine)
  - Reserved (legend only, no file — future Lode kind, letter matching the Lode GAR kind-letters): **RBLDT** tool
- **RBLM** → `Tools/rbk/rblm*` (Lifecycle Marshal — release-maintenance verbs, none of them a consumer's door. Its library `rblm_proscription.sh` holds the proscription: one table judging every enrolled regime field and hardpoint constant either site-scoped, meaning this station's own cloud and federated identity, or common to every installation. The `damnatio` theurge fixture reads that table to prove a delivered tree carries no site value.)
- **RBNNH** → `rbnnh_` prefix family — optional per-nameplate files in `rbmm_moorings/rben_<moniker>/`: `rbnnh_compose.yml` (Compose overlay fragment), `rbnnh_post_charge.sh` (post-charge hook script) and `rbnnh_charge_note.txt` (a note charge prints).
- **RBOB** → `Tools/rbk/rbob_bottle.sh`
- **RBOF** → `Tools/rbk/rbof_foedus.sh` (Foedus verbs on a standing foedus, colophon family `rbw-j`; founding and dissolving one stay the manor verbs affiance and jilt. `rbof_descry` (`rbw-jd`, read-only) reports a named foedus's health under the manor's workforce pool; `rbof_instate` (`rbw-jI`) points `RBRR_ACTIVE_FOEDUS` at it. CLI partner `rbof_cli.sh`. The `foedus-reuse` theurge fixture drives both.)
- **RBPC** → `Tools/rbk/rbpc_constants.sh` (Proving Constants — the freehold test rig's constants, chiefly the one federated subject its tests admit, which stands for the rig's whole life while the foedus and depot it is admitted to are re-made. The zipper's constant emit projects them to the theurge as `RBTGEC_FREEHOLD_*`.)
- **RBYC** → `Tools/rbk/rbyc_common.sh` (Common vocabulary — linked term constants for handbook yelp fragments)
- **RBQ**  → `Tools/rbk/rbq_qualify.sh` (Qualification orchestrator - tabtarget/colophon/nameplate health)
- **RBTD** → `Tools/rbk/rbtd/` (Theurge — crucible test orchestrator)
- **RBTE** → `Tools/rbk/rbtd/rbte_cli.sh` (Theurge test engine CLI entry point)
- **RBUH** → `Tools/rbk/rbuh_http.sh` (Utility HTTP — JSON REST, polling, shared temp-file machinery)

#### Moorings Filesystem Family (`rbm*_`)

The directory names of the consumer configuration tree, `rbmm_moorings/`, apart from the `Tools/rbk/` code files above.
`rbm` itself names nothing; its branches do:

- **`rbmm_`** → the moorings directory itself (`rbmm_moorings/`)
- **`rbmv_`** → vessels: one `rbev_<vessel>/` build context per vessel
- **`rbmf_`** → foedera: one `rbef_<foedus>/rbrf.env` per standing foedus (`rbmf_foedera/rbef_entrada/rbrf.env`), the active one named by `RBRR_ACTIVE_FOEDUS`. The federation regime is stored once here with no copied active file; `rbcc_rbrf_file_flint_capture` resolves the active foedus's `rbrf.env`.

Nameplates stand beside them, one `rben_<moniker>/` directory each.
Tabtargets dispatch through `tt/z-launcher.sh`, naming their launcher in the `BURD_LAUNCHER` config line as a bare `launcher.<id>_workbench.sh` basename that the trampoline resolves directly under `rbmm_moorings/buml_launchers/`.
That directory is BUK's, not a branch of `rbm`: the kit owns the launchers subdirectory (`BUBC_launchers_subdir`) and reads it under every consumer's moorings.
