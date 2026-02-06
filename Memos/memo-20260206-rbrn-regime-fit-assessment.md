# Assessment: axvr_*/axhr*_ Regime Annotation Patterns for RBRN

**Pace:** ₢ARAAQ (assess-rbrn-regime-fit)
**Heat:** ₣AR (rbw-ark-vessel-vocabulary)
**Date:** 2026-02-06
**Inputs:** RBRN, AXLA (with axvr_*/axhr*_ terms from ₢ARAAN), RBSRV (exemplar from ₢ARAAP)

---

## RBRN Structure Summary

RBRN has six feature groups with three distinct conditionality patterns:

| Section | Variables | Conditionality |
|---------|-----------|----------------|
| Core Service Identity | MONIKER, DESCRIPTION, RUNTIME | Unconditional |
| Ark Reference | SENTRY_VESSEL, SENTRY_CONSECRATION, BOTTLE_VESSEL, BOTTLE_CONSECRATION | Unconditional |
| Entry Service | ENTRY_ENABLED, ENTRY_PORT_WORKSTATION, ENTRY_PORT_ENCLAVE | Simple gate (ENTRY_ENABLED=1) |
| Enclave Network | ENCLAVE_BASE_IP, ENCLAVE_NETMASK, ENCLAVE_SENTRY_IP, ENCLAVE_BOTTLE_IP | Unconditional |
| Uplink | PORT_MIN, DNS_ENABLED, ACCESS_ENABLED, DNS_GLOBAL, ACCESS_GLOBAL, ALLOWED_CIDRS, ALLOWED_DOMAINS | Mixed: 5 unconditional + 2 compound-conditional |
| Volume Mount | VOLUME_MOUNTS | Unconditional |

---

## Q1: Do unconditional groups work with axhrgb_group without axhrgc_gate?

**YES — fully supported.**

AXLA's Regime Hierarchy Consistency rules (lines 2618-2624) explicitly address this:

> `axvr_group` without `axd_conditional`: corresponding `axhrgb_group` must have no `axhrgc_gate`

So in the parent document (RBAGS), define:
```asciidoc
[[rbrn_group_core]]
//axvr_group
{rbrn_group_core}::
Core identity variables for {rbrn_regime}.
```

Note: no `axd_conditional` dimension. Then in the RBRN subdocument:
```asciidoc
//axhrgb_group
{rbrn_group_core}

//axhrgv_variable
{rbrn_moniker}
...
```

No `axhrgc_gate` — the group is unconditional.

**This contrasts with RBSRV**, where both groups (Binding, Conjuring) are conditional and gated on `vessel_mode`. RBRN proves the unconditional path works.

---

## Q2: Does stacked axhrgc_gate handle RBRN's compound conditions?

**YES — mechanically sound, with a caveat on boolean typing (see Q3).**

RBRN's compound conditions:

- `ALLOWED_CIDRS`: Required when `ACCESS_ENABLED=1 AND ACCESS_GLOBAL=0`
- `ALLOWED_DOMAINS`: Required when `DNS_ENABLED=1 AND DNS_GLOBAL=0`

AXLA nesting rules (line 2017): "Multiple axhrgc_gate within one group have AND semantics"

Proposed structure for CIDRS (assuming boolean→enum conversion per Q3):

```asciidoc
//axhrgb_group
{rbrn_group_access_allowlist}

//axhrgc_gate
{rbrn_uplink_access_enabled} {rbrn_uplink_access_enabled_on}

//axhrgc_gate
{rbrn_uplink_access_global} {rbrn_uplink_access_global_off}

//axhrgv_variable
{rbrn_uplink_allowed_cidrs}
```

The stacked gates read: "this group is active when access_enabled=on AND access_global=off."

**Key insight:** The two compound conditions (CIDRS vs DOMAINS) use DIFFERENT gate variables, so they need SEPARATE groups — you cannot combine them into one group.

---

## Q3: Boolean variables as gates — axt_boolean vs axt_enumeration?

**axhrgc_gate REQUIRES axt_enumeration. Booleans must be promoted.**

The axhrgc_gate definition (AXLA line 1983-1984) explicitly states:
> "the enumerated variable (voicing axt_enumeration) and the specific axt_enum_value that activates this gate"

RBRN has **5 boolean flags** that serve as gate variables:

| Current Variable | Current Type | Role |
|------------------|-------------|------|
| RBRN_ENTRY_ENABLED | crg_atom_bool | Gates Entry group |
| RBRN_UPLINK_DNS_ENABLED | crg_atom_bool | Gates DNS allowlist |
| RBRN_UPLINK_ACCESS_ENABLED | crg_atom_bool | Gates access allowlist |
| RBRN_UPLINK_DNS_GLOBAL | crg_atom_bool | Gates DNS allowlist (compound) |
| RBRN_UPLINK_ACCESS_GLOBAL | crg_atom_bool | Gates access allowlist (compound) |

Each must become an enumeration with explicit values. Term proliferation:

- **Per boolean → 3 terms**: variable + on-value + off-value
- **5 booleans × 3 = 15 terms** (vs 5 today)

### Recommendation: Standard binary enumeration pattern

Rather than ad-hoc naming, establish a convention:
- `{rbrn_entry_enabled}` voices `axt_enumeration`
- `{rbrn_entry_enabled_on}` voices `axt_enum_value` (maps to `1`)
- `{rbrn_entry_enabled_off}` voices `axt_enum_value` (maps to `0`)

The `_on`/`_off` suffix convention is consistent and self-documenting. The underlying bash `.env` values remain `1` and `0` — only the AXLA vocabulary changes.

### Alternative considered and rejected

Extending `axhrgc_gate` to accept `axt_boolean` would reduce proliferation but violates the gate's semantic contract (gates select among enumerated alternatives, not binary states). The boolean-as-enum approach is more principled: even two-valued choices are choices.

---

## Q4: RBRN patterns that expose gaps in axhr*_ design

### Gap 1: Mixed conditionality within a logical section

RBRN's "Uplink" section contains:
- 5 unconditional variables (PORT_MIN, DNS_ENABLED, ACCESS_ENABLED, DNS_GLOBAL, ACCESS_GLOBAL)
- 2 variables with different compound conditions (ALLOWED_CIDRS, ALLOWED_DOMAINS)

This doesn't map to a single group. The required structure:

```
Ungrouped:   rbrn_uplink_port_min
Ungrouped:   rbrn_uplink_dns_enabled
Ungrouped:   rbrn_uplink_access_enabled
Ungrouped:   rbrn_uplink_dns_global
Ungrouped:   rbrn_uplink_access_global
Group A:     rbrn_group_access_allowlist  (gated: access_enabled=on AND access_global=off)
  Variable:  rbrn_uplink_allowed_cidrs
Group B:     rbrn_group_dns_allowlist     (gated: dns_enabled=on AND dns_global=off)
  Variable:  rbrn_uplink_allowed_domains
```

**This is valid but breaks the RBRN document's current visual grouping.** The original RBRN presents "Uplink Configuration" as one section. Under axhr, the 5 unconditional vars become ungrouped (`axhrv_variable`), and the 2 conditional vars get their own separate groups.

**Verdict:** Not a gap per se — the system works. But it reveals that axhr hierarchies decompose around conditionality, not around human-readable sections. Document authors need to accept that the hierarchy structure may not mirror the original document's visual layout.

### Gap 2: Single-variable conditional groups

Both `rbrn_group_access_allowlist` and `rbrn_group_dns_allowlist` contain exactly ONE variable each. Creating a full group+gate structure for a single variable feels heavyweight.

**Verdict:** Livable. The group exists because the conditionality exists. A group with one variable is degenerate but not incorrect. This is a consequence of Q5's trade-off.

### Gap 3: Boolean-to-enumeration proliferation (see Q3)

5 boolean gate variables × 3 terms each = significant vocabulary growth. This is a real cost.

**Verdict:** Accepted cost. The alternative (extending the gate mechanism) is worse.

---

## Q5: Would variable-level axd_conditional help?

**Variable-level conditionality would reduce verbosity for RBRN's Uplink pattern but is NOT recommended at this time.**

### What it would look like

A hypothetical `axhrv_variable` with inline gate:
```asciidoc
//axhrv_variable axd_conditional(rbrn_uplink_access_enabled rbrn_uplink_access_enabled_on, rbrn_uplink_access_global rbrn_uplink_access_global_off)
{rbrn_uplink_allowed_cidrs}
```

This eliminates the single-variable group wrapper.

### Why not now

1. **RBSRV didn't need it** — groups worked naturally there
2. **RBRN is the first regime to expose the pattern** — premature to generalize from one case
3. **Adds parser complexity** — gates are currently positional (next N attribute references); variable-level gates need different syntax
4. **Groups have value even with one variable** — they name the concept ("access allowlist") which aids readability

### Recommendation

Apply the group-level approach for RBRN now. If a third regime reveals the same single-variable-conditional pattern, revisit variable-level conditionality as a refinement.

---

## Summary: What works, what needs adjustment, recommendations

### Works as-is
- Unconditional groups (axhrgb_group without axhrgc_gate) ✓
- Stacked AND gates for compound conditions ✓
- Ungrouped variables (axhrv_variable) for truly independent vars ✓
- Cross-document validation rules apply cleanly ✓

### Needs adjustment before RBRN application
- **Boolean gate variables → enumerations**: 5 variables must be retyped. Establish `_on`/`_off` naming convention.
- **RBAGS parent document**: Needs `rbrn_*` voicing annotations (axvr_regime, axvr_variable, axvr_group) matching the new structure.
- **Type voicings**: RBRN's CRR types (crg_atom_bool, etc.) need mapping to RBAGS type voicings (rbst_*) or AXLA types.

### Recommended RBRN group structure

| axhr Structure | Variables | Conditionality |
|----------------|-----------|----------------|
| Ungrouped (axhrv_variable) | moniker, description, runtime | — |
| Group: rbrn_group_ark_reference | sentry_vessel, sentry_consecration, bottle_vessel, bottle_consecration | Unconditional |
| Group: rbrn_group_entry | entry_port_workstation, entry_port_enclave | Conditional: entry_enabled=on |
| Ungrouped (axhrv_variable) | entry_enabled | — (it's the gate, not gated) |
| Ungrouped (axhrv_variable) | enclave_base_ip, enclave_netmask, enclave_sentry_ip, enclave_bottle_ip | — |
| Ungrouped (axhrv_variable) | uplink_port_min, dns_enabled, access_enabled, dns_global, access_global | — |
| Group: rbrn_group_access_allowlist | uplink_allowed_cidrs | Conditional: access_enabled=on AND access_global=off |
| Group: rbrn_group_dns_allowlist | uplink_allowed_domains | Conditional: dns_enabled=on AND dns_global=off |
| Ungrouped (axhrv_variable) | volume_mounts | — |

**Note:** Gate variables (entry_enabled, dns_enabled, access_enabled, dns_global, access_global) are ungrouped because they're always required — they ARE the enablers, not the enabled.

### Design consideration: ungrouped enclave variables

An alternative is to make "enclave" an unconditional group. Whether to group or leave ungrouped depends on whether "enclave network configuration" is a meaningful sub-concept of the nameplate regime worth naming, or just a collection of related-but-independent variables. Both approaches are valid under axhr. Recommend grouping if RBAGS will define the concept, ungrouped if they're just implementation details.

---

## Next steps (for future paces)

1. Mint `rbrn_*` terms in RBAGS (voicing annotations for regime, variables, groups, enum values)
2. Mint `rbst_*` type voicings if not already covering RBRN's types
3. Create/retrofit RBRN subdocument (or RBRN itself) with axhr hierarchy markers
4. Boolean → enumeration conversion for 5 gate variables
