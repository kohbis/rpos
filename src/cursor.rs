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
        self.validate(Some(line), Some(column))
            .context("invalid cursor")?;
        self.line = line;
        self.column = column;
        Ok(())
    }

    pub fn set_line(&mut self, line: usize) -> Result<()> {
        self.validate(Some(line), None).context("invalid cursor")?;
        self.line = line;
        Ok(())
    }

    pub fn set_column(&mut self, column: usize) -> Result<()> {
        self.validate(None, Some(column))
            .context("invalid cursor")?;
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

    fn validate(&self, line: Option<usize>, column: Option<usize>) -> Result<()> {
        match (line, column) {
            (Some(line), Some(column)) => {
                if line >= self.line_size || column >= self.column_size {
                    anyhow::bail!(
                        "line and column must be less than line_size: {}, column_size: {} (0-indexed)",
                        self.line_size,
                        self.column_size
                    )
                }
            }
            (Some(line), None) => {
                if line >= self.line_size {
                    anyhow::bail!(
                        "line must be less than line_size: {} (0-indexed)",
                        self.line_size,
                    )
                }
            }
            (None, Some(column)) => {
                if column >= self.column_size {
                    anyhow::bail!(
                        "column must be less than column_size: {} (0-indexed)",
                        self.column_size,
                    )
                }
            }
            (None, None) => {}
        }
        Ok(())
    }
}
