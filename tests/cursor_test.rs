#[cfg(test)]
mod tests {
    use rpos::table::Table;

    #[test]
    fn initialize_cursor() {
        let cursor = Table::new(3, 4).cursor;
        assert_eq!(cursor.current(), (0, 0));
    }

    #[test]
    fn move_cursor() {
        let mut cursor = Table::new(3, 4).cursor;

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
        let mut cursor = Table::new(3, 4).cursor;

        cursor.set((2, 3)).unwrap();
        assert_eq!(cursor.current(), (2, 3));
    }

    #[test]
    #[should_panic]
    fn set_cursor_out_of_range() {
        let mut cursor = Table::new(3, 4).cursor;

        cursor.set((3, 4)).unwrap();
    }
}
