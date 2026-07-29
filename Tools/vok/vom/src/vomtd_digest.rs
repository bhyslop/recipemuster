// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Tests for vomrd_digest - the read-only audit of curated acronym digests
//! (VOSMM "Report and Cadastre" Done-when): a dead cipher-prefixed row is
//! flagged, a live row is not, a non-cipher head is out of jurisdiction, and
//! the digests are never mutated (the audit is pure over census + corpus).

use std::path::Path;

use super::vomrb_builder::vomrb_Builder;
use super::vomrd_digest::{vomrd_audit, vomrd_is_digest};

fn zvomtd_census(corpus: &[(String, String)]) -> super::vomrm_matricula::vomrm_Matricula {
    let mut builder = vomrb_Builder::vomrb_raise();
    builder.vomrb_seat_corpus(corpus);
    builder.vomrb_seal()
}

#[test]
fn vomtd_dead_cipher_row_presents_and_live_row_does_not() {
    // Census seats only rbtt_probe; the digest also claims a vanished RBGONE.
    let census = zvomtd_census(&[(
        "Tools/rbk/rbtt_probe.sh".to_string(),
        "rbtt_probe() {\n:\n}\n".to_string(),
    )]);
    let digests = vec![(
        "Tools/rbk/claude-rbk-acronyms.md".to_string(),
        "- **RBTT** → `rbk/rbtt_probe.sh`\n- **RBGONE** → `rbk/rbgone_lost.sh`\n".to_string(),
    )];

    let findings = vomrd_audit(&census, &digests);
    assert_eq!(findings.len(), 1, "exactly the dead row presents");
    assert_eq!(findings[0].inscriptions, vec!["RBGONE".to_string()]);
    assert_eq!(findings[0].sites[0].file, "Tools/rbk/claude-rbk-acronyms.md");
    assert_eq!(findings[0].sites[0].line, 2);
    assert!(findings[0].advisory, "row-liveness is a fallible gate (VOr_m7w)");
}

#[test]
fn vomtd_non_cipher_head_is_out_of_jurisdiction() {
    // RCG/BCG are guide acronyms outside every registered cipher: skipped,
    // never flagged, even against an empty census.
    let census = zvomtd_census(&[]);
    let digests = vec![(
        "Tools/buk/claude-buk-veiled-acronyms.md".to_string(),
        "- **RCG** → `jjqs_studbook/guides/vok/RCG-RustCodingGuide.md`\n- **BCG** → `jjqs_studbook/guides/buk/BCG-BashConsoleGuide.md`\n".to_string(),
    )];

    assert!(vomrd_audit(&census, &digests).is_empty());
}

#[test]
fn vomtd_family_head_is_live_through_any_seated_member() {
    // A family head (RBTT) with no signet of its own exact spelling is
    // announced live by a seated member extending it.
    let census = zvomtd_census(&[(
        "Tools/rbk/rbtt_probe.sh".to_string(),
        "rbtt_probe() {\n:\n}\n".to_string(),
    )]);
    let digests = vec![(
        "Tools/rbk/claude-rbk-acronyms.md".to_string(),
        "- **RBTT** → `rbk/rbtt_probe.sh`\n".to_string(),
    )];

    assert!(vomrd_audit(&census, &digests).is_empty());
}

#[test]
fn vomtd_veiled_homed_row_is_out_of_jurisdiction() {
    // A non-veiled digest curating a row whose home is veiled: the home sits
    // outside the census walk, so the row is skipped, not falsely dead.
    let census = zvomtd_census(&[]);
    let digests = vec![(
        "Tools/vok/claude-vok-acronyms.md".to_string(),
        "- **VOS0** → `vok/vov_veiled/VOS0-VoxObscuraSpec.adoc`\n".to_string(),
    )];

    assert!(vomrd_audit(&census, &digests).is_empty());
}

#[test]
fn vomtd_non_row_lines_are_ignored() {
    let census = zvomtd_census(&[]);
    let digests = vec![(
        "Tools/rbk/claude-rbk-acronyms.md".to_string(),
        "## File Acronym Mappings\n\nProse about **RBGONE** stays prose.\n".to_string(),
    )];

    assert!(vomrd_audit(&census, &digests).is_empty());
}

#[test]
fn vomtd_discovery_takes_acronym_digests_and_skips_veiled() {
    assert!(vomrd_is_digest(Path::new("Tools/rbk/claude-rbk-acronyms.md")));
    assert!(vomrd_is_digest(Path::new("Tools/buk/claude-buk-acronyms.md")));
    // Veiled digests point at homes the census walk excludes.
    assert!(!vomrd_is_digest(Path::new(
        "Tools/rbk/vov_veiled/claude-rbk-veiled-acronyms.md"
    )));
    // Prose peers are not digests.
    assert!(!vomrd_is_digest(Path::new("Tools/rbk/claude-rbk-core.md")));
    assert!(!vomrd_is_digest(Path::new("CLAUDE.md")));
}

// eof
