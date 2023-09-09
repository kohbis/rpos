#[cfg(test)]
mod tests {
    use rpos::table::Table;

    #[test]
    fn initialize_table() {
        let table = Table::new(3, 4);
        assert_eq!(table.height, 3);
        assert_eq!(table.width, 4);
        assert_eq!(table.cursor.current(), (0, 0));
    }
}
