use anyhow::Result;

use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Table {
    pub height: i32,
    pub width: i32,
    pub cursor: Cursor,
}

impl Table {
    pub fn new(height: i32, width: i32) -> Result<Self> {
        let table = Self {
            height,
            width,
            cursor: Cursor::new((height, width)),
        };
        if let Err(e) = table.validate() {
            anyhow::bail!("invalid table: {}", e)
        }
        Ok(table)
    }

    fn validate(&self) -> Result<()> {
        if self.height.is_positive() && self.width.is_positive() {
            return Ok(());
        }
        anyhow::bail!("height and width must be positive")
    }
}
