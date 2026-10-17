use crate::cursor::{Cursor, HorizontalPolicy, VerticalPolicy, WrapMode};
use crate::{Error, Result};

#[derive(Debug)]
pub struct Table {
    pub cursor: Cursor,
}

impl Table {
    pub fn new(height: usize, width: usize) -> Result<Self> {
        if height == 0 || width == 0 {
            return Err(Error::InvalidTableSize);
        }
        Ok(Self {
            cursor: Cursor::new((height, width))?,
        })
    }

    pub fn new_jagged(widths: Vec<usize>) -> Result<Self> {
        if widths.is_empty() || widths.contains(&0) {
            return Err(Error::InvalidTableSize);
        }
        Ok(Self {
            cursor: Cursor::new_jagged(widths)?,
        })
    }

    /// Each inner vec is the present column indexes on that row.
    ///
    /// See [`Cursor::new_sparse`](crate::Cursor::new_sparse).
    pub fn new_sparse(rows: Vec<Vec<usize>>) -> Result<Self> {
        let cursor = Cursor::new_sparse(rows).map_err(|_| Error::InvalidTableSize)?;
        Ok(Self { cursor })
    }

    /// Number of rows (derived from the cursor's column sizes).
    pub fn height(&self) -> usize {
        self.cursor.line_count()
    }

    /// Maximum row width (derived from the cursor's column sizes).
    pub fn width(&self) -> usize {
        self.cursor
            .column_sizes()
            .iter()
            .copied()
            .max()
            .unwrap_or(0)
    }

    pub fn wrap_mode(mut self, wrap_mode: WrapMode) -> Self {
        self.cursor.set_wrap_mode(wrap_mode);
        self
    }

    pub fn vertical_wrap_mode(mut self, wrap_mode: WrapMode) -> Self {
        self.cursor.set_vertical_wrap_mode(wrap_mode);
        self
    }

    pub fn horizontal_wrap_mode(mut self, wrap_mode: WrapMode) -> Self {
        self.cursor.set_horizontal_wrap_mode(wrap_mode);
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
            return Err(Error::InvalidTableSize);
        }
        self.cursor.set_column_sizes(vec![width; height])
    }

    /// Replace per-row widths (jagged), clamping the cursor into the new bounds.
    pub fn set_widths(&mut self, widths: Vec<usize>) -> Result<()> {
        if widths.is_empty() || widths.contains(&0) {
            return Err(Error::InvalidTableSize);
        }
        self.cursor.set_column_sizes(widths)
    }

    /// Replace the present columns of each row, clamping the cursor onto a present cell.
    pub fn set_rows(&mut self, rows: Vec<Vec<usize>>) -> Result<()> {
        self.cursor
            .set_rows(rows)
            .map_err(|_| Error::InvalidTableSize)
    }
}
