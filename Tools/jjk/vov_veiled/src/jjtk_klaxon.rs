// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

use super::jjrk_klaxon::*;

#[test]
fn jjtk_render_shape() {
    let line = jjrk_render(jjrk_Level::Info, "jjrx_example.rs", 42, "loaded 3 entries");
    assert_eq!(line, "[INFO] [jjrx_example.rs:42] loaded 3 entries");
}

#[test]
fn jjtk_render_every_level_tag() {
    let cases = [
        (jjrk_Level::Trace, "TRACE"),
        (jjrk_Level::Info, "INFO"),
        (jjrk_Level::Error, "ERROR"),
        (jjrk_Level::Fatal, "FATAL"),
    ];
    for (level, tag) in cases {
        let line = jjrk_render(level, "f.rs", 1, "m");
        assert!(line.starts_with(&format!("[{}] ", tag)), "line {:?} missing tag {}", line, tag);
    }
}

#[test]
fn jjtk_render_one_line_no_trailing_newline() {
    let line = jjrk_render(jjrk_Level::Error, "f.rs", 7, "boom");
    assert!(!line.contains('\n'));
}
