use anyhow::{Context, Result};

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum WrapMode {
    Wrap,
    Clamp,
}

#[derive(Debug, PartialEq)]
pub struct Cursor {
    line: usize,
    column: usize,
    column_sizes: Vec<usize>,
    wrap_mode: WrapMode,
}

impl Cursor {
    pub fn new((line_size, column_size): (usize, usize)) -> Self {
        Self {
            line: 0,
            column: 0,
            column_sizes: vec![column_size; line_size],
            wrap_mode: WrapMode::Wrap,
        }
    }

    pub fn new_jagged(column_sizes: Vec<usize>) -> Self {
        Self {
            line: 0,
            column: 0,
            column_sizes,
            wrap_mode: WrapMode::Wrap,
        }
    }

    pub fn current(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    pub fn wrap_mode(&self) -> WrapMode {
        self.wrap_mode
    }

    pub fn set_wrap_mode(&mut self, wrap_mode: WrapMode) {
        self.wrap_mode = wrap_mode;
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

    pub fn up(&mut self) {
        if self.line_size() == 0 {
            return;
        }
        match self.wrap_mode {
            WrapMode::Wrap => {
                if self.line == 0 {
                    self.line = self.line_size() - 1;
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

    pub fn down(&mut self) {
        if self.line_size() == 0 {
            return;
        }
        match self.wrap_mode {
            WrapMode::Wrap => {
                if self.line + 1 == self.line_size() {
                    self.line = 0;
                } else {
                    self.line += 1;
                }
            }
            WrapMode::Clamp => {
                if self.line < self.line_size() - 1 {
                    self.line += 1;
                }
            }
        }
        self.clamp_column();
    }

    pub fn left(&mut self) {
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

    pub fn right(&mut self) {
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

    fn line_size(&self) -> usize {
        self.column_sizes.len()
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
        if line >= self.line_size() {
            anyhow::bail!(
                "line must be less than line_size: {} (0-indexed)",
                self.line_size(),
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
