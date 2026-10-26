// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary
//
// Author: Brad Hyslop <bhyslop@scaleinvariant.org>
//
// RBVCEW — the world election (RBSHC "The election"). Every command conducts
// in one of two worlds — the agora, the real world, or the palaestra, the
// practice world (RBS0 rbshc_agora, rbshc_palaestra) — elected by the one
// explicit argument each command takes, with no default.
//
// The election is single and whole: it selects one bundle, every address and
// working-directory name the ceremony reaches, and every act reads the elected
// bundle and nothing past it. The world-specific constants therefore live here
// and nowhere else in the crate; a second copy of any of them beside its
// consumer is how a practice run reaches the real public repository.
//
// The two words are cited through the breviary, never spelled: a string that
// spells a governed word in its own bytes has left the word's home.

use crate::rbvgeb_breviary;

/// The elected world.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum rbvcew_World {
    Agora,
    Palaestra,
}

/// Everything a world fixes. Both bundles are complete; no field of one is
/// ever consulted under the other, so no mixture of the two can be assembled.
pub struct rbvcew_Bundle {
    /// The base remote's name in the maintainer repository. The cut clones
    /// the world's public repository through it and cuts the regulus one
    /// commit atop its live main. It is read (cloned) and never pushed to, and
    /// the clone's origin is severed after cloning so the name cannot be
    /// pushed to even by accident. Loud on purpose (_UPSTREAM, all caps): it
    /// can never be confused with origin, main, or a hand-typed default.
    pub base_remote: &'static str,

    /// The base is READ-ONLY, and that is asserted, not merely intended. A
    /// push-capable remote to a public target in the maintainer repo is the
    /// one catastrophe the ceremony forbids, so the base remote's PUSH url
    /// must be neutered to this sentinel, and the cut refuses until it is. The
    /// clone still reads (fetch) the real URL; only the push side is dead.
    /// Each sentinel spells its own remote's name.
    pub base_push_disabled: &'static str,

    /// The base's expected fetch URL, hardcoded and asserted before anything
    /// is cloned. The base-inventory sweep is NON-FATAL by design — it
    /// tolerates already-disclosed withheld history because the base is the
    /// genuinely public repository (or, in the practice world, a stand-in
    /// seeded from it). That tolerance is safe ONLY if the base truly IS that
    /// repository: a fetch URL fat-fingered at the private maintainer repo
    /// would clone the whole private history, the inventory would wave it
    /// through as "already disclosed", the delta sweep would see only the
    /// clean one commit, and the regulus would green-light — then a push
    /// would upload the entire private ancestry (the 292 MiB catastrophe,
    /// back through the one unlocked door). The endpoint is a load-bearing
    /// fact, not a runtime input. The kerygma and the promotion target it too.
    pub base_url: &'static str,

    /// The ephemeral private narthex (RBS0 rbshc_narthex): created empty and
    /// private by the operator's own hand before a cut, reached only by
    /// explicit URL — never a configured remote. Fixed and known, unlike the
    /// repository's ephemeral CONTENTS: only its existence is per-cycle. The
    /// single home for docimasy and ostend alike — both gate on the same
    /// repository, and a duplicated copy is exactly the drift a
    /// privacy/freshness gate must never carry (RBSHD "Gate the narthex",
    /// RBSHO "Re-assert the ground").
    pub narthex_url: &'static str,

    /// The anonymous-read form of the same repository, for the 404 privacy
    /// gate. A private GitHub repository 404s to an unauthenticated request; a
    /// public or misnamed one does not.
    pub narthex_https: &'static str,

    /// The URL the harbinger clones (RBSHH). In the real world the anonymous
    /// HTTPS face a stranger lands on — distinct from base_url, which is the
    /// cut's read-only base remote, not a clone target. The practice world's
    /// stand-in is private and cannot be cloned anonymously, so its harbinger
    /// reads it over SSH: the fidelity gap the cosmology accepts.
    pub harbinger_url: &'static str,

    /// The fixed regulus parent directory, a sibling of the maintainer repo.
    /// Not dated: essai runs the lap repeatedly at one conventional location
    /// and seposes any prior, so the location is memorable and the disposal
    /// is safe. Expede builds the clone one level down, at
    /// {parent}/{RBVCEP_REGULUS_SUBDIR}. A conserved spelling, declared in the
    /// cosmology's cadaster: a later run finds an earlier run's location by
    /// these exact characters.
    pub regulus_dirname: &'static str,

    /// The fixed parent directory for the freshness matcher's scratch re-cut
    /// (RBSHD step 2 / RBSHO step 2, RBSHC single-matcher rule), a sibling of
    /// the maintainer repo distinct from regulus_dirname so the freshness
    /// check can never collide with, or dispose of, the standing regulus it
    /// is comparing against. A conserved spelling, declared in the cadaster.
    pub freshness_dirname: &'static str,

    /// The temenos's parent-relative dirname, a fixed basename beside the
    /// maintainer repo. Shared by essai and harbinger — they never stand
    /// simultaneously, so one location per world, any prior seposed. A
    /// conserved spelling, declared in the cadaster; the real world's is the
    /// same directory the bash harbinger stands its rig in.
    pub temenos_dirname: &'static str,
}

/// The real world: the live public repository, the standing narthex, and the
/// conserved rbthdr_ working directories.
pub const RBVCEW_AGORA: rbvcew_Bundle = rbvcew_Bundle {
    base_remote: "ENGROSSMENT_UPSTREAM",
    base_push_disabled: "DISABLED-ENGROSSMENT_UPSTREAM-IS-READ-ONLY",
    base_url: "git@github.com:scaleinv/recipebottle.git",
    narthex_url: "git@github.com:scaleinv/recipebottle-staging.git",
    narthex_https: "https://github.com/scaleinv/recipebottle-staging",
    harbinger_url: "https://github.com/scaleinv/recipebottle.git",
    regulus_dirname: "rbthdr_regulus",
    freshness_dirname: "rbthdr_regulus_freshness",
    temenos_dirname: "rbthdr_coldwalk",
};

/// The practice world: a private stand-in for the public repository, seeded
/// from live public main, with its own private narthex and working
/// directories apart from the real ones. The two repositories are the
/// operator's own, created and destroyed by hand at the console; no tooling
/// creates or destroys either.
pub const RBVCEW_PALAESTRA: rbvcew_Bundle = rbvcew_Bundle {
    base_remote: "PALAESTRA_UPSTREAM",
    base_push_disabled: "DISABLED-PALAESTRA_UPSTREAM-IS-READ-ONLY",
    base_url: "git@github.com:scaleinv/recipebottle-palaestra.git",
    narthex_url: "git@github.com:scaleinv/recipebottle-palaestra-staging.git",
    narthex_https: "https://github.com/scaleinv/recipebottle-palaestra-staging",
    harbinger_url: "git@github.com:scaleinv/recipebottle-palaestra.git",
    regulus_dirname: "palaestra_regulus",
    freshness_dirname: "palaestra_regulus_freshness",
    temenos_dirname: "palaestra_coldwalk",
};

/// The elected world's bundle — the one road from a world to an address.
pub fn rbvcew_bundle(world: rbvcew_World) -> &'static rbvcew_Bundle {
    match world {
        rbvcew_World::Agora => &RBVCEW_AGORA,
        rbvcew_World::Palaestra => &RBVCEW_PALAESTRA,
    }
}

/// The world's word, as the argument spells it and the cachet records it.
pub fn rbvcew_word(world: rbvcew_World) -> &'static str {
    match world {
        rbvcew_World::Agora => rbvgeb_breviary::RBVGEB_AGORA_BASE,
        rbvcew_World::Palaestra => rbvgeb_breviary::RBVGEB_PALAESTRA_BASE,
    }
}

/// The world's word as every section banner prints it — loud.
pub fn rbvcew_banner(world: rbvcew_World) -> &'static str {
    match world {
        rbvcew_World::Agora => rbvgeb_breviary::RBVGEB_AGORA_BASE_VERSAL,
        rbvcew_World::Palaestra => rbvgeb_breviary::RBVGEB_PALAESTRA_BASE_VERSAL,
    }
}

/// The world a word names, exactly as spelled — no case folding, no prefix,
/// no default.
pub fn rbvcew_from_word(word: &str) -> Option<rbvcew_World> {
    match word {
        rbvgeb_breviary::RBVGEB_AGORA_BASE => Some(rbvcew_World::Agora),
        rbvgeb_breviary::RBVGEB_PALAESTRA_BASE => Some(rbvcew_World::Palaestra),
        _ => None,
    }
}

/// Elect the world from the arguments that follow a command: exactly one,
/// naming a world. Pure — the self-proofs reach it, which main cannot be. A
/// bare invocation, an unknown word, and a trailing extra argument each
/// refuse: the world is the ceremony's most dangerous input, so nothing
/// defaults toward either world (RCG Interface Contamination Discipline).
pub fn rbvcew_elect(command: &str, args: &[String]) -> Result<rbvcew_World, String> {
    let agora = rbvgeb_breviary::RBVGEB_AGORA_BASE;
    let palaestra = rbvgeb_breviary::RBVGEB_PALAESTRA_BASE;
    let conductor = rbvgeb_breviary::RBVGEB_HIEROPHANT_BASE;
    let word = match args {
        [] => {
            return Err(format!(
                "{} {}: no world named — elect {} or {}; there is no default",
                conductor, command, agora, palaestra
            ))
        }
        [word] => word.as_str(),
        [_, extra, ..] => {
            return Err(format!(
                "{} {}: unexpected extra argument '{}' after the world",
                conductor, command, extra
            ))
        }
    };
    rbvcew_from_word(word).ok_or_else(|| {
        format!(
            "{} {}: unknown world '{}' — elect {} or {}",
            conductor, command, word, agora, palaestra
        )
    })
}
