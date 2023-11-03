#[cfg(test)]
mod tests {
    use rpos::table::Table;

    #[test]
    fn initialize_cursor() {
        let cursor = Table::new(3, 4).unwrap().cursor;
        assert_eq!(cursor.current(), (0, 0));
    }

    #[test]
    fn move_cursor() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        cursor.down();
        assert_eq!(cursor.current(), (1, 0));

        cursor.right();
        assert_eq!(cursor.current(), (1, 1));

        cursor.up();
        assert_eq!(cursor.current(), (0, 1));

        cursor.left();
        assert_eq!(cursor.current(), (0, 0));
    }

    #[test]
    fn set_cursor() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        cursor.set(2, 3).unwrap();
        assert_eq!(cursor.current(), (2, 3));
    }

    #[test]
    fn set_cursor_out_of_range() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        assert!(cursor
            .set(3, 4)
            .unwrap_err()
            .to_string()
            .contains("invalid cursor"));
    }

    #[test]
    fn set_cursor_line() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        cursor.set_line(2).unwrap();
        assert_eq!(cursor.current(), (2, 0));
    }

    #[test]
    fn set_cursor_line_out_of_range() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        assert!(cursor
            .set_line(3)
            .unwrap_err()
            .to_string()
            .contains("invalid cursor"));
    }

    #[test]
    fn set_cursor_column() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        cursor.set_column(3).unwrap();
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn set_cursor_column_out_of_range() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        assert!(cursor
            .set_column(4)
            .unwrap_err()
            .to_string()
            .contains("invalid cursor"));
    }
}
