use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Table {
    pub height: i32,
    pub width: i32,
    pub cursor: Cursor,
}

impl Table {
    pub fn new(height: i32, width: i32) -> Self {
        Self {
            height,
            width,
            cursor: Cursor::new((height, width)),
        }
    }
}
