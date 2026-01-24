use anyhow::{Context, Result};

use crate::cursor::Cursor;

#[derive(Debug)]
pub struct Table {
    pub height: usize,
    pub width: usize,
    pub cursor: Cursor,
}

impl Table {
    pub fn new(height: usize, width: usize) -> Result<Self> {
        let table = Self {
            height,
            width,
            cursor: Cursor::new((height, width)),
        };
        table.validate().context("invalid table")?;
        Ok(table)
    }

    fn validate(&self) -> Result<()> {
        if self.height == 0 || self.width == 0 {
            anyhow::bail!("invalid table size")
        }
        Ok(())
    }
}
