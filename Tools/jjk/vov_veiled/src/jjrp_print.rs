// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Column table formatting for JJK output
//!
//! Provides structured column layout to eliminate magic numbers/strings
//! from formatted output operations. Single source of truth for headers,
//! widths, and alignment.

use vvc::{vvco_out, vvco_Output};

/// Gap between columns
pub const JJRP_COLUMN_GAP: usize = 2;

/// Minimum column width
pub const JJRP_MIN_WIDTH: usize = 5;

/// Trailing character marking a clipped value
pub const JJRP_ELLIPSIS: char = '…';

/// Column alignment
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum jjrp_Align {
    Left,
    Right,
}

/// Column definition
#[derive(Debug, Clone)]
pub struct jjrp_Column {
    /// Header text (single source of truth)
    pub header: &'static str,
    /// Minimum width (defaults to header.len())
    pub min_width: usize,
    /// Alignment
    pub align: jjrp_Align,
    /// Maximum characters a value may occupy; longer values are clipped
    pub cap: Option<usize>,
}

impl jjrp_Column {
    /// Create a new column with default min_width from header
    pub fn new(header: &'static str, align: jjrp_Align) -> Self {
        Self {
            header,
            min_width: header.len().max(JJRP_MIN_WIDTH),
            align,
            cap: None,
        }
    }

    /// Create a new column with explicit min_width
    pub fn with_width(header: &'static str, min_width: usize, align: jjrp_Align) -> Self {
        Self {
            header,
            min_width: min_width.max(JJRP_MIN_WIDTH),
            align,
            cap: None,
        }
    }

    /// Create a column whose values are clipped to `cap` characters, the last
    /// an ellipsis. Bounds a column carrying unbounded free text — a commit
    /// subject — so row count alone sets the table's size.
    pub fn with_cap(header: &'static str, cap: usize, align: jjrp_Align) -> Self {
        Self {
            header,
            min_width: header.len().max(JJRP_MIN_WIDTH),
            align,
            cap: Some(cap.max(JJRP_MIN_WIDTH)),
        }
    }
}

/// Table with columns and computed widths
pub struct jjrp_Table {
    columns: Vec<jjrp_Column>,
    widths: Vec<usize>,
}

impl jjrp_Table {
    /// Create a new table with the given columns
    pub fn jjrp_new(columns: Vec<jjrp_Column>) -> Self {
        let widths = columns.iter().map(|c| c.min_width).collect();
        Self { columns, widths }
    }

    /// Update column widths based on a data row
    pub fn jjrp_measure(&mut self, row: &[&str]) {
        for (i, value) in row.iter().enumerate() {
            if i < self.widths.len() {
                let clipped = self.jjrp_clip(i, value);
                self.widths[i] = self.widths[i].max(clipped.len());
            }
        }
    }

    /// Clip a value to its column's cap, if the column carries one.
    /// Clipping counts characters, not bytes, so a multi-byte subject never
    /// splits mid-codepoint. Measure and row emission both route through this,
    /// so a capped column's width is computed from what is actually printed.
    pub fn jjrp_clip(&self, index: usize, value: &str) -> String {
        match self.columns.get(index).and_then(|c| c.cap) {
            Some(cap) if value.chars().count() > cap => {
                let kept: String = value.chars().take(cap - 1).collect();
                format!("{}{}", kept, JJRP_ELLIPSIS)
            }
            _ => value.to_string(),
        }
    }

    /// Write the header row to output
    pub fn jjrp_write_header(&self, output: &mut vvco_Output) {
        let mut parts = Vec::new();
        for (i, col) in self.columns.iter().enumerate() {
            let width = self.widths[i];
            let formatted = match col.align {
                jjrp_Align::Left => format!("{:<width$}", col.header, width = width),
                jjrp_Align::Right => format!("{:>width$}", col.header, width = width),
            };
            parts.push(formatted);
        }
        vvco_out!(output, "{}", parts.join(&" ".repeat(JJRP_COLUMN_GAP)));
    }

    /// Write a separator line to output
    /// Uses box drawing character to avoid markdown interpretation of dashes
    pub fn jjrp_write_separator(&self, output: &mut vvco_Output) {
        let mut parts = Vec::new();
        for width in &self.widths {
            parts.push("─".repeat(*width));
        }
        vvco_out!(output, "{}", parts.join(&" ".repeat(JJRP_COLUMN_GAP)));
    }

    /// Write a data row to output
    pub fn jjrp_write_row(&self, output: &mut vvco_Output, values: &[&str]) {
        let mut parts = Vec::new();
        for (i, value) in values.iter().enumerate() {
            if i < self.columns.len() {
                let width = self.widths[i];
                let col = &self.columns[i];
                let value = self.jjrp_clip(i, value);
                let formatted = match col.align {
                    jjrp_Align::Left => format!("{:<width$}", value, width = width),
                    jjrp_Align::Right => format!("{:>width$}", value, width = width),
                };
                parts.push(formatted);
            }
        }
        vvco_out!(output, "{}", parts.join(&" ".repeat(JJRP_COLUMN_GAP)));
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_column_new() {
        let col = jjrp_Column::new("Test", jjrp_Align::Left);
        assert_eq!(col.header, "Test");
        assert_eq!(col.min_width, JJRP_MIN_WIDTH);
        assert_eq!(col.align, jjrp_Align::Left);
    }

    #[test]
    fn test_column_with_width() {
        let col = jjrp_Column::with_width("ID", 10, jjrp_Align::Right);
        assert_eq!(col.header, "ID");
        assert_eq!(col.min_width, 10);
        assert_eq!(col.align, jjrp_Align::Right);
    }

    #[test]
    fn test_table_measure() {
        let columns = vec![
            jjrp_Column::new("Name", jjrp_Align::Left),
            jjrp_Column::new("ID", jjrp_Align::Right),
        ];
        let mut table = jjrp_Table::jjrp_new(columns);

        // Initial widths are min_width
        assert_eq!(table.widths[0], JJRP_MIN_WIDTH);
        assert_eq!(table.widths[1], JJRP_MIN_WIDTH);

        // Measure a row with longer values
        table.jjrp_measure(&["Very Long Name", "12345"]);
        assert_eq!(table.widths[0], 14); // "Very Long Name".len()
        assert_eq!(table.widths[1], JJRP_MIN_WIDTH); // Still min because "12345" is shorter
    }

    #[test]
    fn test_column_with_cap() {
        let col = jjrp_Column::with_cap("Subject", 40, jjrp_Align::Left);
        assert_eq!(col.cap, Some(40));
        assert_eq!(col.min_width, "Subject".len());
    }

    #[test]
    fn test_capped_column_clips_and_bounds_width() {
        let columns = vec![
            jjrp_Column::new("Commit", jjrp_Align::Left),
            jjrp_Column::with_cap("Subject", 10, jjrp_Align::Left),
        ];
        let mut table = jjrp_Table::jjrp_new(columns);

        let long = "abcdefghijklmnopqrstuvwxyz";
        assert_eq!(table.jjrp_clip(1, long), "abcdefghi…");
        // A value at or under the cap passes through whole.
        assert_eq!(table.jjrp_clip(1, "abcdefghij"), "abcdefghij");
        // An uncapped column never clips.
        assert_eq!(table.jjrp_clip(0, long), long);

        // Measuring a long value widens the column only to the clipped form,
        // never to the raw subject.
        table.jjrp_measure(&["abc1234", long]);
        assert!(table.widths[1] < long.len());
    }

    #[test]
    fn test_clip_respects_char_boundaries() {
        let columns = vec![jjrp_Column::with_cap("Subject", 6, jjrp_Align::Left)];
        let table = jjrp_Table::jjrp_new(columns);
        // Multi-byte chars: clipping counts characters, so this never panics
        // on a byte slice landing inside a codepoint.
        assert_eq!(table.jjrp_clip(0, "₣AA ₢AAAAp coronet"), "₣AA ₢…");
    }

    #[test]
    fn test_table_output() {
        let columns = vec![
            jjrp_Column::new("₣Fire", jjrp_Align::Left),
            jjrp_Column::new("Silks", jjrp_Align::Left),
            jjrp_Column::new("Count", jjrp_Align::Right),
        ];
        let mut table = jjrp_Table::jjrp_new(columns);

        // Measure some data
        table.jjrp_measure(&["₣AB", "test-heat", "42"]);
        table.jjrp_measure(&["₣CD", "another-heat-name", "100"]);

        // Write to output and verify they don't panic
        let mut output = vvco_Output::buffer();
        table.jjrp_write_header(&mut output);
        table.jjrp_write_separator(&mut output);
        table.jjrp_write_row(&mut output, &["₣AB", "test-heat", "42"]);
        table.jjrp_write_row(&mut output, &["₣CD", "another-heat-name", "100"]);
        let text = output.vvco_finish();
        assert!(!text.is_empty());
    }
}
