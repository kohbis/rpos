use crate::{Error, Result};

/// How movement behaves at an edge. Vertical and horizontal edges are set separately.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WrapMode {
    /// Continue from the opposite edge.
    Wrap,
    /// Stay on this edge.
    Clamp,
}

/// How vertical movement handles jagged rows that lack the current column.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VerticalPolicy {
    /// Move to the adjacent row and clamp the column to that row's width (default).
    ClampColumn,
    /// Keep the current column and skip rows that do not have it.
    /// Useful when column identity should be preserved across jagged rows.
    SkipMissing,
    /// Remember a preferred column across up/down.
    /// Rows that have that column restore it; rows that lack it, including a hole
    /// at that column, land on their last cell.
    /// Horizontal movement and explicit column changes update the preferred column.
    StickyColumn,
}

/// How horizontal movement behaves at the start/end of a row.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum HorizontalPolicy {
    /// left/right stay within the current row (default).
    WithinRow,
    /// At row edges, left/right move to the previous/next row.
    AcrossRows,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Cursor {
    line: usize,
    column: usize,
    /// Present columns on each row, ascending and unique.
    /// A prefix row of width `n` is `0..n`.
    row_columns: Vec<Vec<usize>>,
    /// One past the last present column of each row. For a prefix row this is its width.
    column_sizes: Vec<usize>,
    vertical_wrap_mode: WrapMode,
    horizontal_wrap_mode: WrapMode,
    vertical_policy: VerticalPolicy,
    horizontal_policy: HorizontalPolicy,
    /// Column to restore on `VerticalPolicy::StickyColumn` up/down.
    /// Tracks the last explicit column. Other vertical policies leave it untouched.
    preferred_column: usize,
}

impl Cursor {
    pub fn new((line_size, column_size): (usize, usize)) -> Result<Self> {
        if line_size == 0 || column_size == 0 {
            return Err(Error::InvalidColumnSizes);
        }
        let column_sizes = vec![column_size; line_size];
        Ok(Self::from_prefix(column_sizes))
    }

    pub fn new_jagged(column_sizes: Vec<usize>) -> Result<Self> {
        if column_sizes.is_empty() || column_sizes.contains(&0) {
            return Err(Error::InvalidColumnSizes);
        }
        Ok(Self::from_prefix(column_sizes))
    }

    /// Rows of present column indexes.
    ///
    /// Each row must contain at least one column. Duplicates are rejected.
    /// Order does not matter. A row of `0..n` is the same shape as a jagged width of `n`.
    /// Columns that are absent in the middle of a row are holes: movement, indexes,
    /// and selection skip them.
    pub fn new_sparse(rows: Vec<Vec<usize>>) -> Result<Self> {
        let (row_columns, column_sizes) = prepare_sparse(rows)?;
        let column = row_columns[0][0];
        Ok(Self {
            line: 0,
            column,
            row_columns,
            column_sizes,
            vertical_wrap_mode: WrapMode::Wrap,
            horizontal_wrap_mode: WrapMode::Wrap,
            vertical_policy: VerticalPolicy::ClampColumn,
            horizontal_policy: HorizontalPolicy::WithinRow,
            preferred_column: column,
        })
    }

    fn from_prefix(column_sizes: Vec<usize>) -> Self {
        let row_columns = column_sizes
            .iter()
            .map(|&width| (0..width).collect())
            .collect();
        Self {
            line: 0,
            column: 0,
            row_columns,
            column_sizes,
            vertical_wrap_mode: WrapMode::Wrap,
            horizontal_wrap_mode: WrapMode::Wrap,
            vertical_policy: VerticalPolicy::ClampColumn,
            horizontal_policy: HorizontalPolicy::WithinRow,
            preferred_column: 0,
        }
    }

    pub fn current(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn column(&self) -> usize {
        self.column
    }

    pub fn line_count(&self) -> usize {
        self.column_sizes.len()
    }

    /// One past the last present column of each row.
    ///
    /// A prefix row of width `n` reports `n`. A sparse row reports one past its
    /// rightmost cell, which can be larger than the number of cells on that row.
    pub fn column_sizes(&self) -> &[usize] {
        &self.column_sizes
    }

    /// Present columns of each row, in ascending order.
    pub fn row_columns(&self) -> &[Vec<usize>] {
        &self.row_columns
    }

    /// Whether `(line, column)` is a present cell.
    pub fn has_cell(&self, line: usize, column: usize) -> bool {
        self.row_columns
            .get(line)
            .is_some_and(|columns| columns.binary_search(&column).is_ok())
    }

    /// `Some(mode)` when both axes use `mode`.
    pub fn wrap_mode(&self) -> Option<WrapMode> {
        if self.vertical_wrap_mode == self.horizontal_wrap_mode {
            Some(self.vertical_wrap_mode)
        } else {
            None
        }
    }

    /// Set both the vertical and horizontal wrap modes.
    pub fn set_wrap_mode(&mut self, wrap_mode: WrapMode) {
        self.vertical_wrap_mode = wrap_mode;
        self.horizontal_wrap_mode = wrap_mode;
    }

    pub fn vertical_wrap_mode(&self) -> WrapMode {
        self.vertical_wrap_mode
    }

    pub fn set_vertical_wrap_mode(&mut self, wrap_mode: WrapMode) {
        self.vertical_wrap_mode = wrap_mode;
    }

    pub fn horizontal_wrap_mode(&self) -> WrapMode {
        self.horizontal_wrap_mode
    }

    pub fn set_horizontal_wrap_mode(&mut self, wrap_mode: WrapMode) {
        self.horizontal_wrap_mode = wrap_mode;
    }

    pub fn vertical_policy(&self) -> VerticalPolicy {
        self.vertical_policy
    }

    pub fn set_vertical_policy(&mut self, vertical_policy: VerticalPolicy) {
        self.vertical_policy = vertical_policy;
    }

    pub fn horizontal_policy(&self) -> HorizontalPolicy {
        self.horizontal_policy
    }

    pub fn set_horizontal_policy(&mut self, horizontal_policy: HorizontalPolicy) {
        self.horizontal_policy = horizontal_policy;
    }

    /// Column remembered for `VerticalPolicy::StickyColumn` up/down.
    pub fn preferred_column(&self) -> usize {
        self.preferred_column
    }

    /// Flat 0-based index of the current cell (row-major over present cells).
    pub fn index(&self) -> usize {
        let prior = self.row_columns[..self.line]
            .iter()
            .map(Vec::len)
            .sum::<usize>();
        let position = self.column_position();
        prior.saturating_add(position)
    }

    /// Total number of present cells.
    pub fn cell_count(&self) -> usize {
        self.row_columns.iter().map(Vec::len).sum()
    }

    /// Move to the cell at the given flat index (row-major over present cells).
    pub fn set_index(&mut self, index: usize) -> Result<()> {
        let total = self.cell_count();
        if index >= total {
            return Err(Error::IndexOutOfBounds {
                index,
                cell_count: total,
            });
        }
        let mut remaining = index;
        for (line, columns) in self.row_columns.iter().enumerate() {
            if remaining < columns.len() {
                self.line = line;
                self.column = columns[remaining];
                self.remember_column();
                return Ok(());
            }
            remaining -= columns.len();
        }
        unreachable!("index validated against cell_count");
    }

    /// Replace per-row column widths, clamping the cursor into the new bounds.
    pub fn set_column_sizes(&mut self, column_sizes: Vec<usize>) -> Result<()> {
        if column_sizes.is_empty() || column_sizes.contains(&0) {
            return Err(Error::InvalidColumnSizes);
        }
        self.row_columns = column_sizes
            .iter()
            .map(|&width| (0..width).collect())
            .collect();
        self.column_sizes = column_sizes;
        self.clamp_into_bounds();
        Ok(())
    }

    /// Replace the present columns of each row, clamping the cursor onto a present cell.
    ///
    /// Does not update the preferred column. See [`Self::new_sparse`] for the row shape.
    pub fn set_rows(&mut self, rows: Vec<Vec<usize>>) -> Result<()> {
        let (row_columns, column_sizes) = prepare_sparse(rows)?;
        self.row_columns = row_columns;
        self.column_sizes = column_sizes;
        self.clamp_into_bounds();
        Ok(())
    }

    pub fn set(&mut self, line: usize, column: usize) -> Result<()> {
        self.validate_line(line)?;
        self.validate_column_at(line, column)?;
        self.line = line;
        self.column = column;
        self.remember_column();
        Ok(())
    }

    pub fn set_line(&mut self, line: usize) -> Result<()> {
        self.validate_line(line)?;
        self.line = line;
        self.clamp_column();
        Ok(())
    }

    pub fn set_column(&mut self, column: usize) -> Result<()> {
        self.validate_column_at(self.line, column)?;
        self.column = column;
        self.remember_column();
        Ok(())
    }

    /// Move to the first cell of the first row.
    pub fn home(&mut self) {
        self.line = 0;
        self.column = self.row_columns[0][0];
        self.remember_column();
    }

    /// Move to the last cell of the last row.
    pub fn end(&mut self) {
        self.line = self.line_count() - 1;
        self.row_end();
    }

    /// Move to the first present cell of the current row.
    pub fn row_start(&mut self) {
        self.column = self.row_columns[self.line][0];
        self.remember_column();
    }

    /// Move to the last present cell of the current row.
    pub fn row_end(&mut self) {
        self.column = *self.row_columns[self.line].last().unwrap();
        self.remember_column();
    }

    /// Move up one row. Returns `true` when the cell changes.
    pub fn up(&mut self) -> bool {
        let before = self.current();
        match self.vertical_policy {
            VerticalPolicy::ClampColumn => self.up_clamp_column(),
            VerticalPolicy::SkipMissing => self.up_skip_missing(),
            VerticalPolicy::StickyColumn => self.up_sticky_column(),
        }
        self.current() != before
    }

    /// Move down one row. Returns `true` when the cell changes.
    pub fn down(&mut self) -> bool {
        let before = self.current();
        match self.vertical_policy {
            VerticalPolicy::ClampColumn => self.down_clamp_column(),
            VerticalPolicy::SkipMissing => self.down_skip_missing(),
            VerticalPolicy::StickyColumn => self.down_sticky_column(),
        }
        self.current() != before
    }

    /// Move left one cell. Returns `true` when the cell changes.
    pub fn left(&mut self) -> bool {
        let before = self.current();
        match self.horizontal_policy {
            HorizontalPolicy::WithinRow => self.left_within_row(),
            HorizontalPolicy::AcrossRows => self.left_across_rows(),
        }
        self.remember_column();
        self.current() != before
    }

    /// Move right one cell. Returns `true` when the cell changes.
    pub fn right(&mut self) -> bool {
        let before = self.current();
        match self.horizontal_policy {
            HorizontalPolicy::WithinRow => self.right_within_row(),
            HorizontalPolicy::AcrossRows => self.right_across_rows(),
        }
        self.remember_column();
        self.current() != before
    }

    /// Move down (`delta > 0`) or up (`delta < 0`) by up to `delta` steps.
    ///
    /// Each step uses the current vertical policy. Movement stops at the first
    /// or last reachable row, including when [`WrapMode`] is [`WrapMode::Wrap`].
    pub fn shift_line(&mut self, delta: isize) {
        if delta > 0 {
            self.repeat_toward_edge(delta.unsigned_abs(), Self::down, WrapAxis::Vertical);
        } else {
            self.repeat_toward_edge(delta.unsigned_abs(), Self::up, WrapAxis::Vertical);
        }
    }

    /// Move right (`delta > 0`) or left (`delta < 0`) by up to `delta` steps.
    ///
    /// Each step uses the current horizontal policy. Movement stops at the edge
    /// of the row, or of the table when crossing rows, including when
    /// [`WrapMode`] is [`WrapMode::Wrap`].
    pub fn shift_column(&mut self, delta: isize) {
        if delta > 0 {
            self.repeat_toward_edge(delta.unsigned_abs(), Self::right, WrapAxis::Horizontal);
        } else {
            self.repeat_toward_edge(delta.unsigned_abs(), Self::left, WrapAxis::Horizontal);
        }
    }

    /// Repeat `step` up to `steps` times with wrapping disabled on `axis`, stopping when a step does not move.
    fn repeat_toward_edge(&mut self, steps: usize, step: fn(&mut Self) -> bool, axis: WrapAxis) {
        if steps == 0 {
            return;
        }
        let previous = match axis {
            WrapAxis::Vertical => self.vertical_wrap_mode,
            WrapAxis::Horizontal => self.horizontal_wrap_mode,
        };
        match axis {
            WrapAxis::Vertical => self.vertical_wrap_mode = WrapMode::Clamp,
            WrapAxis::Horizontal => self.horizontal_wrap_mode = WrapMode::Clamp,
        }
        for _ in 0..steps {
            if !step(self) {
                break;
            }
        }
        match axis {
            WrapAxis::Vertical => self.vertical_wrap_mode = previous,
            WrapAxis::Horizontal => self.horizontal_wrap_mode = previous,
        }
    }

    fn left_within_row(&mut self) {
        let position = self.column_position();
        if position > 0 {
            self.column = self.row_columns[self.line][position - 1];
        } else if self.horizontal_wrap_mode == WrapMode::Wrap {
            self.column = *self.row_columns[self.line].last().unwrap();
        }
    }

    fn right_within_row(&mut self) {
        let position = self.column_position();
        let len = self.row_columns[self.line].len();
        if position + 1 < len {
            self.column = self.row_columns[self.line][position + 1];
        } else if self.horizontal_wrap_mode == WrapMode::Wrap {
            self.column = self.row_columns[self.line][0];
        }
    }

    fn left_across_rows(&mut self) {
        let position = self.column_position();
        if position > 0 {
            self.column = self.row_columns[self.line][position - 1];
            return;
        }
        match self.horizontal_wrap_mode {
            WrapMode::Wrap => {
                if self.line == 0 {
                    self.line = self.line_count() - 1;
                } else {
                    self.line -= 1;
                }
                self.row_end();
            }
            WrapMode::Clamp => {
                if self.line > 0 {
                    self.line -= 1;
                    self.row_end();
                }
            }
        }
    }

    fn right_across_rows(&mut self) {
        let position = self.column_position();
        if position + 1 < self.row_columns[self.line].len() {
            self.column = self.row_columns[self.line][position + 1];
            return;
        }
        match self.horizontal_wrap_mode {
            WrapMode::Wrap => {
                if self.line + 1 == self.line_count() {
                    self.line = 0;
                } else {
                    self.line += 1;
                }
                self.column = self.row_columns[self.line][0];
            }
            WrapMode::Clamp => {
                if self.line + 1 < self.line_count() {
                    self.line += 1;
                    self.column = self.row_columns[self.line][0];
                }
            }
        }
    }

    fn up_clamp_column(&mut self) {
        self.move_line(Direction::Up);
        self.clamp_column();
    }

    fn down_clamp_column(&mut self) {
        self.move_line(Direction::Down);
        self.clamp_column();
    }

    fn up_sticky_column(&mut self) {
        self.move_line(Direction::Up);
        self.place_sticky_column();
    }

    fn down_sticky_column(&mut self) {
        self.move_line(Direction::Down);
        self.place_sticky_column();
    }

    /// Move to the adjacent row, wrapping or stopping according to the vertical wrap mode.
    fn move_line(&mut self, direction: Direction) {
        let n = self.line_count();
        match (self.vertical_wrap_mode, direction) {
            (WrapMode::Wrap, Direction::Up) => {
                if self.line == 0 {
                    self.line = n - 1;
                } else {
                    self.line -= 1;
                }
            }
            (WrapMode::Wrap, Direction::Down) => {
                if self.line + 1 == n {
                    self.line = 0;
                } else {
                    self.line += 1;
                }
            }
            (WrapMode::Clamp, Direction::Up) => {
                if self.line > 0 {
                    self.line -= 1;
                }
            }
            (WrapMode::Clamp, Direction::Down) => {
                if self.line + 1 < n {
                    self.line += 1;
                }
            }
        }
    }

    /// Land on the preferred column, or the row's last cell when that column is absent.
    fn place_sticky_column(&mut self) {
        if self.row_columns[self.line]
            .binary_search(&self.preferred_column)
            .is_ok()
        {
            self.column = self.preferred_column;
        } else {
            self.column = *self.row_columns[self.line].last().unwrap();
        }
    }

    fn remember_column(&mut self) {
        self.preferred_column = self.column;
    }

    fn up_skip_missing(&mut self) {
        if let Some(target) = self.find_line_skip_missing(Direction::Up) {
            self.line = target;
        }
    }

    fn down_skip_missing(&mut self) {
        if let Some(target) = self.find_line_skip_missing(Direction::Down) {
            self.line = target;
        }
    }

    fn find_line_skip_missing(&self, direction: Direction) -> Option<usize> {
        let n = self.line_count();
        match (self.vertical_wrap_mode, direction) {
            (WrapMode::Wrap, Direction::Up) => {
                for i in 1..n {
                    let target = (self.line + n - i) % n;
                    if self.has_cell(target, self.column) {
                        return Some(target);
                    }
                }
                None
            }
            (WrapMode::Wrap, Direction::Down) => {
                for i in 1..n {
                    let target = (self.line + i) % n;
                    if self.has_cell(target, self.column) {
                        return Some(target);
                    }
                }
                None
            }
            (WrapMode::Clamp, Direction::Up) => {
                if self.line == 0 {
                    return None;
                }
                for target in (0..self.line).rev() {
                    if self.has_cell(target, self.column) {
                        return Some(target);
                    }
                }
                None
            }
            (WrapMode::Clamp, Direction::Down) => {
                for target in (self.line + 1)..n {
                    if self.has_cell(target, self.column) {
                        return Some(target);
                    }
                }
                None
            }
        }
    }

    fn column_position(&self) -> usize {
        self.row_columns[self.line]
            .binary_search(&self.column)
            .expect("cursor is on a present cell")
    }

    fn clamp_into_bounds(&mut self) {
        if self.line >= self.line_count() {
            self.line = self.line_count() - 1;
        }
        self.clamp_column();
    }

    /// Keep the column when it exists. Otherwise the greatest present column at
    /// or before it, or the first present column when every cell is to the right.
    fn clamp_column(&mut self) {
        self.column = clamp_to_present(&self.row_columns[self.line], self.column);
    }

    pub(crate) fn validate_cell(&self, line: usize, column: usize) -> Result<()> {
        self.validate_line(line)?;
        self.validate_column_at(line, column)
    }

    fn validate_line(&self, line: usize) -> Result<()> {
        let line_count = self.line_count();
        if line >= line_count {
            return Err(Error::LineOutOfBounds { line, line_count });
        }
        Ok(())
    }

    fn validate_column_at(&self, line: usize, column: usize) -> Result<()> {
        let column_size = self.column_sizes[line];
        if column >= column_size {
            return Err(Error::ColumnOutOfBounds {
                column,
                column_size,
            });
        }
        if self.row_columns[line].binary_search(&column).is_err() {
            return Err(Error::MissingCell { line, column });
        }
        Ok(())
    }
}

fn prepare_sparse(mut rows: Vec<Vec<usize>>) -> Result<(Vec<Vec<usize>>, Vec<usize>)> {
    if rows.is_empty() {
        return Err(Error::InvalidColumnSizes);
    }
    let mut column_sizes = Vec::with_capacity(rows.len());
    for row in &mut rows {
        if row.is_empty() {
            return Err(Error::InvalidColumnSizes);
        }
        row.sort_unstable();
        if row.windows(2).any(|pair| pair[0] == pair[1]) {
            return Err(Error::InvalidColumnSizes);
        }
        let span = row
            .last()
            .copied()
            .unwrap()
            .checked_add(1)
            .ok_or(Error::InvalidColumnSizes)?;
        column_sizes.push(span);
    }
    Ok((rows, column_sizes))
}

/// Greatest present column at or before `column`, or the first present column.
fn clamp_to_present(columns: &[usize], column: usize) -> usize {
    match columns.binary_search(&column) {
        Ok(_) => column,
        Err(0) => columns[0],
        Err(index) => columns[index - 1],
    }
}

enum Direction {
    Up,
    Down,
}

enum WrapAxis {
    Vertical,
    Horizontal,
}
