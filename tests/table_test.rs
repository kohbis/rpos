#[cfg(test)]
mod tests {
    use rpos::table::Table;

    #[test]
    fn initialize_table() {
        let table = Table::new(3, 4);
        assert_eq!(table.height, 3);
        assert_eq!(table.width, 4);
        assert_eq!((table.cursor.line, table.cursor.column), (0, 0));
    }
}
