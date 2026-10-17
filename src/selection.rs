use std::ops::RangeInclusive;

use crate::{Cursor, Result};

/// Inclusive rectangle between an anchor cell and a focus cell.
///
/// The focus is the cursor position at the moving end of the selection. Row and
/// column order are normalized, so either cell may be the top-left. On a jagged
/// table, cells past a row's width are left out of the range. On a sparse table,
/// [`Self::cells_rows`] also leaves out cells missing in the middle of a row.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Selection {
    anchor_line: usize,
    anchor_column: usize,
    focus_line: usize,
    focus_column: usize,
}

impl Selection {
    /// Rectangle from `anchor` to `focus`. Positions are not checked against a table.
    pub fn new(anchor: (usize, usize), focus: (usize, usize)) -> Self {
        Self {
            anchor_line: anchor.0,
            anchor_column: anchor.1,
            focus_line: focus.0,
            focus_column: focus.1,
        }
    }

    /// Collapsed selection on the cursor's current cell.
    pub fn at_cursor(cursor: &Cursor) -> Self {
        Self::new(cursor.current(), cursor.current())
    }

    /// Anchor at `anchor`, focus at the cursor's current cell.
    ///
    /// `anchor` must be a cell on `cursor`'s table.
    pub fn anchored(cursor: &Cursor, anchor: (usize, usize)) -> Result<Self> {
        cursor.validate_cell(anchor.0, anchor.1)?;
        Ok(Self::new(anchor, cursor.current()))
    }

    pub fn anchor(&self) -> (usize, usize) {
        (self.anchor_line, self.anchor_column)
    }

    pub fn focus(&self) -> (usize, usize) {
        (self.focus_line, self.focus_column)
    }

    /// Replace the focus, leaving the anchor in place.
    pub fn with_focus(mut self, focus: (usize, usize)) -> Self {
        self.focus_line = focus.0;
        self.focus_column = focus.1;
        self
    }

    /// Inclusive line bounds, with the smaller line first.
    pub fn lines(&self) -> RangeInclusive<usize> {
        let (start, end) = self.line_bounds();
        start..=end
    }

    /// Inclusive column bounds, with the smaller column first.
    pub fn columns(&self) -> RangeInclusive<usize> {
        let (start, end) = self.column_bounds();
        start..=end
    }

    /// Whether `position` lies in the rectangle and exists on that row.
    pub fn contains(&self, column_sizes: &[usize], position: (usize, usize)) -> bool {
        let (line, column) = position;
        let (line_start, line_end) = self.line_bounds();
        let (column_start, column_end) = self.column_bounds();
        if line < line_start || line > line_end || column < column_start || column > column_end {
            return false;
        }
        column_sizes.get(line).is_some_and(|width| column < *width)
    }

    /// Whether `position` lies in the rectangle and is one of the present columns.
    ///
    /// `rows` is the present columns of each row, as from [`Cursor::row_columns`](crate::Cursor::row_columns).
    /// A hole inside the rectangle is not contained.
    pub fn contains_rows(&self, rows: &[Vec<usize>], position: (usize, usize)) -> bool {
        let (line, column) = position;
        let (line_start, line_end) = self.line_bounds();
        let (column_start, column_end) = self.column_bounds();
        if line < line_start || line > line_end || column < column_start || column > column_end {
            return false;
        }
        rows.get(line)
            .is_some_and(|columns| columns.binary_search(&column).is_ok())
    }

    /// Cells inside the rectangle that exist in `column_sizes`, in row-major order.
    ///
    /// `column_sizes` describes prefix rows: every column in `0..width` exists.
    /// For rows with holes, use [`Self::cells_rows`].
    pub fn cells<'a>(&self, column_sizes: &'a [usize]) -> SelectionCells<'a> {
        let (line_start, line_end) = self.line_bounds();
        let (column_start, column_end) = self.column_bounds();
        SelectionCells {
            line_end,
            column_start,
            column_end,
            column_sizes,
            line: line_start,
            column: column_start,
            finished: false,
        }
    }

    /// Present cells inside the rectangle, in row-major order.
    ///
    /// `rows` is the present columns of each row. Holes and columns past a row are omitted.
    pub fn cells_rows<'a>(&self, rows: &'a [Vec<usize>]) -> SelectionRowCells<'a> {
        let (line_start, line_end) = self.line_bounds();
        let (column_start, column_end) = self.column_bounds();
        SelectionRowCells {
            line_end,
            column_start,
            column_end,
            rows,
            line: line_start,
            index: 0,
            positioned: false,
            finished: false,
        }
    }

    fn line_bounds(&self) -> (usize, usize) {
        if self.anchor_line <= self.focus_line {
            (self.anchor_line, self.focus_line)
        } else {
            (self.focus_line, self.anchor_line)
        }
    }

    fn column_bounds(&self) -> (usize, usize) {
        if self.anchor_column <= self.focus_column {
            (self.anchor_column, self.focus_column)
        } else {
            (self.focus_column, self.anchor_column)
        }
    }
}

/// Row-major walk of the cells in a [`Selection`].
#[derive(Debug, Clone)]
pub struct SelectionCells<'a> {
    line_end: usize,
    column_start: usize,
    column_end: usize,
    column_sizes: &'a [usize],
    line: usize,
    column: usize,
    finished: bool,
}

impl Iterator for SelectionCells<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        while self.line <= self.line_end {
            if self.line >= self.column_sizes.len() {
                break;
            }
            let width = self.column_sizes[self.line];
            if width > self.column_start {
                let last = self.column_end.min(width - 1);
                if self.column <= last {
                    let cell = (self.line, self.column);
                    if self.column == last {
                        self.line += 1;
                        self.column = self.column_start;
                    } else {
                        self.column += 1;
                    }
                    return Some(cell);
                }
            }
            self.line += 1;
            self.column = self.column_start;
        }
        self.finished = true;
        None
    }
}

/// Row-major walk of the present cells in a [`Selection`].
#[derive(Debug, Clone)]
pub struct SelectionRowCells<'a> {
    line_end: usize,
    column_start: usize,
    column_end: usize,
    rows: &'a [Vec<usize>],
    line: usize,
    index: usize,
    positioned: bool,
    finished: bool,
}

impl Iterator for SelectionRowCells<'_> {
    type Item = (usize, usize);

    fn next(&mut self) -> Option<Self::Item> {
        if self.finished {
            return None;
        }
        while self.line <= self.line_end && self.line < self.rows.len() {
            let columns = &self.rows[self.line];
            if !self.positioned {
                self.index = match columns.binary_search(&self.column_start) {
                    Ok(index) | Err(index) => index,
                };
                self.positioned = true;
            }
            if self.index < columns.len() && columns[self.index] <= self.column_end {
                let cell = (self.line, columns[self.index]);
                self.index += 1;
                return Some(cell);
            }
            self.line += 1;
            self.positioned = false;
        }
        self.finished = true;
        None
    }
}
