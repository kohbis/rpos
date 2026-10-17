use std::fmt;

/// Failure from a cursor, table, or selection operation.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Error {
    /// A row list was empty, or a row width was zero.
    InvalidColumnSizes,
    /// A table height or width was zero, or a jagged width list was empty or contained zero.
    InvalidTableSize,
    /// `line` is outside `0..line_count`.
    LineOutOfBounds { line: usize, line_count: usize },
    /// `column` is outside `0..column_size` on that row.
    ColumnOutOfBounds { column: usize, column_size: usize },
    /// `index` is outside `0..cell_count`.
    IndexOutOfBounds { index: usize, cell_count: usize },
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::InvalidColumnSizes => write!(f, "invalid column sizes"),
            Error::InvalidTableSize => write!(f, "invalid table size"),
            Error::LineOutOfBounds { line_count, .. } => {
                write!(
                    f,
                    "line must be less than line_size: {line_count} (0-indexed)"
                )
            }
            Error::ColumnOutOfBounds { column_size, .. } => {
                write!(
                    f,
                    "column must be less than column_size: {column_size} (0-indexed)"
                )
            }
            Error::IndexOutOfBounds { cell_count, .. } => {
                write!(
                    f,
                    "index must be less than cell_count: {cell_count} (0-indexed)"
                )
            }
        }
    }
}

impl std::error::Error for Error {}

/// Result of a cursor, table, or selection operation.
pub type Result<T> = std::result::Result<T, Error>;
