use anyhow::Result;

use crate::cursor::{Cursor, WrapMode};

#[derive(Debug)]
pub struct Table {
    pub height: usize,
    pub width: usize,
    pub cursor: Cursor,
}

impl Table {
    pub fn new(height: usize, width: usize) -> Result<Self> {
        if height == 0 || width == 0 {
            anyhow::bail!("invalid table size");
        }
        Ok(Self {
            height,
            width,
            cursor: Cursor::new((height, width)),
        })
    }

    pub fn new_jagged(widths: Vec<usize>) -> Result<Self> {
        if widths.is_empty() || widths.contains(&0) {
            anyhow::bail!("invalid table size");
        }
        let height = widths.len();
        let width = widths[0];
        Ok(Self {
            height,
            width,
            cursor: Cursor::new_jagged(widths),
        })
    }

    pub fn wrap_mode(mut self, wrap_mode: WrapMode) -> Self {
        self.cursor.set_wrap_mode(wrap_mode);
        self
    }
}
