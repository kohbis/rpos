use anyhow::Result;

#[derive(Debug, PartialEq)]
pub struct Cursor {
    line: i32,
    column: i32,
    line_size: i32,
    column_size: i32,
}

impl Cursor {
    pub fn new((line_size, column_size): (i32, i32)) -> Self {
        Self {
            line: 0,
            column: 0,
            line_size,
            column_size,
        }
    }

    pub fn current(&self) -> (i32, i32) {
        (self.line, self.column)
    }

    pub fn set(&mut self, (line, column): (i32, i32)) -> Result<()> {
        if let Err(e) = self.validate((line, column)) {
            anyhow::bail!("invalid cursor: {}", e)
        }
        self.line = line;
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

    fn validate(&self, (line, column): (i32, i32)) -> Result<()> {
        if line < 0 || column < 0 {
            anyhow::bail!("line and column must be positive")
        }
        if line >= self.line_size || column >= self.column_size {
            anyhow::bail!(
                "line and column must be less than line_size: {}, column_size: {}",
                self.line_size,
                self.column_size
            )
        }
        Ok(())
    }
}
