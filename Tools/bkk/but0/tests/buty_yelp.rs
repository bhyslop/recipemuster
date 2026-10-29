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

//! The yelp module: markers go in, rendered text comes out, and no marker survives.
//!
//! Ported from the bash bench's `buym-yelp` fixture, eighteen cases, all eighteen
//! carried.
//!
//! A DIASTEMA MARKER IS AN INTERIOR DELIMITER, so its absence is asserted beside
//! nearly every rendering here rather than once. The module builds a marker out
//! of a control byte, resolves it at format time, and a survivor in rendered text
//! is a marker the resolver failed to consume — which reaches an operator's
//! terminal as a stray control character in the middle of a sentence, and reaches
//! a log file as a byte that makes the line unsearchable.
//!
//! THE COLD DEATH IS RAISED BARE, which is where this fixture parts from its bash
//! original most sharply. Those two cases contained `buc_die_now` in a nested
//! subshell so the helper could survive to be captured, and then asserted that
//! the helper had survived — an assertion about the containment rather than about
//! the death. Here the death is the coordinator's own, its status and its words
//! read in this process, and what the hurdles assert is what the dying shell
//! managed to say on its way out. The containment goes; the subject stays.
//!
//! THE TTY-POSITIVE BRANCH IS UNPROVABLE HERE AND IS ASSERTED NOWHERE. A spawned
//! child's streams are pipes, never a terminal, so the fallback arm's "stderr IS
//! a tty, therefore colour" branch cannot be reached from a harness of this
//! shape. The bash fixture recorded the same limit for the same reason; it is a
//! declared narrowing and not an oversight.

#![deny(warnings)]

use but::buth_hurdle::buth_Bench;
use but::buth_hurdle::BUTH_CYAN;
use but::buth_hurdle::BUTH_DIASTEMA;
use but::buth_hurdle::BUTH_ESC;
use but::buth_hurdle::BUTH_GRAY;
use but::buth_hurdle::BUTH_OSC8;

/// The modules a rendering needs. The console library rides along because the two
/// cold-death hurdles reach `buc_die_now` through it.
const BUTY_MODULES: &[&str] = &[
    "buym_yelp.sh",
    "buc_command.sh", //
];

/// The token the verdict probes report under.
const BUTY_SAID: &str = "buty_verdict=";

/// The ambient colour a format is handed, so the hurdle can watch it be restored.
///
/// BRIGHT YELLOW IS CHOSEN FOR BEING NOTHING THE MODULE EMITS ITSELF, which is
/// what lets its reappearance after a marker's close mean "the ambient came
/// back" rather than "some escape was printed".
const BUTY_AMBIENT: &str = "\u{1b}[1;33m";

/// Render one yelp body and hand back what the door said.
///
/// The body opens with a colour configurator and a kindle, exactly as the bash
/// helpers did, and ends by writing the rendered form to stderr — which the
/// dispatch merges, so the hurdle reads it out of one text.
fn buty_render(name: &str, mode: &str, work: &str) -> but::buth_hurdle::buth_Said {
    buty_render_stating(name, mode, work, &[])
}

/// The same render with the operator's inputs a hurdle states on the dispatch.
fn buty_render_stating(
    name: &str,
    mode: &str,
    work: &str,
    stated: &[(&str, &str)],
) -> but::buth_hurdle::buth_Said {
    buth_Bench::buth_seat(
        name,
        BUTY_MODULES,
        &format!(
            "{mode}\nzbuym_kindle\n{work}printf '%b' \"${{z_buym_format}}\" >&2\n",
            mode = mode,
            work = work, //
        ),
    )
    .buth_drive_stating(&[], stated)
}

/// Drive the module's own kindle in a fresh process under a controlled
/// environment, and report which way it resolved.
///
/// BOTH STREAMS OF THE PROBE ARE FILES, never a terminal, which is the condition
/// the fallback hurdle below depends on and the reason the tty-positive branch is
/// out of reach.
fn buty_verdict(name: &str, env: &[&str]) -> but::buth_hurdle::buth_Said {
    let mut probe = String::from("env");
    for word in env {
        probe.push(' ');
        probe.push_str(word);
    }
    probe.push_str(
        " bash -c 'source \"${BURD_BUK_DIR}/buym_yelp.sh\"\n\
         zbuym_kindle\n\
         z_mode=plain\n\
         test -z \"${BUYC_CYAN}\" || z_mode=color\n\
         printf \"",
    );
    probe.push_str(BUTY_SAID);
    probe.push_str("%s|%s\\n\" \"${z_mode}\" \"${z_buym_use_hyperlinks}\"'\n");

    buth_Bench::buth_seat(name, &[], &probe).buth_drive(&[])
}

#[test]
fn buty_a_command_marker_resolves_to_cyan() {
    buty_render(
        "substrate-yelp-cmd",
        "buym_unconditional",
        "buym_cmd_yawp \"git status\"\nbuym_format_yawp \"\" \"${z_buym_yelp}\"\n",
    )
    .buth_thrived()
    .buth_carries(&format!(
        "{}git status",
        BUTH_CYAN, //
    ))
    .buth_lacks(BUTH_DIASTEMA);
}

#[test]
fn buty_a_link_marker_resolves_to_an_osc8_hyperlink() {
    buty_render(
        "substrate-yelp-link",
        "buym_unconditional",
        "buym_link_yawp \"https://example.com\" \"Depot\"\n\
         buym_format_yawp \"\" \"${z_buym_yelp}\"\n",
    )
    .buth_thrived()
    .buth_carries(&format!(
        "{}https://example.com#Depot",
        BUTH_OSC8, //
    ))
    .buth_carries("Depot");
}

#[test]
fn buty_a_link_marker_degrades_to_an_angle_bracket_url() {
    buty_render_stating(
        "substrate-yelp-link-fallback",
        "buym_unconditional",
        "buym_link_yawp \"https://example.com\" \"Depot\"\n\
         buym_format_yawp \"\" \"${z_buym_yelp}\"\n",
        &[("BURE_HYPERLINKS", "0")],
    )
    .buth_thrived()
    .buth_carries("<https://example.com#Depot>")
    .buth_lacks(BUTH_OSC8);
}

#[test]
fn buty_plain_mode_keeps_the_url_beside_a_link_or_href_text() {
    // THE PLAIN MODE DROPPED THE URL where the text was not the URL; both
    // marker kinds keep it now, with no escape sequence of any kind.
    buty_render(
        "substrate-yelp-link-plain",
        "buym_plain",
        "buym_link_yawp \"https://example.com\" \"Depot\"\n\
         buym_format_yawp \"\" \"${z_buym_yelp}\"\n\
         printf '%b\\n' \"${z_buym_format}\" >&2\n\
         buym_href_yawp \"https://example.com/docs\" \"Docs\"\n\
         buym_format_yawp \"\" \"${z_buym_yelp}\"\n",
    )
    .buth_thrived()
    .buth_carries("Depot <https://example.com#Depot>")
    .buth_carries("Docs <https://example.com/docs>")
    .buth_lacks(BUTH_ESC)
    .buth_lacks(BUTH_DIASTEMA);
}

#[test]
fn buty_an_href_marker_degrades_to_an_angle_bracket_url_and_shows_a_bare_url_once() {
    // THE OPERATOR'S FALLBACK FOR HREF, plus the text-equals-URL edge: the URL is
    // not printed a second time where it is already the text.
    buty_render_stating(
        "substrate-yelp-href-fallback",
        "buym_unconditional",
        "buym_href_yawp \"https://example.com/docs\" \"Docs\"\n\
         buym_format_yawp \"\" \"${z_buym_yelp}\"\n\
         printf '%b\\n' \"${z_buym_format}\" >&2\n\
         buym_href_yawp \"https://example.com/same\" \"https://example.com/same\"\n\
         buym_format_yawp \"\" \"${z_buym_yelp}\"\n",
        &[("BURE_HYPERLINKS", "0")],
    )
    .buth_thrived()
    .buth_carries("<https://example.com/docs>")
    .buth_carries("https://example.com/same")
    .buth_lacks("<https://example.com/same>")
    .buth_lacks(BUTH_OSC8);
}

#[test]
fn buty_a_closing_marker_restores_the_ambient_colour() {
    // THE AMBIENT, NOT A RESET. A region that closed by resetting the terminal
    // would look right in isolation and wrong in place: the sentence around it
    // would lose whatever colour it was being written in, at the marker, and stay
    // lost for the rest of the line.
    buty_render(
        "substrate-yelp-ambient",
        "buym_unconditional",
        "buym_cmd_yawp \"test\"\nbuym_format_yawp \"\\033[1;33m\" \"${z_buym_yelp}\"\n",
    )
    .buth_thrived()
    .buth_carries(&format!(
        "{}test{}",
        BUTH_CYAN,
        BUTY_AMBIENT, //
    ));
}

#[test]
fn buty_a_markerless_string_takes_the_fast_path() {
    buty_render(
        "substrate-yelp-fast",
        "buym_unconditional",
        "buym_format_yawp \"\" \"plain text no markers\"\n",
    )
    .buth_thrived()
    .buth_carries("plain text no markers")
    .buth_lacks(BUTH_DIASTEMA);
}

#[test]
fn buty_many_markers_in_one_string_all_resolve() {
    // TWO LINKS AND A COMMAND, because a resolver that handled the FIRST marker
    // and stopped would pass every single-marker hurdle above. Each link opens
    // and closes its own sequence, so two links owe at least four.
    let said = buty_render(
        "substrate-yelp-multi",
        "buym_unconditional",
        "buym_link_yawp \"https://example.com\" \"Vessel\"\nz_vessel=\"${z_buym_yelp}\"\n\
         buym_link_yawp \"https://example.com\" \"Depot\"\nz_depot=\"${z_buym_yelp}\"\n\
         buym_cmd_yawp \"run\"\nz_cmd=\"${z_buym_yelp}\"\n\
         buym_format_yawp \"\" \"A ${z_vessel} in a ${z_depot} via ${z_cmd}.\"\n",
    );

    said.buth_thrived()
        .buth_carries(&format!(
            "{}run",
            BUTH_CYAN, //
        ))
        .buth_lacks(BUTH_DIASTEMA);

    let osc = said.buth_tally(BUTH_OSC8);
    assert!(
        osc >= 4,
        "two links owe at least four hyperlink sequences, found {}:\n{}",
        osc,
        said.buth_text, //
    );
}

#[test]
fn buty_plain_mode_emits_no_escape_at_all() {
    buty_render(
        "substrate-yelp-plain",
        "buym_plain",
        "buym_cmd_yawp \"test\"\nbuym_format_yawp \"\" \"${z_buym_yelp}\"\n",
    )
    .buth_thrived()
    .buth_carries("test")
    .buth_lacks(BUTH_ESC)
    .buth_lacks(BUTH_DIASTEMA);
}

#[test]
fn buty_the_gray_constant_resolves_when_colour_is_on() {
    buty_render(
        "substrate-yelp-gray",
        "buym_unconditional",
        "z_buym_format=\"[${BUYC_GRAY}]\"\n",
    )
    .buth_thrived()
    .buth_carries(&format!(
        "[{}]",
        BUTH_GRAY, //
    ));
}

#[test]
fn buty_the_gray_constant_is_empty_when_colour_is_off() {
    // DECLARED AND EMPTY, NEVER UNSET. A caller dereferences this name under
    // `set -u`, so plain mode has to give it an empty value rather than take it
    // away — the pair with the hurdle above is what says the name is always
    // there and only its content moves.
    buty_render(
        "substrate-yelp-gray-plain",
        "buym_plain",
        "z_buym_format=\"[${BUYC_GRAY}]\"\n",
    )
    .buth_thrived()
    .buth_carries("[]")
    .buth_lacks(BUTH_ESC);
}

#[test]
fn buty_a_stripped_command_marker_yields_bare_text_whatever_the_colour_mode() {
    // THE COLOUR MODE IS DELIBERATELY ON. A strip that consulted the mode would
    // pass this hurdle in plain mode and fail it here, which is the whole reason
    // the mode is set the wrong way round for what is being asserted.
    buty_render(
        "substrate-yelp-strip-cmd",
        "buym_unconditional",
        "buym_cmd_yawp \"git status\"\nbuym_strip_yawp \"Run ${z_buym_yelp} now\"\n",
    )
    .buth_thrived()
    .buth_carries("Run git status now")
    .buth_lacks(BUTH_ESC)
    .buth_lacks(BUTH_DIASTEMA);
}

#[test]
fn buty_a_stripped_link_degrades_to_text_and_a_readable_url() {
    // THE URL SURVIVES THE STRIP. A strip that kept only the display text would
    // read cleanly and lose the one thing a reader outside a terminal needs.
    buty_render(
        "substrate-yelp-strip-link",
        "buym_unconditional",
        "buym_link_yawp \"https://example.com\" \"Depot\"\n\
         buym_strip_yawp \"See ${z_buym_yelp}.\"\n",
    )
    .buth_thrived()
    .buth_carries("See Depot <https://example.com#Depot>.")
    .buth_lacks(BUTH_OSC8)
    .buth_lacks(BUTH_ESC);
}

#[test]
fn buty_a_stripped_href_degrades_the_same_way() {
    buty_render(
        "substrate-yelp-strip-href",
        "buym_unconditional",
        "buym_href_yawp \"https://example.com\" \"Docs\"\n\
         buym_strip_yawp \"${z_buym_yelp}\"\n",
    )
    .buth_thrived()
    .buth_carries("Docs <https://example.com>")
    .buth_lacks(BUTH_ESC);
}

#[test]
fn buty_a_markerless_string_takes_the_strips_fast_path() {
    buty_render(
        "substrate-yelp-strip-fast",
        "buym_unconditional",
        "buym_strip_yawp \"plain text no markers\"\n",
    )
    .buth_thrived()
    .buth_carries("plain text no markers");
}

#[test]
fn buty_a_cold_death_kindles_the_module_and_renders_its_sigil() {
    // COLD MEANS THE MODULE IS SOURCED AND NEVER KINDLED, which is the state a
    // door dies in when it dies early. The death path has to kindle lazily, and
    // the failure it is guarding against is not a wrong colour: it is a
    // dereference of an unset readonly under `set -u`, which kills the shell
    // BEFORE the diagnostic and leaves the operator with an unbound-variable
    // message about the logging module instead of the error that actually
    // happened.
    let said = buth_Bench::buth_seat(
        "substrate-yelp-cold",
        BUTY_MODULES,
        "buym_unconditional\nbuc_context \"cold-ctx\"\nbuc_die_now \"cold boom\"\n",
    )
    .buth_drive(&[]);

    said.buth_took(1)
        .buth_lacks("unbound variable")
        .buth_carries("ERROR:")
        .buth_carries("cold boom")
        .buth_carries(&format!(
            "{}cold-ctx",
            BUTH_GRAY, //
        ));
}

#[test]
fn buty_a_cold_death_under_plain_mode_suppresses_the_sigil() {
    // THE LAZY KINDLE MUST STILL HONOUR THE MODE. A death path that kindled
    // itself into colour regardless would put escapes into a stream the operator
    // had asked to keep clean, and would do it at exactly the moment they are
    // reading carefully.
    let said = buth_Bench::buth_seat(
        "substrate-yelp-cold-plain",
        BUTY_MODULES,
        "buym_plain\nbuc_context \"cold-ctx\"\nbuc_die_now \"cold boom\"\n",
    )
    .buth_drive(&[]);

    said.buth_took(1)
        .buth_carries("ERROR:")
        .buth_carries("cold boom")
        .buth_lacks(BUTH_GRAY);
}

#[test]
fn buty_a_dispatch_verdict_against_colour_beats_a_favourable_terminal() {
    buty_verdict(
        "substrate-yelp-verdict-zero",
        &[
            "-u",
            "NO_COLOR",
            "-u",
            "BURD_HYPERLINKS",
            "BURD_COLOR=0",
            "TERM=xterm-256color",
        ],
    )
    .buth_thrived()
    .buth_carries(&format!(
        "{}plain|0",
        BUTY_SAID, //
    ));
}

#[test]
fn buty_a_dispatch_verdict_for_colour_beats_an_unfavourable_terminal() {
    // THE PAIR IS THE POINT: the verdict wins in BOTH directions, so a module
    // that merely happened to agree with the terminal would fail one of the two.
    buty_verdict(
        "substrate-yelp-verdict-one",
        &[
            "-u",
            "NO_COLOR",
            "-u",
            "BURD_HYPERLINKS",
            "BURD_COLOR=1",
            "TERM=dumb",
        ],
    )
    .buth_thrived()
    .buth_carries(&format!(
        "{}color|1",
        BUTY_SAID, //
    ));
}

#[test]
fn buty_with_no_verdict_a_pipe_stays_plain_however_favourable_the_terminal() {
    // THE TERMINAL SAYS COLOUR AND THE STREAM IS A PIPE, and the stream wins.
    // Before this branch existed an environment shaped exactly like this one
    // coloured, which is how escape sequences reach a file that something else
    // will later have to read.
    buty_verdict(
        "substrate-yelp-fallback",
        &[
            "-u",
            "BURD_COLOR",
            "-u",
            "NO_COLOR",
            "-u",
            "BURD_HYPERLINKS",
            "TERM=xterm-256color",
        ],
    )
    .buth_thrived()
    .buth_carries(&format!(
        "{}plain|0",
        BUTY_SAID, //
    ));
}
// eof

/// The launch stand's dirname beneath a target infield, spelled here and never
/// read from the module, so a wrong constant in the module turns the hurdle red.
const BUTY_STAND: &str = "infield/jjqy_equerry";

/// A colophon planted in the stand and in the bench's own tabtarget directory.
const BUTY_STOOD: &str = "zavrw-q";

/// A colophon planted in the bench's own tabtarget directory alone.
const BUTY_UNSTOOD: &str = "zavrw-y";

/// A target stand's door resolves under that infield's stand and never under the
/// tabtarget directory of the tree the resolver runs in.
///
/// BOTH COLOPHONS ALSO STAND IN THE BENCH'S OWN TABTARGET DIRECTORY, so a
/// resolver that fell back to the forge-relative glob would answer each of them
/// from there: the found path would lie under the bench's directory and the
/// absent one would not come back empty. The forge-relative glob's own answer is
/// read beside them as the control that those plants are where it looks.
#[test]
fn buty_a_stand_door_resolves_under_the_target_stand() {
    let bench = buth_Bench::buth_seat(
        "yelp-stand-door",
        BUTY_MODULES,
        &format!(
            "zbuym_kindle\n\
             buym_stand_tt_path infield {stood}\n\
             printf '%s\\n' \"${{z_buym_stand_tt_path}}\" > found.txt\n\
             buym_stand_tt_path infield {unstood}\n\
             printf '%s\\n' \"${{z_buym_stand_tt_path}}\" > absent.txt\n\
             buym_tt_path {stood}\n\
             printf '%s\\n' \"${{z_buym_tt_path}}\" > forge.txt\n\
             printf '%s\\n' \"${{BURD_TABTARGET_DIR}}\" > tabtargets.txt\n",
            stood = BUTY_STOOD,
            unstood = BUTY_UNSTOOD, //
        ),
    );

    let root = bench.buth_root().to_path_buf();
    for relative in [
        format!(
            "{}/tt/{}.Plimmet.sh",
            BUTY_STAND,
            BUTY_STOOD, //
        ),
        format!(
            "tt/{}.Plimmet.sh",
            BUTY_STOOD, //
        ),
        format!(
            "tt/{}.Plimmet.sh",
            BUTY_UNSTOOD, //
        ),
    ] {
        let path = root.join(&relative);
        std::fs::create_dir_all(path.parent().expect("a planted door stands in a directory"))
            .expect("the seat takes the planted directories");
        std::fs::write(&path, "#!/bin/bash\n").expect("the seat takes the planted door");
    }

    bench.buth_drive(&[]).buth_thrived();

    let tabtargets = bench.buth_read("tabtargets.txt").trim().to_string();
    let forge = bench.buth_read("forge.txt").trim().to_string();
    assert_eq!(
        forge,
        format!(
            "{}/{}.Plimmet.sh",
            tabtargets,
            BUTY_STOOD, //
        ),
        "the control: the forge-relative glob finds the colophon in the bench's own directory", //
    );

    let found = bench.buth_read("found.txt").trim().to_string();
    assert_eq!(
        found,
        format!(
            "{}/tt/{}.Plimmet.sh",
            BUTY_STAND,
            BUTY_STOOD, //
        ),
        "the stand's door is composed under the planted stand", //
    );
    assert!(
        !found.starts_with(&format!(
            "{}/",
            tabtargets, //
        )),
        "the stand's door never lies under the bench's own tabtarget directory: {}",
        found, //
    );

    assert_eq!(
        bench.buth_read("absent.txt").trim(),
        "",
        "a colophon the stand does not carry yields empty", //
    );
}
