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
        let err = Table::new(0, 4).unwrap_err();
        assert!(err.to_string().contains("invalid table size"));
        assert!(err.to_string().contains("height"));
    }

    #[test]
    fn initialize_table_with_negative_width() {
        let err = Table::new(4, 0).unwrap_err();
        assert!(err.to_string().contains("invalid table size"));
        assert!(err.to_string().contains("width"));
    }

    #[test]
    fn initialize_large_table() {
        let table = Table::new(10, 20).unwrap();
        assert_eq!(table.height, 10);
        assert_eq!(table.width, 20);
        assert_eq!(table.cursor.current(), (0, 0));
    }

    #[test]
    fn cursor_position_after_table_creation() {
        let table = Table::new(5, 5).unwrap();
        assert_eq!(table.cursor.current(), (0, 0), 
            "Cursor should start at (0,0) regardless of table size");
    }

    #[test]
    fn table_dimension_limits() {
        assert!(Table::new(usize::MAX, usize::MAX).is_ok(), 
            "Should handle maximum usize dimensions");
    }
}
