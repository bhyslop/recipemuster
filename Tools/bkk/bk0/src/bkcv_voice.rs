// Copyright 2026 Scale Invariant, Inc.
// SPDX-License-Identifier: Apache-2.0
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.
//! The voice — the tenant's streams consumed live and rendered as the door's
//! own: the heartbeat, one line per failing case, and the verdict.
//!
//! TWO DESTINATIONS, AND THE DIAL MOVES ONLY ONE. What goes onto the console is
//! the voice: a liveness mark while the run proceeds, a bounded line per
//! failure, and a verdict. What goes into the record is the child's WHOLE
//! stream interleaved with the door's own marks. The verbosity dial moves the
//! console and never the record, and the liveness mark is never written to the
//! record at all — a heartbeat is a fact about a terminal watching, and a
//! re-read log has nothing to be live about.
//!
//! THE RECORD IS THE SUBSTRATE'S LOG FAMILY AND THE KENNEL WRITES IT. A kennel
//! tabtarget declares the own-log mode, so the dispatch composes the three
//! names, writes them into its file, and creates nothing; all three members are
//! written here or none is. The three curation dicta are implemented from the
//! substrate's own words rather than by shelling to its curators: the kennel is
//! a coordinator that owns its stream, and a record assembled by calling back
//! into bash would put the mode's whole point behind the thing it replaces.
//!
//! WHY THE COUNTS ARE THE KENNEL'S AND THE VERDICT IS THE EXIT CODE. Parsing
//! yields the per-case lines and nothing else. A runner's summary is prose that
//! changes between versions, and the standard harness prints one summary PER
//! TEST BINARY — so a door reading a summary for its verdict would report the
//! last binary's opinion of the whole suite. The exit code is the verdict
//! because the kennel owns the invocation and no pipe stands in the way.

use crate::bkco_output;
use std::io::Write;

/// The three members the dispatch composed and wrote into its file. Composed by
/// nobody here, per the family's law that a writer downstream of dispatch writes
/// to the paths it was handed. The historical member stands in the file wherever
/// a log exists; the last and same members stand there under the own-log mode
/// alone, so the PAIR is the mode's declaration.
pub const BKCV_LOG_LAST_VAR: &str = "BURD_LOG_LAST";
pub const BKCV_LOG_SAME_VAR: &str = "BURD_LOG_SAME";
pub const BKCV_LOG_HIST_VAR: &str = "BURD_LOG_HIST";

/// The position line the historical member's second line carries, and the
/// ephemeral directory the normalized member replaces.
pub const BKCV_GIT_CONTEXT_VAR: &str = "BURD_GIT_CONTEXT";
pub const BKCV_TEMP_DIR_VAR: &str = "BURD_TEMP_DIR";

/// The dispatch's two readings of one instant: a LOCAL stamp and the epoch
/// seconds, taken from a single `date` call and written together.
pub const BKCV_NOW_STAMP_VAR: &str = "BURD_NOW_STAMP";
pub const BKCV_NOW_EPOCH_VAR: &str = "BURD_NOW_EPOCH";

/// The position line a record names where the dispatch wrote no git context.
pub(crate) const ZBKCV_GIT_UNAVAILABLE: &str = "git-unavailable";

/// The literal the normalized member puts where the ephemeral directory stood.
const ZBKCV_EPHEMERAL: &str = "BURD_EPHEMERAL_DIR";

/// The marker whose line the normalized member drops whole.
const ZBKCV_VOLATILE: &str = "VOLATILE";

/// The digest line that closes the historical member.
const ZBKCV_CHECKSUM_LEAD: &str = "Same log checksum: ";

/// Whether this process has already opened the log family once.
///
/// PROCESS-WIDE AND NOT PER DOOR, for the reason the open states: what must
/// happen once is the TRUNCATION of a family the dispatch composed for the
/// whole invocation, and only the process knows whether that has happened yet.
static ZBKCV_OPENED: std::sync::atomic::AtomicBool = std::sync::atomic::AtomicBool::new(false);

/// How often the voice marks that it is alive, absent anything to say.
pub const BKCV_CADENCE: std::time::Duration = std::time::Duration::from_secs(10);

/// What the kennel counted for itself while the child spoke.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct bkcv_Tally {
    /// Cases the recognizer saw reported, whatever their outcome.
    pub ran: usize,
    /// Cases reported as having passed.
    pub passed: usize,
    /// The names of the cases reported as having failed, in the order seen.
    pub failed: Vec<String>,
}

/// One case as a recognizer read it off the child's stream.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct bkcv_Case {
    /// The case's full name as the runner spelled it.
    pub name: String,
    /// Whether the runner reported it as having passed.
    pub passed: bool,
}

/// The output shapes the kennel recognizes, as a collar's tongue field declares
/// them. The collar says which; nothing here is inferred from the runner, a
/// runner being askable for an output shape other than its own.
pub const BKCV_TONGUE_NEXTEST: &str = "bknre_nextest";
pub const BKCV_TONGUE_HARNESS: &str = "bknre_harness";

/// The python family's tongue, CITED FROM THE FAMILY THAT DECLARES IT rather
/// than spelled again here. The two rust tongues are spelled twice across this
/// crate already — once where a runner is chosen and once where its shape is
/// read — and the estate tolerates that because uniqueness lives in the value
/// with its owning variable. It is not worth a third: the python family's
/// roster is the one a collar is validated against, so a spelling that drifted
/// from it would admit a tongue no collar could declare.
pub const BKCV_TONGUE_PYTEST: &str = crate::bkcx_python::BKCX_TONGUE_PYTEST;

/// Read one line of a NEXTEST run and answer the case it reports, if it reports
/// one.
///
/// ---- PALISADE MEMBRANE ----
///
/// FOREIGN SIGNATURE. cargo-nextest renders each finished case as a status word,
/// a bracketed duration, a parenthesised ordinal, the binary id, and the case's
/// full name:
///
/// ```text
///         PASS [   0.003s] ( 1/24) vof vofc_registry::tests::vofc_prefix_matching
/// ```
///
/// The status word is what this reads; the ordinal and the duration are stepped
/// over without being interpreted, and the binary id and name are rejoined as
/// the case's spelling.
///
/// WHY A MEMBRANE AT ALL. This is console prose, not a contract. Nextest offers
/// a machine-readable form, but it is behind an unstable flag at the version the
/// estate pins, so reading the rendered line is the honest option and the bend
/// is contained here.
///
/// ABSORB ONLY THE SURVEYED SIGNATURE. A line whose first word is not one of the
/// statuses below is not a case and yields nothing — no guessing from shape, no
/// falling back on position. The summary line is deliberately outside the
/// signature: the verdict is the exit code and never a parse.
///
/// RETIREMENT CONDITION. Retire this membrane when nextest's machine-readable
/// output leaves unstable and the kennel can ask for it by a supported flag.
/// The captured log this was written against stands at
/// `hist-bkmw-m-sh-20260905-195448-1382748-22.txt`, a green drive of `suite-vof`
/// taken at seat 0d2ba2bb16998a435150dc08e83c7df3f474b876; compare a future
/// nextest's rendering against it before trusting this reader.
fn zbkcv_nextest_case(line: &str) -> Option<bkcv_Case> {
    let mut words = line.split_whitespace();
    let status = words.next()?;

    let passed = match status {
        "PASS" => true,
        "FAIL" | "TIMEOUT" | "SIGSEGV" | "ABORT" => false,
        // LEAK is a passing case whose process left something behind, and SLOW
        // is a progress notice about a case still running rather than a report
        // of one finished. Neither is a verdict and both are stepped over: a
        // SLOW counted as a case would count the same case twice when it later
        // reports for real.
        _ => return None,
    };

    // The duration and the ordinal are stepped over as SHAPE rather than read.
    // They are what tells a case line from a stray line beginning with the same
    // word, so their presence is checked and their content is not.
    let duration = words.next()?;
    if !duration.starts_with('[') {
        return None;
    }
    // The bracket may close on the same word or a later one, the duration being
    // right-aligned inside it.
    let mut closed = duration.ends_with(']');
    while !closed {
        closed = words.next()?.ends_with(']');
    }

    let ordinal = words.next()?;
    if !ordinal.starts_with('(') {
        return None;
    }
    let mut counted = ordinal.ends_with(')');
    while !counted {
        counted = words.next()?.ends_with(')');
    }

    let rest: Vec<&str> = words.collect();
    if rest.is_empty() {
        return None;
    }

    Some(bkcv_Case {
        name: rest.join(" "),
        passed,
    })
}

/// Read one line of a STANDARD HARNESS run and answer the case it reports, if it
/// reports one.
///
/// ---- PALISADE MEMBRANE ----
///
/// FOREIGN SIGNATURE. libtest renders each finished case as the word `test`, the
/// case's full name, an ellipsis, and an outcome word:
///
/// ```text
/// test butw_the_whistle_refuses_an_uncommitted_seat_and_names_the_remedy ... ok
/// ```
///
/// THE SUMMARY LINE WEARS THE SAME FIRST WORD and must not be read as a case:
///
/// ```text
/// test result: ok. 1 passed; 0 failed; 0 ignored; ...
/// ```
///
/// It is excluded by the separator rather than by pattern-matching its text —
/// a summary carries no ` ... `, so the exclusion holds however the summary's
/// own wording changes.
///
/// WHY A MEMBRANE AT ALL. libtest's JSON output is nightly-only, so at the
/// channel this estate pins there is no stable machine-readable form to render
/// from and the console text is what there is.
///
/// ABSORB ONLY THE SURVEYED SIGNATURE. `ignored` is neither passed nor failed
/// and is not a case this counts: the kennel has no skip, so a case the runner
/// declined to run is not a case that ran. It is stepped over here and the
/// narrowing is the collar's to declare.
///
/// ONE SUMMARY PER TEST BINARY, which is the reason this reader yields cases and
/// never a tally. A harness run over a manifest launches one binary per target
/// and each prints its own `test result:` line, so a door that took a summary
/// for the run's verdict would report the last binary's opinion of all of them.
///
/// RETIREMENT CONDITION. Retire this membrane when libtest's structured output
/// is stable on the pinned channel. The captured log this was written against
/// stands at `hist-bkmw-m-sh-20260905-195642-1406080-666.txt`, a green drive of
/// `suite-buk` at seat 8af0e16658dc8d44543bff57c90f021621343148, with the red
/// drive that shaped the failure half at
/// `hist-bkmw-m-sh-20260905-195457-1385227-410.txt`.
fn zbkcv_harness_case(line: &str) -> Option<bkcv_Case> {
    let body = line.strip_prefix("test ")?;

    // THE SEPARATOR IS WHAT PARTS A CASE FROM THE SUMMARY, and it is sought from
    // the RIGHT: a case name cannot contain it, but seeking from the left would
    // still be a guess about a name, and the outcome is always last.
    let (name, outcome) = body.rsplit_once(" ... ")?;

    let passed = match outcome.trim() {
        "ok" => true,
        "FAILED" => false,
        // `ignored` and the bench forms are outside the surveyed signature.
        _ => return None,
    };

    if name.trim().is_empty() {
        return None;
    }

    Some(bkcv_Case {
        name: name.trim().to_string(),
        passed,
    })
}

/// Read one line of a PYTEST run and answer the case it reports, if it reports
/// one.
///
/// ---- PALISADE MEMBRANE ----
///
/// FOREIGN SIGNATURE. Under its verbose flag pytest renders each finished case
/// as the node id, an outcome word in capitals, and a right-aligned percentage
/// in brackets:
///
/// ```text
/// tests/test_alpha.py::test_one PASSED                                     [ 25%]
/// tests/test_beta.py::test_red FAILED                                      [100%]
/// ```
///
/// THE SHORT SUMMARY WEARS THE SAME WORDS IN THE OTHER ORDER and must not be
/// read as a second report of the same case:
///
/// ```text
/// FAILED tests/test_beta.py::test_red - assert 1 == 2
/// ```
///
/// It is excluded by the ORDER rather than by matching its text — the outcome
/// stands second here and first there — so the exclusion holds however the
/// summary's own wording changes. That parting is load-bearing: pytest prints
/// the summary for every failing case, so a reader that took both would count
/// each failure twice and report a tally its own runner disagrees with.
///
/// THE PERCENTAGE IS CHECKED AS SHAPE AND NEVER READ. It is what tells a case
/// line from a stray line carrying a capitalized word, so its presence is
/// required and its content is not.
///
/// WHY A MEMBRANE AT ALL. This is console prose, not a contract. pytest offers
/// machine-readable output only through a plugin the kennel does not
/// provision — and the kennel provisions nothing it spawns — so the rendered
/// line is the honest option and the bend is contained here.
///
/// WHY THE RUN IS SPELLED VERBOSE AT ALL. pytest's default output is a row of
/// dots carrying no name, so a tally taken from it could count cases but never
/// name a failing one — and the bounded failure line, which is the whole of what
/// the quiet flavor says, has nothing to print. The verbose flag is what makes a
/// python course readable by the same reader the two rust tongues answer to
/// (`bkci_pipeline::bkci_pytest_listing` carries the listing's own half).
///
/// ABSORB ONLY THE SURVEYED SIGNATURE. `SKIPPED`, `XFAIL` and `XPASS` are
/// stepped over on the harness membrane's own ruling: the kennel has no skip, so
/// a case the runner declined to run is not a case that ran, and an expectation
/// about a failure is not a verdict on one. `ERROR` IS COUNTED AS A FAILURE
/// rather than stepped over — it is pytest's word for a case whose setup died,
/// which is a case that did not pass, and dropping it would let a suite whose
/// fixtures collapsed report as having run nothing.
///
/// RETIREMENT CONDITION. Retire this membrane when the kennel renders from a
/// machine-readable pytest report it can ask for without provisioning a plugin.
/// The captured log this was written against stands at
/// `hist-bkmw-k-sh-20260910-145316-1350227-453.txt`, a mixed green-and-red drive
/// of the capture lure at seat ace9b08ec84a229f4c57372371c514705c6ad789, which
/// carries the four case lines above and the short summary beneath them.
fn zbkcv_pytest_case(line: &str) -> Option<bkcv_Case> {
    let trimmed = line.trim_end();

    // THE PERCENTAGE IS SOUGHT FROM THE RIGHT, the node id being free to carry
    // almost anything and the bracket never being part of one.
    let closed = trimmed.strip_suffix("%]")?;
    let opened = closed.rfind('[')?;

    let body = trimmed[..opened].trim_end();

    // The ordinal's interior is shape and is checked as such: a right-aligned
    // integer and nothing else.
    let percent = &closed[opened + 1..];
    if percent.trim().is_empty() || !percent.trim().chars().all(|one| one.is_ascii_digit()) {
        return None;
    }

    let (name, outcome) = body.rsplit_once(char::is_whitespace)?;

    let passed = match outcome.trim() {
        "PASSED" => true,
        "FAILED" | "ERROR" => false,
        // Outside the surveyed signature, on the ruling stated above.
        _ => return None,
    };

    let name = name.trim();
    if name.is_empty() {
        return None;
    }

    Some(bkcv_Case {
        name: name.to_string(),
        passed,
    })
}

/// Whether a tongue reports a case MORE THAN ONCE in one run, and may therefore
/// be counted by name.
///
/// ---- PALISADE MEMBRANE ----
///
/// FOREIGN SIGNATURE. Nextest renders a failing case twice: once inline as it
/// finishes, and once again in the recapitulation it prints beneath its summary.
/// Counted naively that is two cases, and the door then reports a tally its own
/// runner disagrees with — 26 where nextest said 25 — while printing the
/// operator two identical failure lines for one failure.
///
/// WHY THE REMEDY IS BY NAME, AND WHY ONLY HERE. Nextest qualifies every case
/// with its binary id (`vof::vofx_plant vofx_the_planted_case_fails`), so a name
/// is unique within a run and a repeat of one is a repeat of the case. THE
/// STANDARD HARNESS DOES NOT: it prints a bare case name, and two test binaries
/// of one manifest may each hold a case of the same name — so deduplicating
/// there would silently undercount two real cases as one. That is why this is a
/// property of the tongue rather than a rule of the voice, and why the voice may
/// not simply count distinct names.
///
/// RETIREMENT CONDITION. Retire this membrane when the kennel renders from
/// nextest's machine-readable output, where each case is reported once as data.
/// The captured log this was written against is
/// `hist-bkmw-m-sh-20260905-200936-2048454-747.txt`, a planted-red drive of
/// `suite-vof` at seat 2cab0e7eb60b5a1b1f72ccf6645016dbf8565b87, which carries
/// the same FAIL line at its line 25 and again at its line 55.
pub(crate) fn zbkcv_recapitulates(tongue: &str) -> bool {
    tongue == BKCV_TONGUE_NEXTEST
}

/// Read one line under the tongue the collar declared.
///
/// THE COLLAR ELECTS, AND AN UNDECLARED TONGUE IS A REFUSAL RATHER THAN A GUESS.
/// A door that fell back on a default when it did not recognize the word would
/// render a suite's cases under the wrong reader and report a green run as
/// having no cases at all.
pub fn bkcv_case(tongue: &str, line: &str) -> Result<Option<bkcv_Case>, String> {
    match tongue {
        BKCV_TONGUE_NEXTEST => Ok(zbkcv_nextest_case(line)),
        BKCV_TONGUE_HARNESS => Ok(zbkcv_harness_case(line)),
        BKCV_TONGUE_PYTEST => Ok(zbkcv_pytest_case(line)),
        other => Err(format!(
            "the collar declares the tongue '{}', which is no output shape the kennel \
             recognizes — it renders what a collar says and infers none, so a value outside \
             the declared roster names a reader nobody wrote",
            other, //
        )),
    }
}

/// SHA-256 over a byte slice, as the lowercase hex the digest line carries.
///
/// IMPLEMENTED HERE RATHER THAN TAKEN AS A DEPENDENCY OR SPAWNED. The kennel
/// declares one direct dependency and it is the regime reader beside it; a
/// digest is not worth the permitted-crate ruling a second would need. Spawning
/// a hashing program was the other road and is worse: the kennel provisions
/// nothing it spawns, so a station without that program would turn a record into
/// a refusal, and the algorithm is a fixed standard that will not move under us.
///
/// The constants are FIPS 180-4's own — the first thirty-two bits of the
/// fractional parts of the cube roots of the first sixty-four primes, and of the
/// square roots of the first eight. They are transcribed rather than computed,
/// which is what every implementation does, and the hurdles check the result
/// against the standard's published vectors rather than against this code.
pub(crate) fn zbkcv_sha256(bytes: &[u8]) -> String {
    const K: [u32; 64] = [
        0x428a2f98,
        0x71374491,
        0xb5c0fbcf,
        0xe9b5dba5,
        0x3956c25b,
        0x59f111f1,
        0x923f82a4,
        0xab1c5ed5,
        0xd807aa98,
        0x12835b01,
        0x243185be,
        0x550c7dc3,
        0x72be5d74,
        0x80deb1fe,
        0x9bdc06a7,
        0xc19bf174,
        0xe49b69c1,
        0xefbe4786,
        0x0fc19dc6,
        0x240ca1cc,
        0x2de92c6f,
        0x4a7484aa,
        0x5cb0a9dc,
        0x76f988da,
        0x983e5152,
        0xa831c66d,
        0xb00327c8,
        0xbf597fc7,
        0xc6e00bf3,
        0xd5a79147,
        0x06ca6351,
        0x14292967,
        0x27b70a85,
        0x2e1b2138,
        0x4d2c6dfc,
        0x53380d13,
        0x650a7354,
        0x766a0abb,
        0x81c2c92e,
        0x92722c85,
        0xa2bfe8a1,
        0xa81a664b,
        0xc24b8b70,
        0xc76c51a3,
        0xd192e819,
        0xd6990624,
        0xf40e3585,
        0x106aa070,
        0x19a4c116,
        0x1e376c08,
        0x2748774c,
        0x34b0bcb5,
        0x391c0cb3,
        0x4ed8aa4a,
        0x5b9cca4f,
        0x682e6ff3,
        0x748f82ee,
        0x78a5636f,
        0x84c87814,
        0x8cc70208,
        0x90befffa,
        0xa4506ceb,
        0xbef9a3f7,
        0xc67178f2, //
    ];

    let mut h: [u32; 8] = [
        0x6a09e667,
        0xbb67ae85,
        0x3c6ef372,
        0xa54ff53a,
        0x510e527f,
        0x9b05688c,
        0x1f83d9ab,
        0x5be0cd19,
    ];

    let mut padded = bytes.to_vec();
    let bits = (bytes.len() as u64).wrapping_mul(8);
    padded.push(0x80);
    while padded.len() % 64 != 56 {
        padded.push(0);
    }
    padded.extend_from_slice(&bits.to_be_bytes());

    for block in padded.chunks_exact(64) {
        let mut w = [0u32; 64];
        for (index, word) in block.chunks_exact(4).enumerate() {
            w[index] = u32::from_be_bytes([word[0], word[1], word[2], word[3]]);
        }
        for index in 16..64 {
            let s0 = w[index - 15].rotate_right(7)
                ^ w[index - 15].rotate_right(18)
                ^ (w[index - 15] >> 3);
            let s1 = w[index - 2].rotate_right(17)
                ^ w[index - 2].rotate_right(19)
                ^ (w[index - 2] >> 10);
            w[index] = w[index - 16]
                .wrapping_add(s0)
                .wrapping_add(w[index - 7])
                .wrapping_add(s1);
        }

        let (mut a, mut b, mut c, mut d, mut e, mut f, mut g, mut hh) =
            (h[0], h[1], h[2], h[3], h[4], h[5], h[6], h[7]);

        for index in 0..64 {
            let s1 = e.rotate_right(6) ^ e.rotate_right(11) ^ e.rotate_right(25);
            let choose = (e & f) ^ ((!e) & g);
            let temp1 = hh
                .wrapping_add(s1)
                .wrapping_add(choose)
                .wrapping_add(K[index])
                .wrapping_add(w[index]);
            let s0 = a.rotate_right(2) ^ a.rotate_right(13) ^ a.rotate_right(22);
            let major = (a & b) ^ (a & c) ^ (b & c);
            let temp2 = s0.wrapping_add(major);

            hh = g;
            g = f;
            f = e;
            e = d.wrapping_add(temp1);
            d = c;
            c = b;
            b = a;
            a = temp1.wrapping_add(temp2);
        }

        for (slot, value) in h.iter_mut().zip([a, b, c, d, e, f, g, hh]) {
            *slot = slot.wrapping_add(value);
        }
    }

    h.iter()
        .map(|word| {
            format!(
                "{:08x}",
                word, //
            )
        })
        .collect()
}

/// The local wall-clock stamp for one instant, in the historical member's form.
///
/// THE OFFSET IS DERIVED FROM WHAT THE DISPATCH ALREADY WROTE, and this is
/// the whole trick that keeps the kennel's stamps agreeing with a logged
/// dispatch's without a timezone crate and without spawning `date`. The dispatch
/// takes ONE reading and writes it twice — a LOCAL stamp and the epoch seconds
/// of the same instant — so the difference between them IS this station's offset,
/// measured rather than looked up, and correct across a zone the kennel could not
/// otherwise name.
///
/// Absent either reading the stamp falls back to UTC, and the fallback is
/// deliberately silent: a coordinator running outside a dispatch has no local
/// reading to agree with, so there is nothing to warn about.
fn zbkcv_offset(stamp: Option<&str>, epoch: Option<&str>) -> i64 {
    let stamp = match stamp {
        Some(stamp) => stamp,
        None => return 0,
    };
    let epoch: i64 = match epoch.and_then(|raw| raw.trim().parse().ok()) {
        Some(epoch) => epoch,
        None => return 0,
    };

    // `YYYYMMDD-HHMMSS-<pid>-<rand>`; only the first two tokens are read.
    let mut parts = stamp.split('-');
    let day = parts.next().unwrap_or_default();
    let time = parts.next().unwrap_or_default();
    if day.len() != 8 || time.len() != 6 {
        return 0;
    }

    let read =
        |text: &str, from: usize, to: usize| -> Option<i64> { text.get(from..to)?.parse().ok() };

    let year = match read(day, 0, 4) {
        Some(value) => value,
        None => return 0,
    };
    let month = match read(day, 4, 6) {
        Some(value) => value,
        None => return 0,
    };
    let date = match read(day, 6, 8) {
        Some(value) => value,
        None => return 0,
    };
    let hour = match read(time, 0, 2) {
        Some(value) => value,
        None => return 0,
    };
    let minute = match read(time, 2, 4) {
        Some(value) => value,
        None => return 0,
    };
    let second = match read(time, 4, 6) {
        Some(value) => value,
        None => return 0,
    };

    let local = zbkcv_epoch_from_civil(year, month, date, hour, minute, second);

    // The dispatch's own two readings are of one instant, so the difference is
    // the offset. It is rounded to the minute because a station's offset is
    // whole minutes by construction and the two readings can straddle a second.
    let raw = local - epoch;
    (raw + if raw >= 0 { 30 } else { -30 }) / 60 * 60
}

/// Days-from-civil, and its inverse. Both are the standard algorithms, carried
/// rather than derived, with the era arithmetic left as it is published.
fn zbkcv_epoch_from_civil(
    year: i64,
    month: i64,
    day: i64,
    hour: i64,
    minute: i64,
    second: i64,
) -> i64 {
    let shifted = if month <= 2 { year - 1 } else { year };
    let era = if shifted >= 0 { shifted } else { shifted - 399 } / 400;
    let year_of_era = shifted - era * 400;
    let day_of_year = (153 * (month + if month > 2 { -3 } else { 9 }) + 2) / 5 + day - 1;
    let day_of_era = year_of_era * 365 + year_of_era / 4 - year_of_era / 100 + day_of_year;
    let days = era * 146_097 + day_of_era - 719_468;
    days * 86_400 + hour * 3_600 + minute * 60 + second
}

fn zbkcv_civil_from_epoch(epoch: i64) -> (i64, i64, i64, i64, i64, i64) {
    let days = epoch.div_euclid(86_400);
    let rest = epoch.rem_euclid(86_400);

    let shifted = days + 719_468;
    let era = if shifted >= 0 {
        shifted
    } else {
        shifted - 146_096
    } / 146_097;
    let day_of_era = shifted - era * 146_097;
    let year_of_era =
        (day_of_era - day_of_era / 1_460 + day_of_era / 36_524 - day_of_era / 146_096) / 365;
    let year = year_of_era + era * 400;
    let day_of_year = day_of_era - (365 * year_of_era + year_of_era / 4 - year_of_era / 100);
    let shifted_month = (5 * day_of_year + 2) / 153;
    let day = day_of_year - (153 * shifted_month + 2) / 5 + 1;
    let month = shifted_month + if shifted_month < 10 { 3 } else { -9 };

    (
        if month <= 2 { year + 1 } else { year },
        month,
        day,
        rest / 3_600,
        (rest % 3_600) / 60,
        rest % 60,
    )
}

/// The stamp the historical member prefixes every line with.
fn zbkcv_stamp(offset: i64) -> String {
    let now = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .map(|since| since.as_secs() as i64)
        .unwrap_or(0);

    let (year, month, day, hour, minute, second) = zbkcv_civil_from_epoch(now + offset);
    format!(
        "[{:04}-{:02}-{:02} {:02}:{:02}:{:02}] ",
        year,
        month,
        day,
        hour,
        minute,
        second, //
    )
}

/// The three log members, open and being written.
///
/// ALL THREE OR NONE, which the family's law states and this shape enforces:
/// the constructor answers `None` when the mode is not in force, and there is no
/// road to a half-open family. Two authors in one family produce three files
/// that disagree about one run, and no reader can tell which half is missing.
pub struct bkcv_Record {
    last: std::fs::File,
    same: std::fs::File,
    hist: std::fs::File,
    /// Where the normalized member stands, so its bytes can be digested at the
    /// close by the writer that knows when the last of them was written.
    same_path: std::path::PathBuf,
    /// The ephemeral directory whose every occurrence the normalized member
    /// replaces. Absent where the dispatch declared none.
    ephemeral: Option<String>,
    /// This station's offset from UTC, measured once at the open.
    offset: i64,
}

impl bkcv_Record {
    /// Open the family the dispatch composed, or answer `None` where the
    /// own-log mode is not in force.
    ///
    /// THE MODE IS DECLARED BY THE PAIR STANDING, not by a flag of its own: the
    /// dispatch writes the last and same members into its file under this mode
    /// and under no other, so their presence IS the mode. The historical member
    /// stands there in every logged mode and opens no record by itself, which is
    /// what keeps a kennel run bare inside an ordinary logged dispatch from
    /// writing into that dispatch's own record. A half pair, or a pair standing
    /// without the historical member, is a refusal rather than a best effort —
    /// it means the dispatch composed names this reader does not understand,
    /// and writing two of three would produce exactly the disagreeing family the
    /// law forbids.
    pub fn bkcv_open(
        dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
        invocation: &str,
    ) -> Result<Option<bkcv_Record>, String> {
        let last = dispatch.bkcr_said(BKCV_LOG_LAST_VAR);
        let same = dispatch.bkcr_said(BKCV_LOG_SAME_VAR);
        let hist = dispatch.bkcr_said(BKCV_LOG_HIST_VAR);

        let (last, same, hist) = match (last, same, hist) {
            // NO PAIR IS NO RECORD, inside a dispatch or outside one. The mode
            // is a tabtarget's declaration and no caller's: a child launched
            // bare — a release's derby, a hurdle's spawn — keeps no record, and
            // its caller reads its passing result and its voice.
            (None, None, _) => return Ok(None),
            (Some(last), Some(same), Some(hist)) => (last, same, hist),
            (last, same, hist) => {
                let mut absent = Vec::new();
                for (field, value) in [
                    (BKCV_LOG_LAST_VAR, &last),
                    (BKCV_LOG_SAME_VAR, &same),
                    (BKCV_LOG_HIST_VAR, &hist),
                ] {
                    if value.is_none() {
                        absent.push(field);
                    }
                }
                return Err(format!(
                    "the log family is half-composed in {}: {} stand(s) absent while the others \
                     are written. The own-log mode writes all three, so this is a dispatch the \
                     kennel does not understand rather than a record it may write part of — all \
                     three members are written or none is",
                    dispatch.bkcr_whence(),
                    absent.join(", "), //
                ));
            }
        };

        // THE FIRST OPEN OF A PROCESS TRUNCATES AND EVERY LATER ONE APPENDS,
        // which is what makes a MULTI-COURSE door's record whole. The three
        // names are the dispatch's, composed once for the whole invocation, so
        // a door opening a record per course was re-truncating the one file
        // family at every course and leaving the run's record holding whichever
        // course happened to be last. A derby across ten courses banked one of
        // them, and the nine it threw away were the nine a reader would be
        // looking for.
        //
        // A SINGLE-COURSE DOOR IS UNCHANGED BY THIS, which is why the latch is
        // per PROCESS rather than per door: its one open is the first open, so
        // it truncates exactly as before. What the latch parts is not two kinds
        // of door but the first open from the rest, and no door has to declare
        // which it is.
        let reopening = ZBKCV_OPENED.swap(true, std::sync::atomic::Ordering::SeqCst);

        bkcv_Record::zbkcv_at(
            last,
            same,
            hist,
            dispatch.bkcr_said(BKCV_TEMP_DIR_VAR).map(str::to_string),
            reopening,
            invocation,
            dispatch
                .bkcr_said(BKCV_GIT_CONTEXT_VAR)
                .unwrap_or(ZBKCV_GIT_UNAVAILABLE),
            zbkcv_offset(
                dispatch.bkcr_said(BKCV_NOW_STAMP_VAR),
                dispatch.bkcr_said(BKCV_NOW_EPOCH_VAR),
            ),
        )
        .map(Some)
    }

    /// Open the family at the three paths given, truncating unless reopening.
    ///
    /// THE OPEN ABOVE READS THE DISPATCH AND THIS DOES NOT, which is what lets
    /// a hurdle stand a record from values alone: the position line and the
    /// station's offset are handed in beside the paths.
    #[allow(clippy::too_many_arguments)]
    pub(crate) fn zbkcv_at(
        last: &str,
        same: &str,
        hist: &str,
        ephemeral: Option<String>,
        reopening: bool,
        invocation: &str,
        context: &str,
        offset: i64,
    ) -> Result<bkcv_Record, String> {
        let open = |path: &str| -> Result<std::fs::File, String> {
            std::fs::OpenOptions::new()
                .create(true)
                .write(true)
                .truncate(!reopening)
                .append(reopening)
                .open(path)
                .map_err(|err| {
                    format!(
                        "could not open the log member {}: {}",
                        path,
                        err, //
                    )
                })
        };

        let mut record = bkcv_Record {
            last: open(last)?,
            same: open(same)?,
            hist: open(hist)?,
            same_path: std::path::PathBuf::from(same),
            ephemeral,
            offset,
        };

        // BUr_fbc — a record opens by naming its own invocation, and the
        // historical member also names its position. These two lines are the
        // audit's whole link from a record to a position in the repository.
        //
        // THEY ARE WRITTEN UNSTAMPED, which is what a logged dispatch does and
        // therefore what this mode owes. The stamping dictum governs the
        // COORDINATOR'S STREAM; these two lines stand ahead of it, and the law
        // reads "the FIRST line" and "the SECOND line" of the member — a
        // prefixed line is neither, and a reader comparing two records reads the
        // position line by its position.
        record.zbkcv_bare(&format!(
            "command: {}",
            invocation, //
        ))?;
        record.zbkcv_hist_only(&format!(
            "Git context: {}",
            context, //
        ))?;

        Ok(record)
    }

    /// The historical member's path, for the failure lines that send a reader to
    /// the depth — the dispatch's own wherever a log exists, the kennel's record
    /// or the logged dispatch it runs bare inside.
    pub fn bkcv_hist_path(dispatch: &crate::bkcr_resolve::bkcr_Dispatch) -> Option<String> {
        dispatch.bkcr_said(BKCV_LOG_HIST_VAR).map(str::to_string)
    }

    /// Write one line into all three members, each under its own dictum.
    ///
    /// This is the ordinary road, and everything the kennel records takes it:
    /// the child's whole stream and the door's own marks alike, interleaved in
    /// the order they happened.
    pub fn bkcv_write(&mut self, line: &str) -> Result<(), String> {
        self.zbkcv_raw(line)
    }

    fn zbkcv_raw(&mut self, line: &str) -> Result<(), String> {
        // The last member is the raw stream, uncurated.
        writeln!(
            self.last,
            "{}",
            line, //
        )
        .map_err(zbkcv_blame)?;

        // BUr_amx — the normalized member carries only what a rerun would
        // reproduce. Each rule removes exactly one run-to-run variable, and the
        // member is diffable in proportion to how completely they are applied.
        for frame in zbkcv_normalized(line, self.ephemeral.as_deref()) {
            writeln!(
                self.same,
                "{}",
                frame, //
            )
            .map_err(zbkcv_blame)?;
        }

        // BUr_yht — the historical member stamps every line, and the bytes after
        // the prefix are the coordinator's own, leading and trailing whitespace
        // preserved.
        writeln!(
            self.hist,
            "{}{}",
            zbkcv_stamp(self.offset),
            line, //
        )
        .map_err(zbkcv_blame)?;

        Ok(())
    }

    fn zbkcv_hist_only(&mut self, line: &str) -> Result<(), String> {
        writeln!(
            self.hist,
            "{}",
            line, //
        )
        .map_err(zbkcv_blame)
    }

    /// The opening lines: into every member the mode creates, and into none of
    /// them stamped or normalized. They are the record's frame rather than part
    /// of the stream it holds.
    fn zbkcv_bare(&mut self, line: &str) -> Result<(), String> {
        writeln!(
            self.last,
            "{}",
            line, //
        )
        .map_err(zbkcv_blame)?;
        writeln!(
            self.same,
            "{}",
            line, //
        )
        .map_err(zbkcv_blame)?;
        writeln!(
            self.hist,
            "{}",
            line, //
        )
        .map_err(zbkcv_blame)?;
        Ok(())
    }

    /// The coordinator's own invocation, as a record opens by naming.
    ///
    /// THE KENNEL'S ARGV AND NOT THE CARGO LINE IT COMPOSED. What a logged
    /// dispatch writes here is the coordinator command it ran, so a reader
    /// comparing a kennel-written record against a bash-written one is comparing
    /// like with like. The composed cargo invocation is a fact about what the
    /// kennel decided, and it stands in the stream below where the door says it.
    pub fn bkcv_invocation() -> String {
        std::env::args().collect::<Vec<String>>().join(" ")
    }

    /// Close the family: flush every member, then stamp the historical one with
    /// the normalized member's digest.
    ///
    /// THE DIGEST IS THE WRITER'S, taken after the last byte of the normalized
    /// member is written, because only the writer knows when that is. Under this
    /// mode the stamping and the digest alike sit with the coordinator, which is
    /// one rule reaching its other author rather than a second rule for the mode.
    ///
    /// THE DIGEST LINE IS NOT ITSELF STAMPED, matching the substrate's own
    /// curator: it is appended after the coordinator has exited, outside the
    /// stream the stamping applies to.
    pub fn bkcv_close(mut self) -> Result<(), String> {
        self.last.flush().map_err(zbkcv_blame)?;
        self.same.flush().map_err(zbkcv_blame)?;
        self.hist.flush().map_err(zbkcv_blame)?;

        let bytes = std::fs::read(&self.same_path).map_err(|err| {
            format!(
                "could not read {} for its digest: {}",
                self.same_path.display(),
                err, //
            )
        })?;

        writeln!(
            self.hist,
            "{}{}",
            ZBKCV_CHECKSUM_LEAD,
            zbkcv_sha256(&bytes), //
        )
        .map_err(zbkcv_blame)?;
        self.hist.flush().map_err(zbkcv_blame)?;
        Ok(())
    }
}

fn zbkcv_blame(err: std::io::Error) -> String {
    format!(
        "could not write the log family: {}",
        err, //
    )
}

/// BUr_amx applied to one line, answering the frames it becomes.
///
/// Empty lines are dropped, so a line that normalizes to nothing yields nothing;
/// a line carrying carriage returns yields one frame per redraw state, so a
/// progress redraw lands as its successive states rather than as one unreadable
/// line.
pub(crate) fn zbkcv_normalized(line: &str, ephemeral: Option<&str>) -> Vec<String> {
    if line.is_empty() {
        return Vec::new();
    }

    let stripped = zbkcv_uncolored(line);

    // A line carrying the marker is dropped WHOLE, ahead of the frame split: the
    // marker declares the line volatile, and splitting first would keep the
    // frames of a line that had declared itself unreproducible.
    if stripped.contains(ZBKCV_VOLATILE) {
        return Vec::new();
    }

    let placed = match ephemeral {
        Some(dir) => stripped.replace(dir, ZBKCV_EPHEMERAL),
        None => stripped,
    };

    placed
        .split('\r')
        .filter(|frame| !frame.is_empty())
        .map(|frame| frame.to_string())
        .collect()
}

/// Strip terminal control sequences.
///
/// The two shapes the substrate's own curator removes: a CSI sequence — escape,
/// `[`, parameter bytes, one final letter — and a character-set selection,
/// escape, `(`, one letter. Anything else carrying an escape is left alone
/// rather than guessed at, on the membrane rule that only the surveyed signature
/// is absorbed.
pub(crate) fn zbkcv_uncolored(line: &str) -> String {
    let mut clean = String::with_capacity(line.len());
    let mut glyphs = line.chars().peekable();

    while let Some(glyph) = glyphs.next() {
        if glyph != '\u{1b}' {
            clean.push(glyph);
            continue;
        }

        match glyphs.peek() {
            Some('[') => {
                glyphs.next();
                while let Some(&inner) = glyphs.peek() {
                    glyphs.next();
                    if inner.is_ascii_alphabetic() {
                        break;
                    }
                    if !inner.is_ascii_digit() && inner != ';' {
                        break;
                    }
                }
            }
            Some('(') => {
                glyphs.next();
                if glyphs
                    .peek()
                    .is_some_and(|inner| inner.is_ascii_alphabetic())
                {
                    glyphs.next();
                }
            }
            _ => clean.push(glyph),
        }
    }

    clean
}

/// Where a voice's console lines go: one line at a time, in the order spoken.
///
/// A VALUE AND NOT A CALL SITE, so a hurdle reads what a voice said without
/// capturing the process's own stdout, which every hurdle thread shares. The
/// doors hand the substrate's voice throat; nothing else is handed outside a
/// hurdle.
pub(crate) type zbkcv_Console = Box<dyn FnMut(&str)>;

/// The console every door's voice speaks onto: stdout, flushed per line.
fn zbkcv_stdout() -> zbkcv_Console {
    Box::new(|line: &str| bkco_output::bkco_voice(line))
}

/// What every voice holds whatever it is reading: the record, the console, and
/// the cadence the console's silence is measured on.
///
/// ONE THROAT FOR EVERY READER. A suite and a build read different shapes off
/// their child, and nothing else about them differs: both write the child's
/// whole stream into the record, both mark the console's silence on the same
/// cadence, and both send a reader to the same depth. Held once, the two cannot
/// come to keep the record or the heartbeat differently.
struct zbkcv_Throat {
    record: Option<bkcv_Record>,
    hist: Option<String>,
    verbose: bool,
    began: std::time::Instant,
    /// When the CONSOLE last carried anything — a case line, a failure, a mark.
    /// The cadence is measured from here and never from the child.
    spoke: std::time::Instant,
    /// How long the console may stay silent before the voice marks that the run
    /// is alive.
    cadence: std::time::Duration,
    console: zbkcv_Console,
}

impl zbkcv_Throat {
    fn zbkcv_new(
        record: Option<bkcv_Record>,
        hist: Option<String>,
        cadence: std::time::Duration,
        console: zbkcv_Console,
    ) -> zbkcv_Throat {
        zbkcv_Throat {
            record,
            hist,
            verbose: bkco_output::bkco_verbose(),
            began: std::time::Instant::now(),
            spoke: std::time::Instant::now(),
            cadence,
            console,
        }
    }

    /// One line into the record, where a record stands, and nowhere else.
    fn zbkcv_write(&mut self, line: &str) -> Result<(), String> {
        match self.record.as_mut() {
            Some(record) => record.bkcv_write(line),
            None => Ok(()),
        }
    }

    fn zbkcv_say(&mut self, line: &str) {
        (self.console)(line);
        self.spoke = std::time::Instant::now();
    }

    /// The liveness mark, where the console has been silent a whole cadence.
    ///
    /// THE CADENCE IS MEASURED FROM THE CONSOLE AND NEVER FROM THE CHILD, and
    /// getting this backwards is the failure the mark exists to prevent. A
    /// green suite streams a line per case and a build a line per crate, so the
    /// CHILD is never silent — but in the quiet flavor none of that reaches the
    /// console, and what the operator has in front of them is a blank screen
    /// for as long as the run takes. A mark keyed on the child falling quiet is
    /// a mark that never fires on exactly the run that needed it. Fifty seconds
    /// of a green 1758-case suite is the specimen this was found on.
    fn zbkcv_pulse(&mut self) {
        if self.spoke.elapsed() >= self.cadence {
            self.zbkcv_mark();
        }
    }

    /// The liveness mark: a heartbeat on a cadence, never a per-case dot, every
    /// mark flushed.
    ///
    /// A RUN THAT IS TALKING IS NOT ALSO TOLD IT IS ALIVE. The mark answers the
    /// CONSOLE'S silence, so a verbose run rendering a line per case never
    /// carries one — there is nothing to reassure anybody about — while a quiet
    /// run carries one every cadence however loud its child is.
    ///
    /// IT IS NEVER WRITTEN TO THE RECORD. A heartbeat is a fact about a terminal
    /// watching rather than about the run, and a re-read log has nothing to be
    /// live about.
    fn zbkcv_mark(&mut self) {
        let mark = format!(
            "  · {}s",
            self.began.elapsed().as_secs(), //
        );
        self.zbkcv_say(&mark);
    }

    /// Where a line about a failure sends its reader: the record's path, or a
    /// clause saying none stands rather than an empty column.
    fn zbkcv_beside(&self) -> String {
        match self.hist.as_deref() {
            Some(path) => path.to_string(),
            None => "(no record: this dispatch composed none)".to_string(),
        }
    }

    /// Close the record over a run the leash could not attend, answering the
    /// refusal for the door to report.
    ///
    /// THE RECORD IS CLOSED EVEN HERE. What the child said before the leash
    /// lost it is already written, and a family left open carries no digest and
    /// no line saying why it stops. The console is the door's, which reports the
    /// refusal itself.
    fn zbkcv_abandon(mut self, refusal: String) -> String {
        if let Some(mut record) = self.record.take() {
            let closed = record
                .bkcv_write(&format!(
                    "RED: the run could not be attended: {}",
                    refusal, //
                ))
                .and_then(|()| record.bkcv_close());
            if let Err(err) = closed {
                return format!(
                    "{}; the record could not be closed: {}",
                    refusal,
                    err, //
                );
            }
        }
        refusal
    }

    /// Close the throat: the verdict onto the console and into the record, and
    /// the record's digest onto the historical member.
    fn zbkcv_close(mut self, verdict: &str) -> Result<(), String> {
        (self.console)(verdict);

        if let Some(mut record) = self.record.take() {
            record.bkcv_write(verdict)?;
            record.bkcv_close()?;
        }

        Ok(())
    }
}

/// The whole rendering of one suite launch: the record written, the console
/// spoken, the tally counted.
pub struct bkcv_Voice {
    throat: zbkcv_Throat,
    tongue: String,
    tally: bkcv_Tally,
    /// Cases already counted, kept only where the tongue recapitulates and its
    /// names are unique within a run.
    counted: std::collections::BTreeSet<String>,
}

impl bkcv_Voice {
    /// Open the voice for one launch, writing the invocation into the record.
    pub fn bkcv_open(
        dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
        tongue: &str,
        invocation: &str,
    ) -> Result<bkcv_Voice, String> {
        Ok(bkcv_Voice::zbkcv_with(
            tongue,
            bkcv_Record::bkcv_open(dispatch, invocation)?,
            bkcv_Record::bkcv_hist_path(dispatch),
        ))
    }

    /// The voice over a record already opened, and the path a failure line
    /// sends its reader to.
    pub(crate) fn zbkcv_with(
        tongue: &str,
        record: Option<bkcv_Record>,
        hist: Option<String>,
    ) -> bkcv_Voice {
        bkcv_Voice {
            throat: zbkcv_Throat::zbkcv_new(record, hist, BKCV_CADENCE, zbkcv_stdout()),
            tongue: tongue.to_string(),
            tally: bkcv_Tally::default(),
            counted: std::collections::BTreeSet::new(),
        }
    }

    /// Write one of the door's own marks into the record, saying nothing on the
    /// console. The record holds the child's stream interleaved with these; the
    /// console holds the voice alone.
    pub fn bkcv_mark(&mut self, line: &str) -> Result<(), String> {
        self.throat.zbkcv_write(line)
    }

    /// Take one thing heard from the child.
    pub fn bkcv_heed(&mut self, heard: crate::bkcl_leash::bkcl_Heard) -> Result<(), String> {
        // The cadence's law stands at `zbkcv_Throat::zbkcv_pulse`.
        if let crate::bkcl_leash::bkcl_Heard::Said(line) = &heard {
            let line = line.clone();
            self.zbkcv_said(&line)?;
        }

        // The lull arm carries no work of its own: its whole job is to WAKE this
        // loop when the child is silent, so the check below is reached at all.
        // A child that says nothing for a minute and one that says everything
        // are the same case from the console's side, and they take the same
        // road here.
        self.throat.zbkcv_pulse();

        Ok(())
    }

    fn zbkcv_said(&mut self, line: &str) -> Result<(), String> {
        // INTO THE RECORD GOES THE CHILD'S WHOLE STREAM, whatever the console is
        // shown. The dial moves the console and never the record.
        self.throat.zbkcv_write(line)?;

        if let Some(case) = bkcv_case(&self.tongue, line)? {
            // A CASE THE RUNNER REPEATS IS STILL ONE CASE, counted once and
            // reported once. Held only where the tongue's names are unique
            // within a run; see the membrane above for why the other runner
            // must not be treated this way.
            if zbkcv_recapitulates(&self.tongue) && !self.counted.insert(case.name.clone()) {
                return Ok(());
            }

            self.tally.ran += 1;
            if case.passed {
                self.tally.passed += 1;
            } else {
                self.tally.failed.push(case.name.clone());
            }

            // THE QUIET FLAVOR SPEAKS ONLY OF FAILURE, and speaks of it once per
            // case with the depth at a path. An agent reading test output pays
            // for every line out of its working context, so failure output is
            // bounded exactly as green output is.
            if self.throat.verbose {
                self.throat.zbkcv_say(&format!(
                    "  {} {}",
                    if case.passed { "pass" } else { "FAIL" },
                    case.name, //
                ));
            } else if !case.passed {
                let failure = self.zbkcv_failure(&case.name);
                self.throat.zbkcv_say(&failure);
            }
        }

        Ok(())
    }

    /// One bounded line for a failing case, with the depth at a path beside it.
    /// A launch outside a logging dispatch has no path to send a reader to, and
    /// says so rather than printing an empty column.
    fn zbkcv_failure(&self, name: &str) -> String {
        format!(
            "  FAIL {}  {}",
            name,
            self.throat.zbkcv_beside(), //
        )
    }

    /// Close the record over a run the leash could not attend, answering the
    /// refusal for the door to report (`zbkcv_Throat::zbkcv_abandon`).
    pub fn bkcv_abandon(self, refusal: String) -> String {
        self.throat.zbkcv_abandon(refusal)
    }

    /// Close the voice: the verdict onto the console, the record's digest onto
    /// the historical member. The verdict's own law stands at `zbkcv_verdict`.
    pub fn bkcv_close(self, run: &crate::bkcl_leash::bkcl_Run) -> Result<bkcv_Tally, String> {
        let verdict = zbkcv_verdict(
            &self.tally,
            run.code,
            run.cut.as_deref(),
            self.throat.hist.as_deref(),
        );

        self.throat.zbkcv_close(&verdict)?;

        Ok(self.tally)
    }
}

/// How a run ended short of finishing, in the words a red line carries, or
/// `None` for a runner that finished — green, or red with its failing cases
/// reported.
///
/// A RUN THAT DID NOT FINISH CARRIES NO COUNT AS ITS OWN. What the kennel heard
/// before the end is said as heard, and never as how many passed: forty cases
/// heard passing ahead of a runner that died are forty cases nobody knows the
/// rest of, and a line reading forty passed reads as a suite that did.
///
/// THE EXIT IS NAMED, because it is the one fact that parts a compile that
/// refused from a process that ended itself inside its hold, and a reader
/// without it digs a log to learn which. A red exit with no failing case
/// recognized may still be a membrane fallen behind its runner; the depth the
/// verdict sends its reader to is what settles that.
pub fn bkcv_unfinished(code: Option<i32>, cut: Option<&str>, tally: &bkcv_Tally) -> Option<String> {
    if let Some(cut) = cut {
        return Some(format!(
            "the stream was cut before the runner finished ({}), {} case(s) heard ahead of the cut",
            cut,
            tally.ran, //
        ));
    }

    match code {
        None => Some(format!(
            "a signal took the runner, which left no exit code, {} case(s) heard before it ended",
            tally.ran, //
        )),
        Some(code) if code != 0 && tally.failed.is_empty() => Some(format!(
            "the runner ended on exit {} without reporting a failing case — it refused or died — {} \
             case(s) heard before it ended",
            code, tally.ran, //
        )),
        Some(_) => None,
    }
}

/// The verdict line a launch closes on, over what the kennel counted and how the
/// child ended.
///
/// THE VERDICT IS THE EXIT CODE'S AND THE COUNTS ARE THE KENNEL'S. The exit
/// code decides green or red, because the kennel owns the invocation and no
/// pipe stands in the way; the counts are what this door itself saw reported,
/// never a summary read back off the child. A CUT STREAM IS RED OVER A CLEAN
/// EXIT, since the counts it would print were taken over half a run.
pub(crate) fn zbkcv_verdict(
    tally: &bkcv_Tally,
    code: Option<i32>,
    cut: Option<&str>,
    hist: Option<&str>,
) -> String {
    match bkcv_unfinished(code, cut, tally) {
        Some(clause) => match hist {
            Some(path) => format!(
                "RED: {}; the depth is at {}",
                clause,
                path, //
            ),
            None => format!(
                "RED: {}",
                clause, //
            ),
        },
        None => format!(
            "{}: {} ran, {} passed, {} failed",
            if code == Some(0) { "green" } else { "RED" },
            tally.ran,
            tally.passed,
            tally.failed.len(), //
        ),
    }
}

/// The flag that asks cargo for its machine-readable account of a build.
///
/// THE JSON FORM AND NEVER THE SHORT ONE. Both name a diagnostic's file and
/// position, and only the JSON form carries the diagnostic's whole rendering
/// beside them — which is the depth a build with no record has nowhere else to
/// put, so the short form would leave such a build with one line and nothing
/// beneath it. The one console line is composed here from the record's fields
/// rather than taken from either rendering.
///
/// It spells cargo's own flag, on the lock flag's ground at the leash: cargo
/// owns the word.
pub const BKCV_MESSAGE_FORMAT: &str = "--message-format=json";

/// The field every record of cargo's JSON form carries, and the values the
/// build reading parts by. Cargo's own words, quoted rather than minted.
const ZBKCV_REASON: &str = "reason";
const ZBKCV_REASON_ARTIFACT: &str = "compiler-artifact";
const ZBKCV_REASON_MESSAGE: &str = "compiler-message";
const ZBKCV_REASON_FINISHED: &str = "build-finished";

/// The two diagnostic levels that reach the console. Every other level — a
/// note, a help, the failure-note naming an explanation — stays in the record.
const ZBKCV_LEVEL_ERROR: &str = "error";
const ZBKCV_LEVEL_WARNING: &str = "warning";

/// How cargo's own prose opens an error or a warning of its own, off the JSON
/// form: a lock `--locked` refuses, a manifest that does not parse, a build
/// script's warning.
const ZBKCV_PROSE_ERROR: &str = "error:";
const ZBKCV_PROSE_WARNING: &str = "warning:";

/// Cargo's prose summary of a crate whose compiler errors it already reported.
///
/// ---- PALISADE MEMBRANE ----
///
/// FOREIGN SIGNATURE. A crate that failed to compile ends with cargo's own line
/// on stderr, beside the JSON records that already carried each error:
///
/// ```text
/// error: could not compile `frabblee` (bin "frabblee") due to 1 previous error
/// ```
///
/// ABSORB ONLY THE SURVEYED SIGNATURE, AND ONLY WHERE IT SUMMARIZES. Stepped
/// over where a compiler error has already spoken, since it then repeats what
/// the console just said; forwarded where none has, since it is then the only
/// line saying why the build failed. Every other prose `error:` is forwarded.
///
/// RETIREMENT CONDITION. Retire this membrane when cargo's JSON form carries its
/// own account of a failed crate as a record. The capture it was written
/// against is cargo 1.90.0 building a scratch crate planted with one mismatched
/// type under `--message-format=json`, whose stderr carried that line and the
/// Compiling line alone.
const ZBKCV_COULD_NOT_COMPILE: &str = "error: could not compile ";

/// Which cargo act a cargo voice renders.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
enum zbkcv_Act {
    /// A build, read through the JSON form: each diagnostic one line, the
    /// progress to the record.
    Build,
    /// A lock's re-derivation. Cargo takes no message format for it, and its
    /// prose — the packages locked, added and updated — is what an operator
    /// reads to see what moved, so every line rides the console whole.
    Lock,
}

/// The rendering of one cargo act other than a suite: the record written, the
/// console spoken, the diagnostics counted.
///
/// A BUILD SOUNDS LIKE A TEST. The console carries the launch mark, the
/// heartbeat, one line per diagnostic and the verdict, whatever the compiler
/// said underneath; cargo's per-crate progress goes to the record and never to
/// the console, and a green build is quiet.
///
/// THE BUILD READER IS THE VOICE'S OWN AND NO COLLAR'S. A suite's tongue is a
/// collar's declaration because a runner can be asked for another shape; the
/// shape a build is read in is the one this voice asks cargo for, so there is
/// nothing for a collar to declare and `bkcv_case` admits no build tongue.
pub struct bkcv_Cargo {
    throat: zbkcv_Throat,
    act: zbkcv_Act,
    /// Artifacts cargo reported built or found fresh — counted, never printed.
    artifacts: usize,
    errors: usize,
    warnings: usize,
    /// What cargo's own closing record said of the build, where it spoke one.
    finished: Option<bool>,
}

impl bkcv_Cargo {
    /// Open the voice over one build, writing the invocation into the record.
    pub fn bkcv_build(
        dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
        invocation: &str,
    ) -> Result<bkcv_Cargo, String> {
        Ok(bkcv_Cargo::zbkcv_build_with(
            bkcv_Record::bkcv_open(dispatch, invocation)?,
            bkcv_Record::bkcv_hist_path(dispatch),
            BKCV_CADENCE,
            zbkcv_stdout(),
        ))
    }

    /// Open the voice over one lock's re-derivation.
    pub fn bkcv_lock(
        dispatch: &crate::bkcr_resolve::bkcr_Dispatch,
        invocation: &str,
    ) -> Result<bkcv_Cargo, String> {
        Ok(bkcv_Cargo::zbkcv_cargo(
            zbkcv_Act::Lock,
            bkcv_Record::bkcv_open(dispatch, invocation)?,
            bkcv_Record::bkcv_hist_path(dispatch),
            BKCV_CADENCE,
            zbkcv_stdout(),
        ))
    }

    /// A build voice over a record, cadence and console the caller holds.
    ///
    /// THE HURDLE'S SEAM, on `bkcv_Voice::zbkcv_with`'s ground: nothing here
    /// reads the environment or writes the process's stdout, so a hurdle reads
    /// what the voice said at a cadence it can wait out.
    pub(crate) fn zbkcv_build_with(
        record: Option<bkcv_Record>,
        hist: Option<String>,
        cadence: std::time::Duration,
        console: zbkcv_Console,
    ) -> bkcv_Cargo {
        bkcv_Cargo::zbkcv_cargo(zbkcv_Act::Build, record, hist, cadence, console)
    }

    fn zbkcv_cargo(
        act: zbkcv_Act,
        record: Option<bkcv_Record>,
        hist: Option<String>,
        cadence: std::time::Duration,
        console: zbkcv_Console,
    ) -> bkcv_Cargo {
        bkcv_Cargo {
            throat: zbkcv_Throat::zbkcv_new(record, hist, cadence, console),
            act,
            artifacts: 0,
            errors: 0,
            warnings: 0,
            finished: None,
        }
    }

    /// The launch mark: what is about to run, onto the console and into the
    /// record alike, so a quiet build still says what it is building.
    pub fn bkcv_launch(&mut self, line: &str) -> Result<(), String> {
        self.throat.zbkcv_write(line)?;
        self.throat.zbkcv_say(line);
        Ok(())
    }

    /// Take one thing heard from the child.
    pub fn bkcv_heed(&mut self, heard: crate::bkcl_leash::bkcl_Heard) -> Result<(), String> {
        if let crate::bkcl_leash::bkcl_Heard::Said(line) = &heard {
            // INTO THE RECORD GOES THE CHILD'S WHOLE STREAM, the JSON records
            // and cargo's prose alike, whatever the console is shown.
            self.throat.zbkcv_write(line)?;

            match self.act {
                zbkcv_Act::Build => self.zbkcv_build_said(line)?,
                zbkcv_Act::Lock => {
                    if !line.trim().is_empty() {
                        self.throat.zbkcv_say(line);
                    }
                }
            }
        }

        self.throat.zbkcv_pulse();

        Ok(())
    }

    /// Read one line of a build: a JSON record by its reason, or cargo's prose.
    fn zbkcv_build_said(&mut self, line: &str) -> Result<(), String> {
        let trimmed = line.trim_start();

        // A LINE THAT IS NOT A RECORD IS CARGO'S PROSE — the per-crate progress
        // above all, which is what the console is kept clear of. Only an error
        // or a warning of cargo's own reaches the console, since a build that
        // failed with no compiler message must still say why.
        let record = if trimmed.starts_with('{') {
            crate::bkcj_json::bkcj_read(trimmed).ok()
        } else {
            None
        };

        let Some(record) = record else {
            if trimmed.starts_with(ZBKCV_COULD_NOT_COMPILE) && self.errors > 0 {
                return Ok(());
            }
            let error = trimmed.starts_with(ZBKCV_PROSE_ERROR);
            if error || trimmed.starts_with(ZBKCV_PROSE_WARNING) {
                if error {
                    self.errors += 1;
                } else {
                    self.warnings += 1;
                }
                let said = format!(
                    "  {}  {}",
                    trimmed,
                    self.throat.zbkcv_beside(), //
                );
                self.throat.zbkcv_say(&said);
            }
            return Ok(());
        };

        match record
            .bkcj_field(ZBKCV_REASON)
            .and_then(|reason| reason.bkcj_string())
        {
            Some(ZBKCV_REASON_ARTIFACT) => {
                self.artifacts += 1;
                // THE VERBOSE FLAVOR'S PER-CASE LINE, which for a build is the
                // target each artifact was built for. Quiet, it is counted only.
                if self.throat.verbose {
                    let name = record
                        .bkcj_field("target")
                        .and_then(|target| target.bkcj_field("name"))
                        .and_then(|name| name.bkcj_string())
                        .unwrap_or("?");
                    self.throat.zbkcv_say(&format!(
                        "  built {}",
                        name, //
                    ));
                }
            }
            Some(ZBKCV_REASON_MESSAGE) => {
                if let Some(message) = record.bkcj_field("message") {
                    self.zbkcv_diagnostic(message)?;
                }
            }
            Some(ZBKCV_REASON_FINISHED) => {
                self.finished = match record.bkcj_field("success") {
                    Some(crate::bkcj_json::bkcj_Value::Bool(success)) => Some(*success),
                    _ => None,
                };
            }
            // A build script's report and every reason this reading does not
            // part stand in the record already, and say nothing on the console.
            _ => {}
        }

        Ok(())
    }

    /// One compiler diagnostic: one console line naming its level, file,
    /// position and message, and its rendering wherever the depth must go.
    ///
    /// WHERE NO RECORD STANDS THE RENDERING RIDES THE CONSOLE WHOLE beneath its
    /// line, errors and warnings alike. A kennel called bare inside a dispatch
    /// that composed it no family writes the depth nowhere, and the caller's own
    /// record is the console it tees; a build failing there would otherwise
    /// leave one line and no account. Where a record stands, the line stays one
    /// line and the rendering goes into the record beside the raw record it came
    /// in, readable rather than escaped.
    fn zbkcv_diagnostic(&mut self, message: &crate::bkcj_json::bkcj_Value) -> Result<(), String> {
        let text = |field: &str| {
            message
                .bkcj_field(field)
                .and_then(|value| value.bkcj_string())
        };

        let level = text("level").unwrap_or("");
        if level == ZBKCV_LEVEL_ERROR || level.starts_with(ZBKCV_PROSE_ERROR) {
            self.errors += 1;
        } else if level == ZBKCV_LEVEL_WARNING {
            self.warnings += 1;
        } else {
            return Ok(());
        }

        let code = message
            .bkcj_field("code")
            .and_then(|code| code.bkcj_field("code"))
            .and_then(|code| code.bkcj_string())
            .map(|code| {
                format!(
                    "[{}]",
                    code, //
                )
            })
            .unwrap_or_default();

        // THE PRIMARY SPAN IS THE POSITION. A diagnostic carries every span it
        // points at, and the one the compiler marks primary is where it says the
        // fault stands; a diagnostic with none — a link failure — names no
        // position rather than borrowing one.
        let position = message
            .bkcj_field("spans")
            .and_then(|spans| spans.bkcj_array())
            .and_then(|spans| {
                spans.iter().find(|span| {
                    span.bkcj_field("is_primary") == Some(&crate::bkcj_json::bkcj_Value::Bool(true))
                })
            })
            .map(|span| {
                let number = |field: &str| {
                    span.bkcj_field(field)
                        .and_then(|value| value.bkcj_number())
                        .map(|value| value as u64)
                        .unwrap_or(0)
                };
                format!(
                    "{}:{}:{} ",
                    span.bkcj_field("file_name")
                        .and_then(|file| file.bkcj_string())
                        .unwrap_or("?"),
                    number("line_start"),
                    number("column_start"), //
                )
            })
            .unwrap_or_default();

        let said = text("message")
            .and_then(|said| said.lines().next())
            .unwrap_or("");

        let line = format!(
            "  {}{} {}{}  {}",
            level,
            code,
            position,
            said,
            self.throat.zbkcv_beside(), //
        );
        self.throat.zbkcv_say(&line);

        if let Some(rendered) = text("rendered") {
            for depth in rendered.trim_end().lines() {
                if self.throat.record.is_some() {
                    self.throat.zbkcv_write(depth)?;
                } else {
                    self.throat.zbkcv_say(depth);
                }
            }
        }

        Ok(())
    }

    /// Run the act under this voice: the leash attended through `attend`, every
    /// line heeded, the verdict spoken, and whether the act landed answered
    /// beside the run.
    ///
    /// THE CALLER NAMES THE LEASH FACE AND THIS HOLDS THE WIRING, so a build and
    /// a lock attend the same way. The face is handed the cadence the voice
    /// marks silence on, so the lull that wakes the voice and the cadence it
    /// measures are one value.
    ///
    /// A HEED THAT CANNOT REFUSE, on the launch seam's own ground: a record write
    /// that fails mid-stream is held and surfaced after the close, where the run
    /// still exists to be reported, rather than abandoning a child whose output
    /// is gone the moment it is not taken.
    ///
    /// WHAT LANDED IS THE VOICE'S VERDICT, and the exit stays the child's on the
    /// run: a door acting on the build reads the first, a door relaying cargo
    /// relays the second.
    pub fn bkcv_ride<A>(mut self, attend: A) -> Result<(crate::bkcl_leash::bkcl_Run, bool), String>
    where
        A: FnOnce(
            std::time::Duration,
            &mut dyn FnMut(crate::bkcl_leash::bkcl_Heard),
        ) -> Result<crate::bkcl_leash::bkcl_Run, String>,
    {
        let cadence = self.throat.cadence;
        let mut grievance: Option<String> = None;

        let attended = {
            let mut heed = |heard: crate::bkcl_leash::bkcl_Heard| {
                if let Err(err) = self.bkcv_heed(heard) {
                    grievance.get_or_insert(err);
                }
            };
            attend(cadence, &mut heed)
        };

        let run = match attended {
            Ok(run) => run,
            Err(err) => return Err(self.throat.zbkcv_abandon(err)),
        };

        let (verdict, landed) = self.zbkcv_verdict(&run);
        self.throat.zbkcv_close(&verdict)?;

        if let Some(err) = grievance {
            return Err(err);
        }

        Ok((run, landed))
    }

    /// The verdict line the act closes on, and whether it landed.
    ///
    /// THE BUILD'S OWN CLOSING RECORD IS THE GROUND, the exit beside it. A build
    /// landed where cargo exited clean, the stream was heard whole, and the
    /// build-finished record — where cargo spoke one — did not say otherwise.
    /// A RUN THAT DID NOT FINISH IS SAID AS SUCH and never as a count of what
    /// was built, on `bkcv_unfinished`'s honesty rule.
    fn zbkcv_verdict(&self, run: &crate::bkcl_leash::bkcl_Run) -> (String, bool) {
        let depth = match self.throat.hist.as_deref() {
            Some(path) => format!(
                "; the depth is at {}",
                path, //
            ),
            None => String::new(),
        };

        if let Some(cut) = run.cut.as_deref() {
            return (
                format!(
                    "RED: the stream was cut before cargo finished ({}){}",
                    cut,
                    depth, //
                ),
                false,
            );
        }

        let Some(code) = run.code else {
            return (
                format!(
                    "RED: a signal took cargo, which left no exit code{}",
                    depth, //
                ),
                false,
            );
        };

        match self.act {
            zbkcv_Act::Build if code == 0 && self.finished != Some(false) => (
                format!(
                    "green: built, {} artifact(s), {} warning(s)",
                    self.artifacts,
                    self.warnings, //
                ),
                true,
            ),
            zbkcv_Act::Build => (
                format!(
                    "RED: did not build, {} error(s), {} warning(s), cargo exited {}{}",
                    self.errors,
                    self.warnings,
                    code,
                    depth, //
                ),
                false,
            ),
            zbkcv_Act::Lock if code == 0 => ("green: the lock was derived".to_string(), true),
            zbkcv_Act::Lock => (
                format!(
                    "RED: the lock was not derived, cargo exited {}{}",
                    code,
                    depth, //
                ),
                false,
            ),
        }
    }
}

// eof
