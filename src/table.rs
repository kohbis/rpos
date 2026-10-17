use anyhow::Result;

use crate::cursor::{Cursor, HorizontalPolicy, VerticalPolicy, WrapMode};

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
        let width = widths.iter().copied().max().unwrap_or(0);
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

    pub fn vertical_policy(mut self, vertical_policy: VerticalPolicy) -> Self {
        self.cursor.set_vertical_policy(vertical_policy);
        self
    }

    pub fn horizontal_policy(mut self, horizontal_policy: HorizontalPolicy) -> Self {
        self.cursor.set_horizontal_policy(horizontal_policy);
        self
    }

    /// Resize to a rectangular table, clamping the cursor into the new bounds.
    pub fn set_size(&mut self, height: usize, width: usize) -> Result<()> {
        if height == 0 || width == 0 {
            anyhow::bail!("invalid table size");
        }
        self.cursor.set_column_sizes(vec![width; height])?;
        self.height = height;
        self.width = width;
        Ok(())
    }

    /// Replace per-row widths (jagged), clamping the cursor into the new bounds.
    pub fn set_widths(&mut self, widths: Vec<usize>) -> Result<()> {
        if widths.is_empty() || widths.contains(&0) {
            anyhow::bail!("invalid table size");
        }
        let height = widths.len();
        let width = widths.iter().copied().max().unwrap_or(0);
        self.cursor.set_column_sizes(widths)?;
        self.height = height;
        self.width = width;
        Ok(())
    }
}
