#[cfg(test)]
mod tests {
    use rpos::table::Table;

    #[test]
    fn initialize_table() {
        let table = Table::new(3, 4).unwrap();
        assert_eq!(table.height, 3);
        assert_eq!(table.width, 4);
        assert_eq!(table.cursor.current(), (0, 0));
    }

    #[test]
    fn initialize_table_with_negative_height() {
        assert!(Table::new(0, 4)
            .unwrap_err()
            .to_string()
            .contains("invalid table"));
    }

    #[test]
    fn initialize_table_with_zero_width() {
        assert!(Table::new(3, 0)
            .unwrap_err()
            .to_string()
            .contains("invalid table"));
    }

    #[test]
    fn initialize_table_with_zero_dimensions() {
        assert!(Table::new(0, 0)
            .unwrap_err()
            .to_string()
            .contains("invalid table"));
    }

    #[test]
    fn initialize_minimum_table() {
        let table = Table::new(1, 1).unwrap();
        assert_eq!(table.height, 1);
        assert_eq!(table.width, 1);
        assert_eq!(table.cursor.current(), (0, 0));
    }

    #[test]
    fn initialize_single_row_table() {
        let table = Table::new(1, 5).unwrap();
        assert_eq!(table.height, 1);
        assert_eq!(table.width, 5);
        assert_eq!(table.cursor.current(), (0, 0));
    }

    #[test]
    fn initialize_single_column_table() {
        let table = Table::new(5, 1).unwrap();
        assert_eq!(table.height, 5);
        assert_eq!(table.width, 1);
        assert_eq!(table.cursor.current(), (0, 0));
    }
}
