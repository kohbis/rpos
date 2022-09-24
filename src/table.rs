use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Table {
    pub height: u32,
    pub width: u32,
    pub cursor: Cursor,
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_table() {
        let table = Table::new(3, 4);
        assert_eq!(table.height, 3);
        assert_eq!(table.width, 4);
        assert_eq!(table.cursor, Cursor { line: 0, column: 0 });
    }
}
