// Copyright 2026 Scale Invariant, Inc.
// All rights reserved.
// SPDX-License-Identifier: LicenseRef-Proprietary

//! Column table formatting for JJK output
//!
//! Provides structured column layout to eliminate magic numbers/strings
//! from formatted output operations. Single source of truth for headers,
//! widths, and alignment.

/// Gap between columns
pub const JJRP_COLUMN_GAP: usize = 2;

/// Minimum column width
pub const JJRP_MIN_WIDTH: usize = 5;

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
}

impl jjrp_Column {
    /// Create a new column with default min_width from header
    pub fn new(header: &'static str, align: jjrp_Align) -> Self {
        Self {
            header,
            min_width: header.len().max(JJRP_MIN_WIDTH),
            align,
        }
    }

    /// Create a new column with explicit min_width
    pub fn with_width(header: &'static str, min_width: usize, align: jjrp_Align) -> Self {
        Self {
            header,
            min_width: min_width.max(JJRP_MIN_WIDTH),
            align,
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
                self.widths[i] = self.widths[i].max(value.len());
            }
        }
    }

    /// Print the header row
    pub fn jjrp_print_header(&self) {
        let mut parts = Vec::new();
        for (i, col) in self.columns.iter().enumerate() {
            let width = self.widths[i];
            let formatted = match col.align {
                jjrp_Align::Left => format!("{:<width$}", col.header, width = width),
                jjrp_Align::Right => format!("{:>width$}", col.header, width = width),
            };
            parts.push(formatted);
        }
        println!("{}", parts.join(&" ".repeat(JJRP_COLUMN_GAP)));
    }

    /// Print a separator line using computed widths
    /// Uses box drawing character to avoid markdown interpretation of dashes
    pub fn jjrp_print_separator(&self) {
        let mut parts = Vec::new();
        for width in &self.widths {
            parts.push("─".repeat(*width));
        }
        println!("{}", parts.join(&" ".repeat(JJRP_COLUMN_GAP)));
    }

    /// Print a data row
    pub fn jjrp_print_row(&self, values: &[&str]) {
        let mut parts = Vec::new();
        for (i, value) in values.iter().enumerate() {
            if i < self.columns.len() {
                let width = self.widths[i];
                let col = &self.columns[i];
                let formatted = match col.align {
                    jjrp_Align::Left => format!("{:<width$}", value, width = width),
                    jjrp_Align::Right => format!("{:>width$}", value, width = width),
                };
                parts.push(formatted);
            }
        }
        println!("{}", parts.join(&" ".repeat(JJRP_COLUMN_GAP)));
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

        // These would print to stdout in actual usage
        // Just verify they don't panic
        table.jjrp_print_header();
        table.jjrp_print_separator();
        table.jjrp_print_row(&["₣AB", "test-heat", "42"]);
        table.jjrp_print_row(&["₣CD", "another-heat-name", "100"]);
    }
}
