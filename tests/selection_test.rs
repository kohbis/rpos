#[cfg(test)]
mod tests {
    use rpos::table::Table;
    use rpos::Selection;

    #[test]
    fn new_normalizes_bounds_when_focus_is_above_the_anchor() {
        let selection = Selection::new((2, 3), (0, 1));

        assert_eq!(selection.anchor(), (2, 3));
        assert_eq!(selection.focus(), (0, 1));
        assert_eq!(selection.lines().collect::<Vec<_>>(), vec![0, 1, 2]);
        assert_eq!(selection.columns().collect::<Vec<_>>(), vec![1, 2, 3]);
    }

    #[test]
    fn collapsed_selection_is_one_cell() {
        let selection = Selection::new((1, 2), (1, 2));
        let widths = [4, 4, 4];

        assert_eq!(selection.cells(&widths).collect::<Vec<_>>(), vec![(1, 2)]);
        assert!(selection.contains(&widths, (1, 2)));
        assert!(!selection.contains(&widths, (1, 1)));
    }

    #[test]
    fn jagged_cells_omit_columns_past_a_row() {
        // Columns 1..=3. Row 1 only has columns 0 and 1.
        let selection = Selection::new((0, 1), (2, 3));
        let widths = [4, 2, 4];

        assert_eq!(
            selection.cells(&widths).collect::<Vec<_>>(),
            vec![(0, 1), (0, 2), (0, 3), (1, 1), (2, 1), (2, 2), (2, 3)]
        );
        assert!(selection.contains(&widths, (1, 1)));
        assert!(!selection.contains(&widths, (1, 2)));
        assert!(!selection.contains(&widths, (1, 3)));
    }

    #[test]
    fn row_shorter_than_the_column_range_contributes_nothing() {
        // Columns 3..=4. Row 1 has width 2, so it has no selected cell.
        let selection = Selection::new((0, 3), (2, 4));
        let widths = [4, 2, 5];

        assert_eq!(
            selection.cells(&widths).collect::<Vec<_>>(),
            vec![(0, 3), (2, 3), (2, 4)]
        );
        assert!(!selection.contains(&widths, (1, 3)));
    }

    #[test]
    fn contains_rejects_positions_outside_the_table() {
        let selection = Selection::new((0, 0), (1, 1));
        let widths = [2, 2];

        assert!(!selection.contains(&widths, (2, 0)));
        assert!(!selection.contains(&[], (0, 0)));
    }

    #[test]
    fn at_cursor_starts_on_the_current_cell() {
        let mut table = Table::new_jagged(vec![4, 2, 4]).unwrap();
        table.cursor.set(1, 1).unwrap();

        let selection = Selection::at_cursor(&table.cursor);
        assert_eq!(selection.anchor(), (1, 1));
        assert_eq!(selection.focus(), (1, 1));
    }

    #[test]
    fn anchored_uses_the_cursor_as_focus() {
        let mut table = Table::new_jagged(vec![4, 2, 4]).unwrap();
        table.cursor.set(2, 3).unwrap();

        let selection = Selection::anchored(&table.cursor, (0, 1)).unwrap();
        assert_eq!(selection.anchor(), (0, 1));
        assert_eq!(selection.focus(), (2, 3));
        assert_eq!(
            selection
                .cells(table.cursor.column_sizes())
                .collect::<Vec<_>>(),
            vec![(0, 1), (0, 2), (0, 3), (1, 1), (2, 1), (2, 2), (2, 3)]
        );
    }

    #[test]
    fn anchored_rejects_a_missing_cell() {
        let table = Table::new_jagged(vec![4, 2, 4]).unwrap();

        assert!(Selection::anchored(&table.cursor, (1, 2)).is_err());
        assert!(Selection::anchored(&table.cursor, (3, 0)).is_err());
        assert_eq!(table.cursor.current(), (0, 0));
    }

    #[test]
    fn with_focus_keeps_the_anchor_when_the_cursor_moves() {
        let mut table = Table::new(3, 4).unwrap();
        let selection = Selection::at_cursor(&table.cursor);

        table.cursor.shift_line(2);
        table.cursor.shift_column(3);
        assert_eq!(selection.focus(), (0, 0));

        let selection = selection.with_focus(table.cursor.current());
        assert_eq!(selection.anchor(), (0, 0));
        assert_eq!(selection.focus(), (2, 3));
        assert_eq!(selection.cells(&[4, 4, 4]).count(), 12);
    }
}
