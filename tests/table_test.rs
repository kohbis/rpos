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
        assert!(Table::new(-1, 4)
            .unwrap_err()
            .to_string()
            .contains("invalid table"));
    }
}
