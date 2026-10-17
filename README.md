# rpos

[![Crates.io](https://img.shields.io/crates/v/rpos.svg)](https://crates.io/crates/rpos)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A cursor manager on table for Rust.

## Overview

`rpos` manages a 0-indexed cursor on a 2D table — rectangular or jagged — with configurable edge behavior. It is intended as a small, reusable building block for TUIs, menus, grids, and other 2D selection UIs.

- **Wrap / clamp** at table edges (`WrapMode`)
- **Jagged rows** with per-row widths
- **Vertical policies** for shorter rows: clamp column, skip rows missing the column, or remember a preferred column
- **Horizontal policies**: stay within a row, or continue across rows
- **Multi-step movement** (`shift_line` / `shift_column`) that stops at the edge instead of wrapping
- **Jump helpers**: `home` / `end` / `row_start` / `row_end`
- **Linear index**: row-major `index` / `set_index`
- **Resize** without recreating the table
- **Selection** as its own rectangle between an anchor cell and the cursor, clipped to jagged rows

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rpos = "0.4.0"
```

## Usage

### Creating a Table with Cursor

```rust
use rpos::table::Table;

fn main() {
    // Create a 3x4 table (height=3, width=4)
    let mut table = Table::new(3, 4).unwrap();

    // Get current cursor position (0-indexed)
    assert_eq!(table.cursor.current(), (0, 0));
}
```

### Moving the Cursor

```rust
use rpos::table::Table;

fn main() {
    let mut table = Table::new(3, 4).unwrap();

    table.cursor.down();
    assert_eq!(table.cursor.current(), (1, 0));

    table.cursor.right();
    assert_eq!(table.cursor.current(), (1, 1));

    table.cursor.up();
    assert_eq!(table.cursor.current(), (0, 1));

    table.cursor.left();
    assert_eq!(table.cursor.current(), (0, 0));
}
```

`shift_line` and `shift_column` repeat that step. Positive values move down and right. They stop at the edge instead of wrapping, so a count larger than the table does not spin:

```rust
use rpos::table::Table;

fn main() {
    let mut table = Table::new(5, 4).unwrap();

    table.cursor.shift_line(2);
    table.cursor.shift_column(3);
    assert_eq!(table.cursor.current(), (2, 3));

    table.cursor.shift_line(10);
    assert_eq!(table.cursor.current(), (4, 3));
}
```

### Setting Cursor Position

```rust
use rpos::table::Table;

fn main() {
    let mut table = Table::new(3, 4).unwrap();

    table.cursor.set(2, 3).unwrap();
    assert_eq!(table.cursor.current(), (2, 3));

    table.cursor.set_line(1).unwrap();
    assert_eq!(table.cursor.current(), (1, 3));

    table.cursor.set_column(0).unwrap();
    assert_eq!(table.cursor.current(), (1, 0));
}
```

### Wraparound and Clamp

Wrap mode (default) wraps at edges. Clamp mode stops at edges:

```rust
use rpos::{table::Table, WrapMode};

fn main() {
    let mut wrapped = Table::new(3, 4).unwrap();
    wrapped.cursor.up();
    wrapped.cursor.left();
    assert_eq!(wrapped.cursor.current(), (2, 3));

    let mut clamped = Table::new(3, 4).unwrap().wrap_mode(WrapMode::Clamp);
    clamped.cursor.set(0, 0).unwrap();
    clamped.cursor.up();
    clamped.cursor.left();
    assert_eq!(clamped.cursor.current(), (0, 0));
}
```

### Jagged (Non-Uniform) Tables

By default (`VerticalPolicy::ClampColumn`), moving onto a shorter row clamps the column:

```rust
use rpos::table::Table;

fn main() {
    // Row 0: 4 columns, Row 1: 2 columns, Row 2: 3 columns
    let mut table = Table::new_jagged(vec![4, 2, 3]).unwrap();

    table.cursor.set(0, 3).unwrap();
    table.cursor.down();
    assert_eq!(table.cursor.current(), (1, 1)); // clamped
}
```

`VerticalPolicy::SkipMissing` keeps the column and skips rows that do not have it:

```rust
use rpos::{table::Table, VerticalPolicy};

fn main() {
    let mut table = Table::new_jagged(vec![4, 2, 4])
        .unwrap()
        .vertical_policy(VerticalPolicy::SkipMissing);

    table.cursor.set(0, 3).unwrap();
    table.cursor.down(); // skip row 1
    assert_eq!(table.cursor.current(), (2, 3));
}
```

`VerticalPolicy::StickyColumn` remembers the column from horizontal movement and from explicit column changes. Up and down land on the last cell of a shorter row, then restore the preferred column when the row is wide enough:

```rust
use rpos::{table::Table, VerticalPolicy};

fn main() {
    let mut table = Table::new_jagged(vec![4, 2, 4])
        .unwrap()
        .vertical_policy(VerticalPolicy::StickyColumn);

    table.cursor.set(0, 3).unwrap();
    table.cursor.down();
    assert_eq!(table.cursor.current(), (1, 1)); // last cell of the short row
    table.cursor.down();
    assert_eq!(table.cursor.current(), (2, 3)); // preferred column restored
}
```

### Across-Row Horizontal Movement

By default, left/right stay inside the current row. `HorizontalPolicy::AcrossRows` continues onto the next/previous row at the edges (useful for reading order over a flat list laid out as a grid):

```rust
use rpos::{table::Table, HorizontalPolicy};

fn main() {
    let mut table = Table::new(2, 2)
        .unwrap()
        .horizontal_policy(HorizontalPolicy::AcrossRows);

    table.cursor.set(0, 1).unwrap();
    table.cursor.right();
    assert_eq!(table.cursor.current(), (1, 0));
}
```

### Jump Helpers and Linear Index

```rust
use rpos::table::Table;

fn main() {
    let mut table = Table::new(3, 4).unwrap();

    table.cursor.end();
    assert_eq!(table.cursor.current(), (2, 3));
    assert_eq!(table.cursor.index(), 11);

    table.cursor.set_index(6).unwrap();
    assert_eq!(table.cursor.current(), (1, 2));

    table.cursor.row_start();
    assert_eq!(table.cursor.current(), (1, 0));

    table.cursor.home();
    assert_eq!(table.cursor.current(), (0, 0));
}
```

### Resizing

```rust
use rpos::table::Table;

fn main() {
    let mut table = Table::new(3, 4).unwrap();
    table.cursor.set(2, 3).unwrap();

    // Rectangular resize
    table.set_size(2, 2).unwrap();
    assert_eq!(table.cursor.current(), (1, 1)); // clamped

    // Jagged resize (also via cursor — table dimensions stay in sync)
    table.set_widths(vec![3, 1, 2]).unwrap();
    assert_eq!(table.height(), 3);
    assert_eq!(table.width(), 3); // max row width

    table.cursor.set_column_sizes(vec![2, 2]).unwrap();
    assert_eq!(table.height(), 2);
    assert_eq!(table.width(), 2);
}
```

### Selection

`Selection` is separate from the cursor. It is the inclusive rectangle between an anchor and a focus (the cursor's current cell). On a jagged table, cells past a row's width are omitted:

```rust
use rpos::{table::Table, Selection};

fn main() {
    let mut table = Table::new_jagged(vec![4, 2, 4]).unwrap();
    table.cursor.set(2, 3).unwrap();

    let selection = Selection::anchored(&table.cursor, (0, 1)).unwrap();
    assert_eq!(
        selection.cells(table.cursor.column_sizes()).collect::<Vec<_>>(),
        vec![
            (0, 1),
            (0, 2),
            (0, 3),
            (1, 1), // row 1 has no column 2 or 3
            (2, 1),
            (2, 2),
            (2, 3),
        ]
    );
}
```

## API Reference

### Table

| Method | Description |
|--------|-------------|
| `Table::new(height, width)` | Creates a rectangular table |
| `Table::new_jagged(widths)` | Creates a jagged table |
| `table.height()` / `table.width()` | Row count / max row width (derived from cursor) |
| `table.wrap_mode(mode)` | Builder: set wrap mode |
| `table.vertical_policy(policy)` | Builder: set vertical policy |
| `table.horizontal_policy(policy)` | Builder: set horizontal policy |
| `table.set_size(height, width)` | Resize to a rectangle (clamps cursor) |
| `table.set_widths(widths)` | Resize to jagged widths (clamps cursor) |

### WrapMode

| Variant | Description |
|---------|-------------|
| `WrapMode::Wrap` | Wrap at table / row edges |
| `WrapMode::Clamp` | Stop at table / row edges |

### VerticalPolicy

| Variant | Description |
|---------|-------------|
| `VerticalPolicy::ClampColumn` | On up/down, move to adjacent row and clamp column (default) |
| `VerticalPolicy::SkipMissing` | On up/down, keep column and skip rows that lack it |
| `VerticalPolicy::StickyColumn` | On up/down, keep a preferred column: land on the last cell of a shorter row, restore it on a wider row |

### HorizontalPolicy

| Variant | Description |
|---------|-------------|
| `HorizontalPolicy::WithinRow` | left/right stay in the current row (default) |
| `HorizontalPolicy::AcrossRows` | At row edges, continue to the previous/next row |

### Cursor

| Method | Description |
|--------|-------------|
| `current()` / `line()` / `column()` | Current position |
| `line_count()` / `column_sizes()` | Table shape |
| `set` / `set_line` / `set_column` | Absolute positioning |
| `index` / `set_index` / `cell_count` | Row-major linear index |
| `set_column_sizes` | Replace per-row widths (clamps cursor) |
| `wrap_mode` / `set_wrap_mode` | Edge wrap vs clamp |
| `vertical_policy` / `set_vertical_policy` | Jagged vertical behavior |
| `preferred_column` | Column remembered by `StickyColumn` |
| `horizontal_policy` / `set_horizontal_policy` | Horizontal row-crossing behavior |
| `up` / `down` / `left` / `right` | Directional movement |
| `shift_line` / `shift_column` | Repeat that step, stopping at the edge |
| `home` / `end` / `row_start` / `row_end` | Jump helpers |

### Selection

| Method | Description |
|--------|-------------|
| `Selection::new(anchor, focus)` | Rectangle between two cells |
| `Selection::at_cursor(cursor)` | Anchor and focus on the cursor's current cell |
| `Selection::anchored(cursor, anchor)` | Anchor on a cell of the cursor's table, focus at the cursor |
| `anchor` / `focus` | The two ends |
| `with_focus(focus)` | Move the focus, keeping the anchor |
| `lines` / `columns` | Inclusive normalized bounds |
| `contains(column_sizes, position)` | Cell is inside the rectangle and exists on that row |
| `cells(column_sizes)` | Existing cells in row-major order |

## License

MIT License - see [LICENSE](LICENSE) for details.
