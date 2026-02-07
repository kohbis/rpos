#[cfg(test)]
mod tests {
    use rpos::table::Table;
    use rpos::WrapMode;

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

    #[test]
    fn initialize_table_with_wrap_mode() {
        let mut table = Table::new(3, 4).unwrap().wrap_mode(WrapMode::Wrap);
        table.cursor.set(0, 0).unwrap();
        table.cursor.up();
        assert_eq!(table.cursor.current(), (2, 0));
    }

    // Jagged array tests
    #[test]
    fn initialize_jagged_table() {
        let table = Table::new_jagged(vec![4, 2, 3]).unwrap();
        assert_eq!(table.height, 3);
        assert_eq!(table.width, 4); // width is first row's width
        assert_eq!(table.cursor.current(), (0, 0));
    }

    #[test]
    fn initialize_jagged_table_with_empty_widths() {
        assert!(Table::new_jagged(vec![])
            .unwrap_err()
            .to_string()
            .contains("invalid table"));
    }

    #[test]
    fn initialize_jagged_table_with_zero_width() {
        assert!(Table::new_jagged(vec![3, 0, 2])
            .unwrap_err()
            .to_string()
            .contains("invalid table"));
    }

    #[test]
    fn jagged_table_down_clamps_column() {
        let mut table = Table::new_jagged(vec![4, 2, 3]).unwrap();
        table.cursor.set(0, 3).unwrap();

        table.cursor.down(); // row 1 has 2 cols
        assert_eq!(table.cursor.current(), (1, 1));
    }

    #[test]
    fn jagged_table_up_clamps_column() {
        let mut table = Table::new_jagged(vec![4, 2, 3]).unwrap();
        table.cursor.set(2, 2).unwrap();

        table.cursor.up(); // row 1 has 2 cols
        assert_eq!(table.cursor.current(), (1, 1));
    }

    #[test]
    fn jagged_table_set_line_clamps_column() {
        let mut table = Table::new_jagged(vec![4, 2, 3]).unwrap();
        table.cursor.set(0, 3).unwrap();

        table.cursor.set_line(1).unwrap();
        assert_eq!(table.cursor.current(), (1, 1));
    }

    #[test]
    fn jagged_table_set_validates_target_row() {
        let mut table = Table::new_jagged(vec![4, 2, 3]).unwrap();

        assert!(table.cursor.set(0, 3).is_ok());
        assert!(table.cursor.set(1, 2).is_err()); // row 1 only has 2 cols (0,1)
        assert!(table.cursor.set(2, 2).is_ok());
    }

    #[test]
    fn jagged_table_right_uses_current_row_width() {
        let mut table = Table::new_jagged(vec![4, 2, 3]).unwrap();
        table.cursor.set(1, 1).unwrap(); // row 1 has 2 cols

        table.cursor.right(); // wraps within row 1
        assert_eq!(table.cursor.current(), (1, 0));
    }

    #[test]
    fn jagged_table_left_uses_current_row_width() {
        let mut table = Table::new_jagged(vec![4, 2, 3]).unwrap();
        table.cursor.set(1, 0).unwrap(); // row 1 has 2 cols

        table.cursor.left(); // wraps within row 1
        assert_eq!(table.cursor.current(), (1, 1));
    }

    #[test]
    fn jagged_table_clamp_mode() {
        let mut table = Table::new_jagged(vec![4, 2, 3])
            .unwrap()
            .wrap_mode(WrapMode::Clamp);
        table.cursor.set(0, 3).unwrap();

        table.cursor.down(); // row 1 has 2 cols, column clamped
        assert_eq!(table.cursor.current(), (1, 1));

        table.cursor.down(); // row 2 has 3 cols, column stays 1
        assert_eq!(table.cursor.current(), (2, 1));

        table.cursor.down(); // clamp at bottom
        assert_eq!(table.cursor.current(), (2, 1));
    }

    #[test]
    fn jagged_table_with_wrap_mode() {
        let mut table = Table::new_jagged(vec![4, 2, 3])
            .unwrap()
            .wrap_mode(WrapMode::Wrap);
        table.cursor.set(0, 0).unwrap();
        table.cursor.up();
        assert_eq!(table.cursor.current(), (2, 0));
    }
}
