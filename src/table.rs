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

    pub fn new_with_wrap_mode(height: usize, width: usize, wrap_mode: WrapMode) -> Result<Self> {
        if height == 0 || width == 0 {
            anyhow::bail!("invalid table size");
        }
        Ok(Self {
            height,
            width,
            cursor: Cursor::new_with_mode((height, width), wrap_mode),
        })
    }
}
