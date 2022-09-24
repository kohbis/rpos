use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Table {
    height: u32,
    width: u32,
    cursor: Cursor,
}

impl Table {
    pub fn new(height: u32, width: u32) -> Self {
        Self {
            height,
            width,
            cursor: Cursor::new(),
        }
    }

    pub fn up(&mut self) {
        if 0 < self.cursor.line {
            self.cursor.up();
        }
    }

    pub fn down(&mut self) {
        if self.cursor.line < self.height - 1 {
            self.cursor.down();
        }
    }

    pub fn left(&mut self) {
        if 0 < self.cursor.column {
            self.cursor.left();
        }
    }

    pub fn right(&mut self) {
        if self.cursor.column < self.width - 1 {
            self.cursor.right();
        }
    }
}
