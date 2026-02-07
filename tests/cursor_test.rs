#[cfg(test)]
mod tests {
    use rpos::table::Table;
    use rpos::WrapMode;

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

    // Boundary movement tests - verify cursor wraps around
    #[test]
    fn move_up_at_top_boundary_wraps_to_bottom() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        assert_eq!(cursor.current(), (0, 0));

        cursor.up();
        assert_eq!(cursor.current(), (2, 0));

        // Multiple attempts wrap around
        cursor.up();
        assert_eq!(cursor.current(), (1, 0));
    }

    #[test]
    fn move_down_at_bottom_boundary_wraps_to_top() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set(2, 0).unwrap(); // Move to bottom row

        cursor.down();
        assert_eq!(cursor.current(), (0, 0));

        // Multiple attempts wrap around
        cursor.down();
        assert_eq!(cursor.current(), (1, 0));
    }

    #[test]
    fn move_left_at_left_boundary_wraps_to_right() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        assert_eq!(cursor.current(), (0, 0));

        cursor.left();
        assert_eq!(cursor.current(), (0, 3));

        // Multiple attempts wrap around
        cursor.left();
        assert_eq!(cursor.current(), (0, 2));
    }

    #[test]
    fn move_right_at_right_boundary_wraps_to_left() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set(0, 3).unwrap(); // Move to rightmost column

        cursor.right();
        assert_eq!(cursor.current(), (0, 0));

        // Multiple attempts wrap around
        cursor.right();
        assert_eq!(cursor.current(), (0, 1));
    }

    // Corner position tests
    #[test]
    fn set_cursor_to_max_valid_position() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        cursor.set(2, 3).unwrap(); // max line=2, max column=3
        assert_eq!(cursor.current(), (2, 3));
    }

    #[test]
    fn set_cursor_to_all_corners() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        // Top-left (0, 0)
        cursor.set(0, 0).unwrap();
        assert_eq!(cursor.current(), (0, 0));

        // Top-right (0, max_col)
        cursor.set(0, 3).unwrap();
        assert_eq!(cursor.current(), (0, 3));

        // Bottom-left (max_line, 0)
        cursor.set(2, 0).unwrap();
        assert_eq!(cursor.current(), (2, 0));

        // Bottom-right (max_line, max_col)
        cursor.set(2, 3).unwrap();
        assert_eq!(cursor.current(), (2, 3));
    }

    // State consistency tests - failed operations should not change cursor
    #[test]
    fn failed_set_does_not_change_cursor() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set(1, 2).unwrap();

        let _ = cursor.set(10, 10); // Invalid, should fail
        assert_eq!(cursor.current(), (1, 2)); // Should remain unchanged
    }

    #[test]
    fn failed_set_line_does_not_change_cursor() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set(1, 2).unwrap();

        let _ = cursor.set_line(10); // Invalid, should fail
        assert_eq!(cursor.current(), (1, 2)); // Should remain unchanged
    }

    #[test]
    fn failed_set_column_does_not_change_cursor() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set(1, 2).unwrap();

        let _ = cursor.set_column(10); // Invalid, should fail
        assert_eq!(cursor.current(), (1, 2)); // Should remain unchanged
    }

    // Minimum table (1x1) cursor behavior
    #[test]
    fn cursor_in_1x1_table_cannot_move() {
        let mut cursor = Table::new(1, 1).unwrap().cursor;
        assert_eq!(cursor.current(), (0, 0));

        cursor.up();
        assert_eq!(cursor.current(), (0, 0));

        cursor.down();
        assert_eq!(cursor.current(), (0, 0));

        cursor.left();
        assert_eq!(cursor.current(), (0, 0));

        cursor.right();
        assert_eq!(cursor.current(), (0, 0));
    }

    #[test]
    fn cursor_in_single_row_table_wraps_vertically() {
        let mut cursor = Table::new(1, 3).unwrap().cursor;

        // Vertical movement wraps to the same row
        cursor.up();
        assert_eq!(cursor.current(), (0, 0));
        cursor.down();
        assert_eq!(cursor.current(), (0, 0));

        // Horizontal movement should work
        cursor.right();
        assert_eq!(cursor.current(), (0, 1));
        cursor.right();
        assert_eq!(cursor.current(), (0, 2));
        cursor.left();
        assert_eq!(cursor.current(), (0, 1));
    }

    #[test]
    fn cursor_in_single_column_table_wraps_horizontally() {
        let mut cursor = Table::new(3, 1).unwrap().cursor;

        // Horizontal movement wraps to the same column
        cursor.left();
        assert_eq!(cursor.current(), (0, 0));
        cursor.right();
        assert_eq!(cursor.current(), (0, 0));

        // Vertical movement should work
        cursor.down();
        assert_eq!(cursor.current(), (1, 0));
        cursor.down();
        assert_eq!(cursor.current(), (2, 0));
        cursor.up();
        assert_eq!(cursor.current(), (1, 0));
    }

    #[test]
    fn clamp_mode_does_not_wrap_at_boundaries() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set_wrap_mode(WrapMode::Clamp);

        // Top-left stays when moving up/left
        cursor.set(0, 0).unwrap();
        cursor.up();
        cursor.left();
        assert_eq!(cursor.current(), (0, 0));

        // Bottom-right stays when moving down/right
        cursor.set(2, 3).unwrap();
        cursor.down();
        cursor.right();
        assert_eq!(cursor.current(), (2, 3));
    }

    // Out-of-bounds edge cases
    #[test]
    fn set_cursor_line_at_exact_boundary_fails() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        // line_size is 3, so line=3 should fail (0-indexed: valid is 0, 1, 2)
        assert!(cursor.set_line(3).is_err());
    }

    #[test]
    fn set_cursor_column_at_exact_boundary_fails() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        // column_size is 4, so column=4 should fail (0-indexed: valid is 0, 1, 2, 3)
        assert!(cursor.set_column(4).is_err());
    }
}
