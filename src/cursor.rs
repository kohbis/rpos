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
}
