#[cfg(test)]
mod tests {
    use rpos::table::Table;
    use rpos::{Cursor, Error};

    fn assert_std_error<E: std::error::Error>(error: &E) {
        let _ = error.to_string();
    }

    #[test]
    fn invalid_sizes_are_distinct() {
        assert_eq!(Cursor::new((0, 2)).unwrap_err(), Error::InvalidColumnSizes);
        assert_eq!(
            Cursor::new_jagged(vec![2, 0]).unwrap_err(),
            Error::InvalidColumnSizes
        );
        assert_eq!(Table::new(0, 2).unwrap_err(), Error::InvalidTableSize);
        assert_eq!(
            Table::new_jagged(vec![1, 0]).unwrap_err(),
            Error::InvalidTableSize
        );
    }

    #[test]
    fn set_reports_line_and_column_bounds() {
        let mut cursor = Cursor::new((2, 3)).unwrap();

        assert_eq!(
            cursor.set(2, 0).unwrap_err(),
            Error::LineOutOfBounds {
                line: 2,
                line_count: 2
            }
        );
        assert_eq!(
            cursor.set(1, 3).unwrap_err(),
            Error::ColumnOutOfBounds {
                column: 3,
                column_size: 3
            }
        );
        assert_eq!(cursor.current(), (0, 0));
    }

    #[test]
    fn set_index_reports_the_cell_count() {
        let mut cursor = Cursor::new((2, 2)).unwrap();

        assert_eq!(
            cursor.set_index(4).unwrap_err(),
            Error::IndexOutOfBounds {
                index: 4,
                cell_count: 4
            }
        );
    }

    #[test]
    fn display_keeps_the_bounds_wording() {
        let error = Error::LineOutOfBounds {
            line: 4,
            line_count: 3,
        };
        assert_eq!(
            error.to_string(),
            "line must be less than line_size: 3 (0-indexed)"
        );
        assert_std_error(&error);
        assert_std_error(&Error::InvalidColumnSizes);
        assert_std_error(&Error::InvalidTableSize);
    }
}
