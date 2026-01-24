use anyhow::{Context, Result};

#[derive(Debug, PartialEq)]
pub struct Cursor {
    line: usize,
    column: usize,
    line_size: usize,
    column_size: usize,
}

impl Cursor {
    pub fn new((line_size, column_size): (usize, usize)) -> Self {
        Self {
            line: 0,
            column: 0,
            line_size,
            column_size,
        }
    }

    pub fn current(&self) -> (usize, usize) {
        (self.line, self.column)
    }

    pub fn set(&mut self, line: usize, column: usize) -> Result<()> {
        self.validate_line(line).context("invalid cursor")?;
        self.validate_column(column).context("invalid cursor")?;
        self.line = line;
        self.column = column;
        Ok(())
    }

    pub fn set_line(&mut self, line: usize) -> Result<()> {
        self.validate_line(line).context("invalid cursor")?;
        self.line = line;
        Ok(())
    }

    pub fn set_column(&mut self, column: usize) -> Result<()> {
        self.validate_column(column).context("invalid cursor")?;
        self.column = column;
        Ok(())
    }

    pub fn up(&mut self) {
        if 0 < self.line {
            self.line -= 1
        }
    }

    pub fn down(&mut self) {
        if self.line < self.line_size - 1 {
            self.line += 1
        }
    }

    pub fn left(&mut self) {
        if 0 < self.column {
            self.column -= 1
        }
    }

    pub fn right(&mut self) {
        if self.column < self.column_size - 1 {
            self.column += 1
        }
    }

    fn validate_line(&self, line: usize) -> Result<()> {
        if line >= self.line_size {
            anyhow::bail!(
                "line must be less than line_size: {} (0-indexed)",
                self.line_size,
            )
        }
        Ok(())
    }

    fn validate_column(&self, column: usize) -> Result<()> {
        if column >= self.column_size {
            anyhow::bail!(
                "column must be less than column_size: {} (0-indexed)",
                self.column_size,
            )
        }
        Ok(())
    }
}
