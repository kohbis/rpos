# rpos

[![Crates.io](https://img.shields.io/crates/v/rpos.svg)](https://crates.io/crates/rpos)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A cursor manager on table for Rust.

## Overview

`rpos` manages a 0-indexed cursor on a 2D table — rectangular or jagged — with configurable edge behavior. It is intended as a small, reusable building block for TUIs, menus, grids, and other 2D selection UIs.

- **Wrap / clamp** at table edges (`WrapMode`)
- **Jagged rows** with per-row widths
- **Vertical policies** for shorter rows: clamp column, or skip rows missing the column
- **Horizontal policies**: stay within a row, or continue across rows
- **Jump helpers**: `home` / `end` / `row_start` / `row_end`
- **Linear index**: row-major `index` / `set_index`
- **Resize** without recreating the table

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
| `horizontal_policy` / `set_horizontal_policy` | Horizontal row-crossing behavior |
| `up` / `down` / `left` / `right` | Directional movement |
| `home` / `end` / `row_start` / `row_end` | Jump helpers |

## License

MIT License - see [LICENSE](LICENSE) for details.
