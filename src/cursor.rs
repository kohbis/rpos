use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WrapMode {
    Wrap,
    Clamp,
}

/// How vertical movement handles jagged rows that lack the current column.
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum VerticalPolicy {
    /// Move to the adjacent row and clamp the column to that row's width.
    /// This is the default and preserves pre-0.5 behavior.
    ClampColumn,
    /// Keep the current column and skip rows that do not have it.
    /// Useful when column identity should be preserved across jagged rows.
    SkipMissing,
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
    column_sizes: Vec<usize>,
    wrap_mode: WrapMode,
    vertical_policy: VerticalPolicy,
    horizontal_policy: HorizontalPolicy,
}

impl Cursor {
    pub fn new((line_size, column_size): (usize, usize)) -> Self {
        Self {
            line: 0,
            column: 0,
            column_sizes: vec![column_size; line_size],
            wrap_mode: WrapMode::Wrap,
            vertical_policy: VerticalPolicy::ClampColumn,
            horizontal_policy: HorizontalPolicy::WithinRow,
        }
    }

    pub fn new_jagged(column_sizes: Vec<usize>) -> Self {
        Self {
            line: 0,
            column: 0,
            column_sizes,
            wrap_mode: WrapMode::Wrap,
            vertical_policy: VerticalPolicy::ClampColumn,
            horizontal_policy: HorizontalPolicy::WithinRow,
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

    pub fn column_sizes(&self) -> &[usize] {
        &self.column_sizes
    }

    pub fn wrap_mode(&self) -> WrapMode {
        self.wrap_mode
    }

    pub fn set_wrap_mode(&mut self, wrap_mode: WrapMode) {
        self.wrap_mode = wrap_mode;
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

    /// Flat 0-based index of the current cell (row-major).
    pub fn index(&self) -> usize {
        self.column_sizes[..self.line]
            .iter()
            .sum::<usize>()
            .saturating_add(self.column)
    }

    /// Total number of addressable cells.
    pub fn cell_count(&self) -> usize {
        self.column_sizes.iter().sum()
    }

    /// Move to the cell at the given flat index (row-major).
    pub fn set_index(&mut self, index: usize) -> Result<()> {
        let total = self.cell_count();
        if index >= total {
            anyhow::bail!("index must be less than cell_count: {total} (0-indexed)");
        }
        let mut remaining = index;
        for (line, &size) in self.column_sizes.iter().enumerate() {
            if remaining < size {
                self.line = line;
                self.column = remaining;
                return Ok(());
            }
            remaining -= size;
        }
        unreachable!("index validated against cell_count");
    }

    /// Replace per-row column widths, clamping the cursor into the new bounds.
    pub fn set_column_sizes(&mut self, column_sizes: Vec<usize>) -> Result<()> {
        if column_sizes.is_empty() || column_sizes.contains(&0) {
            anyhow::bail!("invalid column sizes");
        }
        self.column_sizes = column_sizes;
        if self.line >= self.line_count() {
            self.line = self.line_count() - 1;
        }
        self.clamp_column();
        Ok(())
    }

    pub fn set(&mut self, line: usize, column: usize) -> Result<()> {
        self.validate_line(line).context("invalid cursor")?;
        self.validate_column_at(line, column)
            .context("invalid cursor")?;
        self.line = line;
        self.column = column;
        Ok(())
    }

    pub fn set_line(&mut self, line: usize) -> Result<()> {
        self.validate_line(line).context("invalid cursor")?;
        self.line = line;
        self.clamp_column();
        Ok(())
    }

    pub fn set_column(&mut self, column: usize) -> Result<()> {
        self.validate_column_at(self.line, column)
            .context("invalid cursor")?;
        self.column = column;
        Ok(())
    }

    /// Move to the first cell `(0, 0)`.
    pub fn home(&mut self) {
        if self.line_count() == 0 {
            return;
        }
        self.line = 0;
        self.column = 0;
    }

    /// Move to the last cell of the last row.
    pub fn end(&mut self) {
        if self.line_count() == 0 {
            return;
        }
        self.line = self.line_count() - 1;
        self.row_end();
    }

    /// Move to column 0 of the current row.
    pub fn row_start(&mut self) {
        self.column = 0;
    }

    /// Move to the last column of the current row.
    pub fn row_end(&mut self) {
        let col_size = self.current_column_size();
        if col_size > 0 {
            self.column = col_size - 1;
        }
    }

    pub fn up(&mut self) {
        match self.vertical_policy {
            VerticalPolicy::ClampColumn => self.up_clamp_column(),
            VerticalPolicy::SkipMissing => self.up_skip_missing(),
        }
    }

    pub fn down(&mut self) {
        match self.vertical_policy {
            VerticalPolicy::ClampColumn => self.down_clamp_column(),
            VerticalPolicy::SkipMissing => self.down_skip_missing(),
        }
    }

    pub fn left(&mut self) {
        match self.horizontal_policy {
            HorizontalPolicy::WithinRow => self.left_within_row(),
            HorizontalPolicy::AcrossRows => self.left_across_rows(),
        }
    }

    pub fn right(&mut self) {
        match self.horizontal_policy {
            HorizontalPolicy::WithinRow => self.right_within_row(),
            HorizontalPolicy::AcrossRows => self.right_across_rows(),
        }
    }

    fn left_within_row(&mut self) {
        let col_size = self.current_column_size();
        if col_size == 0 {
            return;
        }
        match self.wrap_mode {
            WrapMode::Wrap => {
                if self.column == 0 {
                    self.column = col_size - 1;
                } else {
                    self.column -= 1;
                }
            }
            WrapMode::Clamp => {
                if 0 < self.column {
                    self.column -= 1;
                }
            }
        }
    }

    fn right_within_row(&mut self) {
        let col_size = self.current_column_size();
        if col_size == 0 {
            return;
        }
        match self.wrap_mode {
            WrapMode::Wrap => {
                if self.column + 1 == col_size {
                    self.column = 0;
                } else {
                    self.column += 1;
                }
            }
            WrapMode::Clamp => {
                if self.column < col_size - 1 {
                    self.column += 1;
                }
            }
        }
    }

    fn left_across_rows(&mut self) {
        if self.line_count() == 0 || self.current_column_size() == 0 {
            return;
        }
        if self.column > 0 {
            self.column -= 1;
            return;
        }
        match self.wrap_mode {
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
        if self.line_count() == 0 || self.current_column_size() == 0 {
            return;
        }
        let col_size = self.current_column_size();
        if self.column + 1 < col_size {
            self.column += 1;
            return;
        }
        match self.wrap_mode {
            WrapMode::Wrap => {
                if self.line + 1 == self.line_count() {
                    self.line = 0;
                } else {
                    self.line += 1;
                }
                self.column = 0;
            }
            WrapMode::Clamp => {
                if self.line + 1 < self.line_count() {
                    self.line += 1;
                    self.column = 0;
                }
            }
        }
    }

    fn up_clamp_column(&mut self) {
        if self.line_count() == 0 {
            return;
        }
        match self.wrap_mode {
            WrapMode::Wrap => {
                if self.line == 0 {
                    self.line = self.line_count() - 1;
                } else {
                    self.line -= 1;
                }
            }
            WrapMode::Clamp => {
                if 0 < self.line {
                    self.line -= 1;
                }
            }
        }
        self.clamp_column();
    }

    fn down_clamp_column(&mut self) {
        if self.line_count() == 0 {
            return;
        }
        match self.wrap_mode {
            WrapMode::Wrap => {
                if self.line + 1 == self.line_count() {
                    self.line = 0;
                } else {
                    self.line += 1;
                }
            }
            WrapMode::Clamp => {
                if self.line < self.line_count() - 1 {
                    self.line += 1;
                }
            }
        }
        self.clamp_column();
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
        if n == 0 {
            return None;
        }

        match (self.wrap_mode, direction) {
            (WrapMode::Wrap, Direction::Up) => {
                for i in 1..n {
                    let target = (self.line + n - i) % n;
                    if self.column_sizes[target] > self.column {
                        return Some(target);
                    }
                }
                None
            }
            (WrapMode::Wrap, Direction::Down) => {
                for i in 1..n {
                    let target = (self.line + i) % n;
                    if self.column_sizes[target] > self.column {
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
                    if self.column_sizes[target] > self.column {
                        return Some(target);
                    }
                }
                None
            }
            (WrapMode::Clamp, Direction::Down) => {
                for target in (self.line + 1)..n {
                    if self.column_sizes[target] > self.column {
                        return Some(target);
                    }
                }
                None
            }
        }
    }

    fn current_column_size(&self) -> usize {
        self.column_sizes[self.line]
    }

    fn clamp_column(&mut self) {
        let col_size = self.current_column_size();
        if col_size > 0 && self.column >= col_size {
            self.column = col_size - 1;
        }
    }

    fn validate_line(&self, line: usize) -> Result<()> {
        if line >= self.line_count() {
            anyhow::bail!(
                "line must be less than line_size: {} (0-indexed)",
                self.line_count(),
            )
        }
        Ok(())
    }

    fn validate_column_at(&self, line: usize, column: usize) -> Result<()> {
        let col_size = self.column_sizes[line];
        if column >= col_size {
            anyhow::bail!("column must be less than column_size: {col_size} (0-indexed)",)
        }
        Ok(())
    }
}

enum Direction {
    Up,
    Down,
}
