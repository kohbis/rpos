#[cfg(test)]
mod tests {
    use rpos::table::Table;

    #[test]
    fn initialize_cursor() {
        let cursor = Table::new(3, 4).cursor;
        assert_eq!((cursor.line, cursor.column), (0, 0));
    }

    #[test]
    fn move_cursor() {
        let mut cursor = Table::new(3, 4).cursor;

        cursor.down();
        assert_eq!((cursor.line, cursor.column), (1, 0));

        cursor.right();
        assert_eq!((cursor.line, cursor.column), (1, 1));

        cursor.up();
        assert_eq!((cursor.line, cursor.column), (0, 1));

        cursor.left();
        assert_eq!((cursor.line, cursor.column), (0, 0));
    }
}
