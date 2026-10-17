#[cfg(test)]
mod tests {
    use rpos::table::Table;
    use rpos::{Cursor, HorizontalPolicy, VerticalPolicy, WrapMode};

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

    // Jagged array tests
    #[test]
    fn jagged_down_clamps_column_on_shorter_row() {
        // Row 0: 4 cols, Row 1: 2 cols, Row 2: 3 cols
        let mut cursor = Table::new_jagged(vec![4, 2, 3]).unwrap().cursor;
        cursor.set(0, 3).unwrap(); // column 3 on row 0

        cursor.down(); // move to row 1 (only 2 cols: 0,1)
        assert_eq!(cursor.current(), (1, 1)); // clamped to max valid column
    }

    #[test]
    fn jagged_up_clamps_column_on_shorter_row() {
        let mut cursor = Table::new_jagged(vec![4, 2, 3]).unwrap().cursor;
        cursor.set(2, 2).unwrap(); // column 2 on row 2

        cursor.up(); // move to row 1 (only 2 cols: 0,1)
        assert_eq!(cursor.current(), (1, 1)); // clamped
    }

    #[test]
    fn jagged_wrap_around_bottom_to_top_clamps_column() {
        let mut cursor = Table::new_jagged(vec![2, 4, 3]).unwrap().cursor;
        cursor.set(2, 2).unwrap(); // column 2 on row 2

        cursor.down(); // wraps to row 0 (only 2 cols: 0,1)
        assert_eq!(cursor.current(), (0, 1)); // clamped
    }

    #[test]
    fn jagged_wrap_around_top_to_bottom_clamps_column() {
        let mut cursor = Table::new_jagged(vec![4, 3, 2]).unwrap().cursor;
        cursor.set(0, 3).unwrap(); // column 3 on row 0

        cursor.up(); // wraps to row 2 (only 2 cols: 0,1)
        assert_eq!(cursor.current(), (2, 1)); // clamped
    }

    #[test]
    fn jagged_set_line_clamps_column() {
        let mut cursor = Table::new_jagged(vec![4, 2, 3]).unwrap().cursor;
        cursor.set(0, 3).unwrap(); // column 3 on row 0

        cursor.set_line(1).unwrap(); // row 1 has only 2 cols
        assert_eq!(cursor.current(), (1, 1)); // clamped
    }

    #[test]
    fn jagged_set_validates_against_target_row_column_size() {
        let mut cursor = Table::new_jagged(vec![4, 2, 3]).unwrap().cursor;

        // Valid: column 3 on row 0 (4 cols)
        assert!(cursor.set(0, 3).is_ok());

        // Invalid: column 3 on row 1 (only 2 cols)
        assert!(cursor.set(1, 3).is_err());

        // Valid: column 2 on row 2 (3 cols)
        assert!(cursor.set(2, 2).is_ok());

        // Invalid: column 3 on row 2 (only 3 cols)
        assert!(cursor.set(2, 3).is_err());
    }

    #[test]
    fn jagged_failed_set_does_not_change_cursor() {
        let mut cursor = Table::new_jagged(vec![4, 2, 3]).unwrap().cursor;
        cursor.set(0, 3).unwrap();

        let _ = cursor.set(1, 3); // Invalid for row 1, should fail
        assert_eq!(cursor.current(), (0, 3)); // Should remain unchanged
    }

    #[test]
    fn jagged_right_uses_current_row_width() {
        let mut cursor = Table::new_jagged(vec![4, 2, 3]).unwrap().cursor;
        cursor.set(1, 0).unwrap(); // row 1 has 2 cols

        cursor.right();
        assert_eq!(cursor.current(), (1, 1));

        cursor.right(); // wraps within row 1
        assert_eq!(cursor.current(), (1, 0));
    }

    #[test]
    fn jagged_left_uses_current_row_width() {
        let mut cursor = Table::new_jagged(vec![4, 2, 3]).unwrap().cursor;
        cursor.set(1, 0).unwrap(); // row 1 has 2 cols

        cursor.left(); // wraps within row 1
        assert_eq!(cursor.current(), (1, 1));
    }

    #[test]
    fn jagged_set_column_validates_against_current_row() {
        let mut cursor = Table::new_jagged(vec![4, 2, 3]).unwrap().cursor;
        cursor.set(1, 0).unwrap(); // row 1 has 2 cols

        assert!(cursor.set_column(1).is_ok());
        assert!(cursor.set_column(2).is_err()); // out of bounds for row 1
    }

    // VerticalPolicy::SkipMissing — keep column, skip rows that lack it
    #[test]
    fn skip_missing_down_skips_shorter_rows() {
        // Row 0: 4 cols, Row 1: 2 cols, Row 2: 3 cols
        let mut cursor = Table::new_jagged(vec![4, 2, 3])
            .unwrap()
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.down(); // row 1 lacks col 3; skip to… nowhere with col 3 under wrap? only row 0 has 4 cols
                       // Only row 0 has column 3, so wrap finds no other row → stay
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn skip_missing_down_moves_to_next_row_with_column() {
        // Row 0: 4, Row 1: 2, Row 2: 4 — col 3 exists on rows 0 and 2
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.down();
        assert_eq!(cursor.current(), (2, 3)); // skipped row 1
    }

    #[test]
    fn skip_missing_up_moves_to_previous_row_with_column() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(2, 3).unwrap();

        cursor.up();
        assert_eq!(cursor.current(), (0, 3)); // skipped row 1
    }

    #[test]
    fn skip_missing_wrap_from_bottom_to_top() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .wrap_mode(WrapMode::Wrap)
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(2, 3).unwrap();

        cursor.down(); // wrap: skip row 1, land on row 0
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn skip_missing_wrap_from_top_to_bottom() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .wrap_mode(WrapMode::Wrap)
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.up(); // wrap: skip row 1, land on row 2
        assert_eq!(cursor.current(), (2, 3));
    }

    #[test]
    fn skip_missing_clamp_stays_when_no_row_above() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .wrap_mode(WrapMode::Clamp)
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.up();
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn skip_missing_clamp_stays_when_no_row_below_with_column() {
        let mut cursor = Table::new_jagged(vec![4, 2, 3])
            .unwrap()
            .wrap_mode(WrapMode::Clamp)
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.down(); // rows 1 and 2 lack col 3
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn skip_missing_clamp_skips_intermediate_shorter_rows() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .wrap_mode(WrapMode::Clamp)
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.down();
        assert_eq!(cursor.current(), (2, 3));

        cursor.down(); // already at last eligible row
        assert_eq!(cursor.current(), (2, 3));
    }

    #[test]
    fn skip_missing_preserves_column_on_adjacent_equal_width() {
        let mut cursor = Table::new_jagged(vec![3, 3, 2])
            .unwrap()
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(0, 2).unwrap();

        cursor.down();
        assert_eq!(cursor.current(), (1, 2));
    }

    #[test]
    fn default_vertical_policy_is_clamp_column() {
        let cursor = Table::new_jagged(vec![4, 2, 3]).unwrap().cursor;
        assert_eq!(cursor.vertical_policy(), VerticalPolicy::ClampColumn);
    }

    #[test]
    fn set_vertical_policy_updates_behavior() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4]).unwrap().cursor;
        cursor.set(0, 3).unwrap();

        // Default ClampColumn: moves to row 1 and clamps
        cursor.down();
        assert_eq!(cursor.current(), (1, 1));

        cursor.set(0, 3).unwrap();
        cursor.set_vertical_policy(VerticalPolicy::SkipMissing);
        cursor.down();
        assert_eq!(cursor.current(), (2, 3));
    }

    // HorizontalPolicy::AcrossRows
    #[test]
    fn across_rows_right_moves_to_next_row() {
        let mut cursor = Table::new(3, 2)
            .unwrap()
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;
        cursor.set(0, 1).unwrap();

        cursor.right();
        assert_eq!(cursor.current(), (1, 0));
    }

    #[test]
    fn across_rows_left_moves_to_previous_row_end() {
        let mut cursor = Table::new(3, 2)
            .unwrap()
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;
        cursor.set(1, 0).unwrap();

        cursor.left();
        assert_eq!(cursor.current(), (0, 1));
    }

    #[test]
    fn across_rows_wrap_from_last_cell_to_first() {
        let mut cursor = Table::new(2, 2)
            .unwrap()
            .wrap_mode(WrapMode::Wrap)
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;
        cursor.set(1, 1).unwrap();

        cursor.right();
        assert_eq!(cursor.current(), (0, 0));
    }

    #[test]
    fn across_rows_wrap_from_first_cell_to_last() {
        let mut cursor = Table::new(2, 2)
            .unwrap()
            .wrap_mode(WrapMode::Wrap)
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;

        cursor.left();
        assert_eq!(cursor.current(), (1, 1));
    }

    #[test]
    fn across_rows_clamp_stays_at_last_cell() {
        let mut cursor = Table::new(2, 2)
            .unwrap()
            .wrap_mode(WrapMode::Clamp)
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;
        cursor.set(1, 1).unwrap();

        cursor.right();
        assert_eq!(cursor.current(), (1, 1));
    }

    #[test]
    fn across_rows_clamp_stays_at_first_cell() {
        let mut cursor = Table::new(2, 2)
            .unwrap()
            .wrap_mode(WrapMode::Clamp)
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;

        cursor.left();
        assert_eq!(cursor.current(), (0, 0));
    }

    #[test]
    fn across_rows_works_on_jagged_tables() {
        let mut cursor = Table::new_jagged(vec![3, 1, 2])
            .unwrap()
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;
        cursor.set(0, 2).unwrap();

        cursor.right();
        assert_eq!(cursor.current(), (1, 0));

        cursor.right();
        assert_eq!(cursor.current(), (2, 0));

        cursor.left();
        assert_eq!(cursor.current(), (1, 0));
    }

    #[test]
    fn default_horizontal_policy_is_within_row() {
        let cursor = Table::new(2, 2).unwrap().cursor;
        assert_eq!(cursor.horizontal_policy(), HorizontalPolicy::WithinRow);
    }

    // home / end / row_start / row_end
    #[test]
    fn home_moves_to_origin() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set(2, 3).unwrap();
        cursor.home();
        assert_eq!(cursor.current(), (0, 0));
    }

    #[test]
    fn end_moves_to_last_cell() {
        let mut cursor = Table::new_jagged(vec![3, 1, 2]).unwrap().cursor;
        cursor.home();
        cursor.end();
        assert_eq!(cursor.current(), (2, 1));
    }

    #[test]
    fn row_start_and_row_end() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set(1, 2).unwrap();

        cursor.row_start();
        assert_eq!(cursor.current(), (1, 0));

        cursor.row_end();
        assert_eq!(cursor.current(), (1, 3));
    }

    // linear index
    #[test]
    fn index_is_row_major() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set(1, 2).unwrap();
        assert_eq!(cursor.index(), 6); // 4 + 2
        assert_eq!(cursor.cell_count(), 12);
    }

    #[test]
    fn index_on_jagged_table() {
        let mut cursor = Table::new_jagged(vec![3, 1, 2]).unwrap().cursor;
        cursor.set(2, 1).unwrap();
        assert_eq!(cursor.index(), 3 + 1 + 1);
        assert_eq!(cursor.cell_count(), 6);
    }

    #[test]
    fn set_index_round_trips() {
        let mut cursor = Table::new_jagged(vec![3, 1, 2]).unwrap().cursor;
        for i in 0..cursor.cell_count() {
            cursor.set_index(i).unwrap();
            assert_eq!(cursor.index(), i);
        }
    }

    #[test]
    fn set_index_out_of_range_fails() {
        let mut cursor = Table::new(2, 2).unwrap().cursor;
        assert!(cursor.set_index(4).is_err());
        assert_eq!(cursor.current(), (0, 0));
    }

    // set_column_sizes / accessors
    #[test]
    fn set_column_sizes_clamps_cursor() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set(2, 3).unwrap();

        cursor.set_column_sizes(vec![2, 2]).unwrap();
        assert_eq!(cursor.line_count(), 2);
        assert_eq!(cursor.current(), (1, 1));
        assert_eq!(cursor.column_sizes(), &[2, 2]);
    }

    #[test]
    fn set_column_sizes_rejects_invalid() {
        let mut cursor = Table::new(2, 2).unwrap().cursor;
        assert!(cursor.set_column_sizes(vec![]).is_err());
        assert!(cursor.set_column_sizes(vec![2, 0]).is_err());
    }

    #[test]
    fn line_and_column_accessors() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;
        cursor.set(2, 3).unwrap();
        assert_eq!(cursor.line(), 2);
        assert_eq!(cursor.column(), 3);
    }

    #[test]
    fn cursor_is_cloneable() {
        let cursor = Table::new(2, 3).unwrap().cursor;
        let cloned = cursor.clone();
        assert_eq!(cloned.current(), (0, 0));
        assert_eq!(cloned.column_sizes(), &[3, 3]);
    }

    #[test]
    fn cursor_new_rejects_zero_dimensions() {
        assert!(Cursor::new((0, 3)).is_err());
        assert!(Cursor::new((3, 0)).is_err());
        assert!(Cursor::new_jagged(vec![]).is_err());
        assert!(Cursor::new_jagged(vec![2, 0]).is_err());
    }

    #[test]
    fn across_rows_and_skip_missing_are_independent() {
        let mut cursor = Table::new_jagged(vec![3, 1, 3])
            .unwrap()
            .vertical_policy(VerticalPolicy::SkipMissing)
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;
        cursor.set(0, 2).unwrap();

        // AcrossRows: end of row → next row start (does not skip)
        cursor.right();
        assert_eq!(cursor.current(), (1, 0));

        // SkipMissing: from col 2, down skips the short middle row
        cursor.set(0, 2).unwrap();
        cursor.down();
        assert_eq!(cursor.current(), (2, 2));
    }

    #[test]
    fn clamp_column_does_not_restore_preferred_column() {
        // Documented behavior: ClampColumn forgets the wider column after a short row
        let mut cursor = Table::new_jagged(vec![4, 2, 4]).unwrap().cursor;
        cursor.set(0, 3).unwrap();
        cursor.down();
        assert_eq!(cursor.current(), (1, 1));
        cursor.down();
        assert_eq!(cursor.current(), (2, 1)); // not (2, 3)
    }

    // VerticalPolicy::StickyColumn — land on short rows, restore on wider ones
    fn sticky_cursor(widths: Vec<usize>) -> Cursor {
        Table::new_jagged(widths)
            .unwrap()
            .vertical_policy(VerticalPolicy::StickyColumn)
            .cursor
    }

    #[test]
    fn sticky_down_lands_on_short_row_then_restores() {
        let mut cursor = sticky_cursor(vec![4, 2, 4]);
        cursor.set(0, 3).unwrap();
        assert_eq!(cursor.preferred_column(), 3);

        cursor.down();
        assert_eq!(cursor.current(), (1, 1));
        assert_eq!(cursor.preferred_column(), 3);

        cursor.down();
        assert_eq!(cursor.current(), (2, 3));
        assert_eq!(cursor.preferred_column(), 3);
    }

    #[test]
    fn sticky_up_restores_preferred_column() {
        let mut cursor = sticky_cursor(vec![4, 2, 4]);
        cursor.set(2, 3).unwrap();

        cursor.up();
        assert_eq!(cursor.current(), (1, 1));

        cursor.up();
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn sticky_wrap_restores_across_the_edge() {
        let mut cursor = sticky_cursor(vec![4, 2, 4]);
        cursor.set(2, 3).unwrap();

        cursor.down();
        assert_eq!(cursor.current(), (0, 3));

        cursor.up();
        assert_eq!(cursor.current(), (2, 3));
    }

    #[test]
    fn sticky_clamp_stays_on_the_edge_row() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .wrap_mode(WrapMode::Clamp)
            .vertical_policy(VerticalPolicy::StickyColumn)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.up();
        assert_eq!(cursor.current(), (0, 3));

        cursor.set(2, 3).unwrap();
        cursor.down();
        assert_eq!(cursor.current(), (2, 3));
    }

    #[test]
    fn sticky_horizontal_move_updates_preferred_column() {
        let mut cursor = sticky_cursor(vec![4, 2, 4]);
        cursor.set(0, 3).unwrap();
        cursor.down();
        assert_eq!(cursor.current(), (1, 1));

        cursor.left();
        assert_eq!(cursor.current(), (1, 0));
        assert_eq!(cursor.preferred_column(), 0);

        cursor.down();
        assert_eq!(cursor.current(), (2, 0));
    }

    #[test]
    fn sticky_right_at_short_row_end_adopts_that_column() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .wrap_mode(WrapMode::Clamp)
            .vertical_policy(VerticalPolicy::StickyColumn)
            .cursor;
        cursor.set(0, 3).unwrap();
        cursor.down();
        assert_eq!(cursor.current(), (1, 1));

        cursor.right(); // already at the row end; the current column becomes preferred
        assert_eq!(cursor.preferred_column(), 1);

        cursor.down();
        assert_eq!(cursor.current(), (2, 1));
    }

    #[test]
    fn sticky_across_rows_updates_preferred_column() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .vertical_policy(VerticalPolicy::StickyColumn)
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.right();
        assert_eq!(cursor.current(), (1, 0));
        assert_eq!(cursor.preferred_column(), 0);

        cursor.down();
        assert_eq!(cursor.current(), (2, 0));
    }

    #[test]
    fn sticky_set_column_sizes_keeps_preferred_column() {
        let mut cursor = sticky_cursor(vec![4, 4]);
        cursor.set(0, 3).unwrap();

        cursor.set_column_sizes(vec![2, 4]).unwrap();
        assert_eq!(cursor.current(), (0, 1));
        assert_eq!(cursor.preferred_column(), 3);

        cursor.down();
        assert_eq!(cursor.current(), (1, 3));
    }

    #[test]
    fn sticky_explicit_column_changes_update_preferred_column() {
        let mut cursor = sticky_cursor(vec![4, 2, 4]);
        cursor.set_index(3).unwrap();
        assert_eq!(cursor.current(), (0, 3));
        assert_eq!(cursor.preferred_column(), 3);

        cursor.home();
        assert_eq!(cursor.preferred_column(), 0);
        cursor.down();
        cursor.down();
        assert_eq!(cursor.current(), (2, 0));

        cursor.row_end();
        assert_eq!(cursor.preferred_column(), 3);
        cursor.up();
        assert_eq!(cursor.current(), (1, 1));
        cursor.up();
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn sticky_set_line_does_not_change_preferred_column() {
        let mut cursor = sticky_cursor(vec![4, 2, 4]);
        cursor.set(0, 3).unwrap();
        cursor.down();
        assert_eq!(cursor.current(), (1, 1));

        cursor.set_line(2).unwrap();
        assert_eq!(cursor.current(), (2, 1));
        assert_eq!(cursor.preferred_column(), 3);

        cursor.down(); // wrap to row 0 and restore
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn shift_line_moves_by_delta_and_stops_at_the_edge() {
        let mut cursor = Table::new(5, 3).unwrap().cursor;
        assert_eq!(cursor.wrap_mode(), Some(WrapMode::Wrap));

        cursor.shift_line(2);
        assert_eq!(cursor.current(), (2, 0));

        cursor.shift_line(10);
        assert_eq!(cursor.current(), (4, 0));

        cursor.shift_line(-2);
        assert_eq!(cursor.current(), (2, 0));

        cursor.shift_line(-10);
        assert_eq!(cursor.current(), (0, 0));

        cursor.shift_line(0);
        assert_eq!(cursor.current(), (0, 0));
        assert_eq!(cursor.wrap_mode(), Some(WrapMode::Wrap));
    }

    #[test]
    fn shift_column_moves_within_the_row_and_stops_at_the_edge() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        cursor.shift_column(2);
        assert_eq!(cursor.current(), (0, 2));

        cursor.shift_column(10);
        assert_eq!(cursor.current(), (0, 3));

        cursor.shift_column(-1);
        assert_eq!(cursor.current(), (0, 2));

        cursor.shift_column(-10);
        assert_eq!(cursor.current(), (0, 0));
        assert_eq!(cursor.wrap_mode(), Some(WrapMode::Wrap));
    }

    #[test]
    fn shift_line_one_step_matches_down_until_the_edge() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4]).unwrap().cursor;
        cursor.set(0, 3).unwrap();

        cursor.shift_line(1);
        assert_eq!(cursor.current(), (1, 1)); // same clamp as down()

        cursor.shift_line(1);
        assert_eq!(cursor.current(), (2, 1));

        cursor.shift_line(1); // down() would wrap to row 0
        assert_eq!(cursor.current(), (2, 1));
    }

    #[test]
    fn shift_line_skip_missing_stops_on_the_last_matching_row() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4, 2, 4])
            .unwrap()
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.shift_line(1);
        assert_eq!(cursor.current(), (2, 3));

        cursor.shift_line(10);
        assert_eq!(cursor.current(), (4, 3));

        cursor.shift_line(-1);
        assert_eq!(cursor.current(), (2, 3));

        cursor.shift_line(-10);
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn shift_line_sticky_restores_without_wrapping() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .vertical_policy(VerticalPolicy::StickyColumn)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.shift_line(1);
        assert_eq!(cursor.current(), (1, 1));
        assert_eq!(cursor.preferred_column(), 3);

        cursor.shift_line(5);
        assert_eq!(cursor.current(), (2, 3));
        assert_eq!(cursor.preferred_column(), 3);
        assert_eq!(cursor.wrap_mode(), Some(WrapMode::Wrap));
    }

    #[test]
    fn shift_column_across_rows_stops_at_the_last_cell() {
        let mut cursor = Table::new(2, 2)
            .unwrap()
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;

        cursor.shift_column(1);
        assert_eq!(cursor.current(), (0, 1));

        cursor.shift_column(10);
        assert_eq!(cursor.current(), (1, 1));

        cursor.shift_column(-3);
        assert_eq!(cursor.current(), (0, 0));

        cursor.shift_column(-1);
        assert_eq!(cursor.current(), (0, 0));
        assert_eq!(cursor.preferred_column(), 0);
    }

    #[test]
    fn shift_column_updates_preferred_column() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .vertical_policy(VerticalPolicy::StickyColumn)
            .cursor;
        cursor.set(0, 3).unwrap();

        cursor.shift_column(-1);
        assert_eq!(cursor.current(), (0, 2));
        assert_eq!(cursor.preferred_column(), 2);

        cursor.shift_line(2);
        assert_eq!(cursor.current(), (2, 2));
    }

    #[test]
    fn clamp_move_reports_whether_the_cell_changed() {
        let mut cursor = Table::new(3, 4).unwrap().wrap_mode(WrapMode::Clamp).cursor;

        assert!(!cursor.up());
        assert!(!cursor.left());
        assert_eq!(cursor.current(), (0, 0));

        assert!(cursor.down());
        assert!(cursor.right());
        assert_eq!(cursor.current(), (1, 1));

        cursor.set(2, 3).unwrap();
        assert!(!cursor.down());
        assert!(!cursor.right());
        assert_eq!(cursor.current(), (2, 3));
    }

    #[test]
    fn wrap_move_reports_a_change_at_the_edge() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        assert!(cursor.up());
        assert_eq!(cursor.current(), (2, 0));
        assert!(cursor.left());
        assert_eq!(cursor.current(), (2, 3));
    }

    #[test]
    fn move_on_a_single_cell_reports_no_change() {
        let mut cursor = Table::new(1, 1).unwrap().cursor;

        assert!(!cursor.up());
        assert!(!cursor.down());
        assert!(!cursor.left());
        assert!(!cursor.right());
        assert_eq!(cursor.current(), (0, 0));
    }

    #[test]
    fn skip_missing_reports_when_no_row_has_the_column() {
        let mut cursor = Table::new_jagged(vec![4, 2, 3])
            .unwrap()
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(0, 3).unwrap();

        assert!(!cursor.down());
        assert_eq!(cursor.current(), (0, 3));

        cursor.set_column_sizes(vec![4, 2, 4]).unwrap();
        assert!(cursor.down());
        assert_eq!(cursor.current(), (2, 3));
    }

    #[test]
    fn sticky_clamp_reports_no_change_at_the_edge() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .wrap_mode(WrapMode::Clamp)
            .vertical_policy(VerticalPolicy::StickyColumn)
            .cursor;
        cursor.set(0, 3).unwrap();

        assert!(!cursor.up());
        assert_eq!(cursor.current(), (0, 3));

        assert!(cursor.down());
        assert_eq!(cursor.current(), (1, 1));
        assert_eq!(cursor.preferred_column(), 3);
    }

    #[test]
    fn across_rows_clamp_reports_no_change_at_the_last_cell() {
        let mut cursor = Table::new(2, 2)
            .unwrap()
            .wrap_mode(WrapMode::Clamp)
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;
        cursor.set(1, 1).unwrap();

        assert!(!cursor.right());
        assert_eq!(cursor.current(), (1, 1));

        assert!(cursor.left());
        assert_eq!(cursor.current(), (1, 0));
    }

    #[test]
    fn set_wrap_mode_sets_both_axes() {
        let mut cursor = Table::new(2, 2).unwrap().cursor;
        cursor.set_wrap_mode(WrapMode::Clamp);

        assert_eq!(cursor.wrap_mode(), Some(WrapMode::Clamp));
        assert_eq!(cursor.vertical_wrap_mode(), WrapMode::Clamp);
        assert_eq!(cursor.horizontal_wrap_mode(), WrapMode::Clamp);
    }

    #[test]
    fn vertical_clamp_does_not_stop_horizontal_wrap() {
        let mut cursor = Table::new(3, 4)
            .unwrap()
            .vertical_wrap_mode(WrapMode::Clamp)
            .horizontal_wrap_mode(WrapMode::Wrap)
            .cursor;

        assert_eq!(cursor.wrap_mode(), None);
        assert!(!cursor.up());
        assert_eq!(cursor.current(), (0, 0));
        assert!(cursor.left());
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn horizontal_clamp_does_not_stop_vertical_wrap() {
        let mut cursor = Table::new(3, 4)
            .unwrap()
            .vertical_wrap_mode(WrapMode::Wrap)
            .horizontal_wrap_mode(WrapMode::Clamp)
            .cursor;

        assert!(cursor.up());
        assert_eq!(cursor.current(), (2, 0));
        assert!(!cursor.left());
        assert_eq!(cursor.current(), (2, 0));
    }

    #[test]
    fn across_rows_uses_the_horizontal_wrap_mode() {
        let mut cursor = Table::new(2, 2)
            .unwrap()
            .vertical_wrap_mode(WrapMode::Clamp)
            .horizontal_wrap_mode(WrapMode::Wrap)
            .horizontal_policy(HorizontalPolicy::AcrossRows)
            .cursor;
        cursor.set(1, 1).unwrap();

        assert!(cursor.right());
        assert_eq!(cursor.current(), (0, 0));
        assert!(!cursor.up());
    }

    #[test]
    fn skip_missing_uses_the_vertical_wrap_mode() {
        let mut cursor = Table::new_jagged(vec![4, 2, 4])
            .unwrap()
            .vertical_wrap_mode(WrapMode::Wrap)
            .horizontal_wrap_mode(WrapMode::Clamp)
            .vertical_policy(VerticalPolicy::SkipMissing)
            .cursor;
        cursor.set(2, 3).unwrap();

        assert!(cursor.down());
        assert_eq!(cursor.current(), (0, 3));

        cursor.set_vertical_wrap_mode(WrapMode::Clamp);
        cursor.set(0, 3).unwrap();
        assert!(cursor.down());
        assert_eq!(cursor.current(), (2, 3));
        assert!(!cursor.down());
    }

    #[test]
    fn shift_line_clamps_only_the_vertical_axis() {
        let mut cursor = Table::new(3, 4).unwrap().cursor;

        cursor.shift_line(10);
        assert_eq!(cursor.current(), (2, 0));
        assert_eq!(cursor.vertical_wrap_mode(), WrapMode::Wrap);
        assert_eq!(cursor.horizontal_wrap_mode(), WrapMode::Wrap);

        assert!(cursor.left());
        assert_eq!(cursor.current(), (2, 3));
    }
}
