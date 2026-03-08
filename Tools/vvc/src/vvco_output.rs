// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! VVC Output - Context-aware output abstraction
//!
//! Provides a unified output channel that behaves differently depending on
//! invocation context:
//!
//! - **Console mode**: `vvco_out!` writes to stdout, `vvco_err!` writes to stderr.
//!   Standard CLI behavior where the terminal is visible.
//!
//! - **Buffer mode**: Both `vvco_out!` and `vvco_err!` accumulate into an internal
//!   String buffer. Used by MCP server handlers where stderr is swallowed by the
//!   transport layer and must be captured for return via the MCP protocol.
//!
//! # Usage
//!
//! ```ignore
//! // MCP handler (buffer mode)
//! let mut output = vvco_Output::buffer();
//! vvco_out!(output, "committed {}", hash);
//! vvco_err!(output, "warning: size near limit");
//! let text = output.vvco_finish();  // returns accumulated String
//!
//! // CLI entry point (console mode)
//! let mut output = vvco_Output::console();
//! vvco_out!(output, "committed {}", hash);  // prints to stdout
//! vvco_err!(output, "error: {}", msg);      // prints to stderr
//! let text = output.vvco_finish();           // returns empty (already printed)
//! ```

#![allow(non_camel_case_types)]

/// Invocation context determining output routing.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum vvco_Mode {
    /// Direct terminal invocation — stdout and stderr visible to user.
    Console,
    /// MCP transport invocation — stderr swallowed, all output must be buffered.
    Buffer,
}

/// Context-aware output accumulator.
///
/// Routes output lines to stdout/stderr (console) or an internal buffer (MCP)
/// depending on the mode set at construction.
#[derive(Debug)]
pub struct vvco_Output {
    mode: vvco_Mode,
    buffer: String,
}

impl vvco_Output {
    /// Create output in console mode.
    ///
    /// `vvco_out!` writes to stdout, `vvco_err!` writes to stderr.
    /// `vvco_finish()` returns whatever was buffered (empty for pure console use).
    pub fn console() -> Self {
        Self {
            mode: vvco_Mode::Console,
            buffer: String::new(),
        }
    }

    /// Create output in buffer mode.
    ///
    /// Both `vvco_out!` and `vvco_err!` accumulate into the internal buffer.
    /// `vvco_finish()` returns the accumulated text.
    pub fn buffer() -> Self {
        Self {
            mode: vvco_Mode::Buffer,
            buffer: String::new(),
        }
    }

    /// Append a standard output line.
    ///
    /// Console mode: prints to stdout.
    /// Buffer mode: appends to internal buffer.
    pub fn vvco_out_line(&mut self, line: &str) {
        match self.mode {
            vvco_Mode::Console => println!("{}", line),
            vvco_Mode::Buffer => {
                self.buffer.push_str(line);
                self.buffer.push('\n');
            }
        }
    }

    /// Append an error/warning output line.
    ///
    /// Console mode: prints to stderr.
    /// Buffer mode: appends to internal buffer.
    pub fn vvco_err_line(&mut self, line: &str) {
        match self.mode {
            vvco_Mode::Console => eprintln!("{}", line),
            vvco_Mode::Buffer => {
                self.buffer.push_str(line);
                self.buffer.push('\n');
            }
        }
    }

    /// Consume the output and return the accumulated buffer.
    ///
    /// Console mode: returns empty string (output already printed).
    /// Buffer mode: returns all accumulated text.
    pub fn vvco_finish(self) -> String {
        self.buffer
    }
}

/// Write a formatted standard output line.
///
/// Console mode: prints to stdout.
/// Buffer mode: appends to internal buffer.
#[macro_export]
macro_rules! vvco_out {
    ($out:expr, $($arg:tt)*) => {
        $out.vvco_out_line(&format!($($arg)*))
    };
}

/// Write a formatted error/warning output line.
///
/// Console mode: prints to stderr.
/// Buffer mode: appends to internal buffer.
#[macro_export]
macro_rules! vvco_err {
    ($out:expr, $($arg:tt)*) => {
        $out.vvco_err_line(&format!($($arg)*))
    };
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffer_mode_accumulates() {
        let mut output = vvco_Output::buffer();
        vvco_out!(output, "line {}", 1);
        vvco_err!(output, "error: {}", "bad");
        vvco_out!(output, "line {}", 2);
        let text = output.vvco_finish();
        assert_eq!(text, "line 1\nerror: bad\nline 2\n");
    }

    #[test]
    fn test_console_mode_returns_empty() {
        let mut output = vvco_Output::console();
        // These would print to stdout/stderr in a real terminal
        vvco_out!(output, "hello");
        vvco_err!(output, "warning");
        let text = output.vvco_finish();
        // Console mode prints directly; buffer is empty
        assert_eq!(text, "");
    }

    #[test]
    fn test_buffer_empty_finish() {
        let output = vvco_Output::buffer();
        assert_eq!(output.vvco_finish(), "");
    }

    #[test]
    fn test_mode_query() {
        let console = vvco_Output::console();
        assert_eq!(console.mode, vvco_Mode::Console);

        let buffer = vvco_Output::buffer();
        assert_eq!(buffer.mode, vvco_Mode::Buffer);
    }
}
