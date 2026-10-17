#[cfg(test)]
mod tests {
    use rpos::table::Table;
    use rpos::{Cursor, Error, HorizontalPolicy, Selection, VerticalPolicy, WrapMode};

    fn sample() -> Cursor {
        // Row 0: 0, 2, 5. Row 1: 0, 1. Row 2: 1, 2, 4.
        Cursor::new_sparse(vec![vec![0, 2, 5], vec![0, 1], vec![1, 2, 4]]).unwrap()
    }

    #[test]
    fn prefix_rows_match_a_jagged_cursor() {
        let jagged = Cursor::new_jagged(vec![4, 2, 4]).unwrap();
        let sparse =
            Cursor::new_sparse(vec![vec![0, 1, 2, 3], vec![0, 1], vec![3, 2, 1, 0]]).unwrap();

        assert_eq!(jagged, sparse);
        assert_eq!(jagged.column_sizes(), sparse.column_sizes());
        assert_eq!(jagged.row_columns(), sparse.row_columns());
    }

    #[test]
    fn new_sparse_sorts_columns_and_starts_on_the_first_cell() {
        let cursor = Cursor::new_sparse(vec![vec![5, 2], vec![3, 1]]).unwrap();

        assert_eq!(cursor.row_columns(), &[vec![2, 5], vec![1, 3]]);
        assert_eq!(cursor.column_sizes(), &[6, 4]);
        assert_eq!(cursor.current(), (0, 2));
        assert_eq!(cursor.preferred_column(), 2);
        assert_eq!(cursor.cell_count(), 4);
        assert_eq!(cursor.index(), 0);
    }

    #[test]
    fn new_sparse_rejects_empty_duplicate_and_unsized_columns() {
        assert_eq!(
            Cursor::new_sparse(vec![]).unwrap_err(),
            Error::InvalidColumnSizes
        );
        assert_eq!(
            Cursor::new_sparse(vec![vec![]]).unwrap_err(),
            Error::InvalidColumnSizes
        );
        assert_eq!(
            Cursor::new_sparse(vec![vec![1, 2, 1]]).unwrap_err(),
            Error::InvalidColumnSizes
        );
        assert_eq!(
            Cursor::new_sparse(vec![vec![usize::MAX]]).unwrap_err(),
            Error::InvalidColumnSizes
        );
        assert_eq!(
            Table::new_sparse(vec![vec![1, 1]]).unwrap_err(),
            Error::InvalidTableSize
        );
        assert_eq!(
            Table::new_sparse(vec![]).unwrap_err(),
            Error::InvalidTableSize
        );
    }

    #[test]
    fn set_rejects_a_hole_and_a_column_past_the_span() {
        let mut cursor = sample();

        assert_eq!(
            cursor.set(0, 1).unwrap_err(),
            Error::MissingCell { line: 0, column: 1 }
        );
        assert_eq!(
            cursor.set(0, 6).unwrap_err(),
            Error::ColumnOutOfBounds {
                column: 6,
                column_size: 6
            }
        );
        assert_eq!(
            Error::MissingCell { line: 0, column: 1 }.to_string(),
            "missing cell at line 0, column 1 (0-indexed)"
        );
        assert_eq!(cursor.current(), (0, 0));
        assert!(!cursor.has_cell(0, 1));
        assert!(cursor.has_cell(0, 2));
    }

    #[test]
    fn horizontal_movement_steps_between_present_cells() {
        let mut cursor = sample();
        cursor.set_wrap_mode(WrapMode::Clamp);

        assert!(cursor.right());
        assert_eq!(cursor.current(), (0, 2));
        assert_eq!(cursor.preferred_column(), 2);
        cursor.shift_column(1);
        assert_eq!(cursor.current(), (0, 5));
        assert!(!cursor.right());

        cursor.set_wrap_mode(WrapMode::Wrap);
        assert!(cursor.right());
        assert_eq!(cursor.current(), (0, 0));
        assert!(cursor.left());
        assert_eq!(cursor.current(), (0, 5));
    }

    #[test]
    fn single_present_cell_does_not_move_when_it_wraps() {
        let mut cursor = Cursor::new_sparse(vec![vec![3]]).unwrap();

        assert!(!cursor.left());
        assert!(!cursor.right());
        assert_eq!(cursor.current(), (0, 3));
    }

    #[test]
    fn across_rows_lands_on_the_first_and_last_present_cell() {
        let mut cursor = Cursor::new_sparse(vec![vec![0, 2], vec![1, 3]]).unwrap();
        cursor.set_horizontal_policy(HorizontalPolicy::AcrossRows);
        cursor.set_wrap_mode(WrapMode::Clamp);

        cursor.set(0, 2).unwrap();
        assert!(cursor.right());
        assert_eq!(cursor.current(), (1, 1));

        assert!(cursor.left());
        assert_eq!(cursor.current(), (0, 2));
        cursor.row_start();
        assert_eq!(cursor.current(), (0, 0));
        assert!(!cursor.left());
    }

    #[test]
    fn clamp_column_snaps_off_a_hole_toward_the_previous_cell() {
        let mut cursor = sample();
        cursor.set_wrap_mode(WrapMode::Clamp);
        cursor.set(0, 2).unwrap();

        assert!(cursor.down());
        assert_eq!(cursor.current(), (1, 1));

        cursor.set(0, 5).unwrap();
        cursor.down();
        assert_eq!(cursor.current(), (1, 1));
    }

    #[test]
    fn set_line_snaps_off_a_hole_without_changing_the_preferred_column() {
        let mut cursor = sample();
        cursor.set(0, 2).unwrap();
        assert_eq!(cursor.preferred_column(), 2);

        cursor.set_line(1).unwrap();
        assert_eq!(cursor.current(), (1, 1));
        assert_eq!(cursor.preferred_column(), 2);
    }

    #[test]
    fn skip_missing_skips_a_wider_row_that_lacks_the_column() {
        // Row 1 is wider than row 0, and column 5 is a hole in it.
        let mut cursor =
            Cursor::new_sparse(vec![vec![0, 2, 5], vec![0, 1, 3, 4, 6], vec![0, 2, 5]]).unwrap();
        cursor.set_vertical_policy(VerticalPolicy::SkipMissing);
        cursor.set_wrap_mode(WrapMode::Clamp);
        cursor.set(0, 5).unwrap();

        assert!(cursor.down());
        assert_eq!(cursor.current(), (2, 5));

        cursor.set(0, 2).unwrap();
        cursor.shift_line(1);
        assert_eq!(cursor.current(), (2, 2));
    }

    #[test]
    fn sticky_column_restores_a_present_column_and_uses_the_last_cell_for_a_hole() {
        let mut cursor = Cursor::new_sparse(vec![vec![0, 2, 5], vec![0, 1], vec![0, 5]]).unwrap();
        cursor.set_vertical_policy(VerticalPolicy::StickyColumn);
        cursor.set_wrap_mode(WrapMode::Clamp);
        cursor.set(0, 5).unwrap();

        cursor.down();
        assert_eq!(cursor.current(), (1, 1));
        assert_eq!(cursor.preferred_column(), 5);
        cursor.down();
        assert_eq!(cursor.current(), (2, 5));
    }

    #[test]
    fn index_counts_present_cells_only() {
        let mut cursor = sample();

        assert_eq!(cursor.cell_count(), 8);
        cursor.set(0, 5).unwrap();
        assert_eq!(cursor.index(), 2);
        cursor.set_index(6).unwrap();
        assert_eq!(cursor.current(), (2, 2));
        assert_eq!(cursor.preferred_column(), 2);
        cursor.home();
        assert_eq!(cursor.current(), (0, 0));
        cursor.end();
        assert_eq!(cursor.current(), (2, 4));
    }

    #[test]
    fn set_rows_clamps_onto_a_present_cell_and_keeps_the_preferred_column() {
        let mut table = Table::new_sparse(vec![vec![0, 2, 5]]).unwrap();
        table.cursor.set(0, 5).unwrap();

        table.set_rows(vec![vec![0, 1], vec![0]]).unwrap();
        assert_eq!(table.cursor.current(), (0, 1));
        assert_eq!(table.cursor.preferred_column(), 5);
        assert_eq!(table.height(), 2);
        assert_eq!(table.width(), 2);

        assert_eq!(
            table.set_rows(vec![vec![]]).unwrap_err(),
            Error::InvalidTableSize
        );
        assert_eq!(table.cursor.current(), (0, 1));
    }

    #[test]
    fn selection_omits_holes() {
        let cursor = sample();
        let selection = Selection::new((0, 0), (2, 4));

        assert_eq!(
            selection
                .cells_rows(cursor.row_columns())
                .collect::<Vec<_>>(),
            vec![(0, 0), (0, 2), (1, 0), (1, 1), (2, 1), (2, 2), (2, 4)]
        );
        assert!(selection.contains_rows(cursor.row_columns(), (0, 2)));
        assert!(!selection.contains_rows(cursor.row_columns(), (0, 1)));
        assert!(!selection.contains_rows(cursor.row_columns(), (0, 5)));

        let prefix = Cursor::new_jagged(vec![4, 2, 4]).unwrap();
        let prefix_selection = Selection::anchored(&prefix, (0, 1)).unwrap();
        let from_widths = prefix_selection
            .cells(prefix.column_sizes())
            .collect::<Vec<_>>();
        let from_rows = prefix_selection
            .cells_rows(prefix.row_columns())
            .collect::<Vec<_>>();
        assert_eq!(from_widths, from_rows);

        assert_eq!(
            Selection::anchored(&cursor, (0, 1)).unwrap_err(),
            Error::MissingCell { line: 0, column: 1 }
        );
        assert_eq!(
            Selection::anchored(&cursor, (0, 6)).unwrap_err(),
            Error::ColumnOutOfBounds {
                column: 6,
                column_size: 6
            }
        );
    }
}
