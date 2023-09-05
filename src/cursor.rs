#[derive(Debug, PartialEq)]
pub struct Cursor {
    pub line: i32,
    pub column: i32,
}

impl Cursor {
    pub fn new() -> Self {
        Self { line: 0, column: 0 }
    }

    pub fn up(&mut self) {
        self.line -= 1
    }

    pub fn down(&mut self) {
        self.line += 1
    }

    pub fn left(&mut self) {
        self.column -= 1
    }

    pub fn right(&mut self) {
        self.column += 1
    }
}
