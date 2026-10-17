# rpos

[![Crates.io](https://img.shields.io/crates/v/rpos.svg)](https://crates.io/crates/rpos)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A cursor manager on table for Rust.

## Overview

`rpos` provides a simple and intuitive way to manage cursor position on a 2D table. The cursor can navigate within the table using directional movements (up, down, left, right) or by setting specific positions. By default, movement wraps at edges, and clamp mode can be enabled when needed. Tables support both uniform (rectangular) and jagged (non-uniform) rows where each row can have a different number of columns. For jagged tables, vertical movement can either clamp the column or skip rows that lack the current column.

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

    // Move cursor down
    table.cursor.down();
    assert_eq!(table.cursor.current(), (1, 0));

    // Move cursor right
    table.cursor.right();
    assert_eq!(table.cursor.current(), (1, 1));

    // Move cursor up
    table.cursor.up();
    assert_eq!(table.cursor.current(), (0, 1));

    // Move cursor left
    table.cursor.left();
    assert_eq!(table.cursor.current(), (0, 0));
}
```

### Setting Cursor Position

```rust
use rpos::table::Table;

fn main() {
    let mut table = Table::new(3, 4).unwrap();

    // Set cursor to specific position (line, column)
    table.cursor.set(2, 3).unwrap();
    assert_eq!(table.cursor.current(), (2, 3));

    // Set only the line
    table.cursor.set_line(1).unwrap();
    assert_eq!(table.cursor.current(), (1, 3));

    // Set only the column
    table.cursor.set_column(0).unwrap();
    assert_eq!(table.cursor.current(), (1, 0));
}
```

### Wraparound Movement

Wrap mode is the default and wraps around table edges when moving:

```rust
use rpos::table::Table;

fn main() {
    let mut table = Table::new(3, 4).unwrap();

    // Cursor wraps around edges
    table.cursor.up(); // From top row, wraps to bottom row
    table.cursor.left(); // From left edge, wraps to right edge
    assert_eq!(table.cursor.current(), (2, 3));

    // Setting out-of-bounds position returns an error
    let result = table.cursor.set(10, 10);
    assert!(result.is_err());
}
```

### Clamp Mode

Clamp mode keeps the cursor at the table edges:

```rust
use rpos::{table::Table, WrapMode};

fn main() {
    let mut table = Table::new(3, 4).unwrap().wrap_mode(WrapMode::Clamp);

    table.cursor.set(0, 0).unwrap();
    table.cursor.up();
    table.cursor.left();
    assert_eq!(table.cursor.current(), (0, 0));
}
```

### Jagged (Non-Uniform) Tables

Tables can have rows with different column counts. By default (`VerticalPolicy::ClampColumn`), moving to a shorter row clamps the column to the last valid position:

```rust
use rpos::table::Table;

fn main() {
    // Row 0: 4 columns, Row 1: 2 columns, Row 2: 3 columns
    let mut table = Table::new_jagged(vec![4, 2, 3]).unwrap();

    table.cursor.set(0, 3).unwrap(); // column 3 on row 0
    assert_eq!(table.cursor.current(), (0, 3));

    table.cursor.down(); // move to row 1 (only 2 columns)
    assert_eq!(table.cursor.current(), (1, 1)); // column clamped to 1

    table.cursor.down(); // move to row 2 (3 columns)
    assert_eq!(table.cursor.current(), (2, 1)); // column stays at 1

    // Left/right movement uses the current row's width
    table.cursor.set(1, 0).unwrap();
    table.cursor.right();
    assert_eq!(table.cursor.current(), (1, 1)); // row 1 max
    table.cursor.right(); // wraps within row 1
    assert_eq!(table.cursor.current(), (1, 0));
}
```

### Skip Missing Columns (Jagged)

`VerticalPolicy::SkipMissing` keeps the current column and skips rows that do not have it. This matches grid UIs where column identity matters (for example a channel selector laid out in columns):

```rust
use rpos::{table::Table, VerticalPolicy};

fn main() {
    // Row 0: 4 columns, Row 1: 2 columns, Row 2: 4 columns
    let mut table = Table::new_jagged(vec![4, 2, 4])
        .unwrap()
        .vertical_policy(VerticalPolicy::SkipMissing);

    table.cursor.set(0, 3).unwrap();
    table.cursor.down(); // skip row 1 (no column 3)
    assert_eq!(table.cursor.current(), (2, 3));
}
```


## API Reference

### Table

| Method | Description |
|--------|-------------|
| `Table::new(height, width)` | Creates a new table with specified dimensions |
| `Table::new_jagged(widths)` | Creates a jagged table with per-row column widths |
| `table.wrap_mode(mode)` | Sets wrap mode (builder pattern, chainable after `new` / `new_jagged`) |
| `table.vertical_policy(policy)` | Sets vertical policy for jagged up/down movement (builder pattern) |

### WrapMode

| Variant | Description |
|---------|-------------|
| `WrapMode::Wrap` | Wraps cursor movement at table edges |
| `WrapMode::Clamp` | Clamps cursor movement at table edges |

### VerticalPolicy

| Variant | Description |
|---------|-------------|
| `VerticalPolicy::ClampColumn` | On up/down, move to the adjacent row and clamp the column (default) |
| `VerticalPolicy::SkipMissing` | On up/down, keep the column and skip rows that lack it |

### Cursor

| Method | Description |
|--------|-------------|
| `current()` | Returns current position as `(line, column)` |
| `set(line, column)` | Sets cursor to specific position |
| `set_line(line)` | Sets cursor line (row), clamping column if needed |
| `set_column(column)` | Sets cursor column |
| `wrap_mode()` | Returns current wrap mode |
| `set_wrap_mode(wrap_mode)` | Sets wrap mode for movement |
| `vertical_policy()` | Returns current vertical policy |
| `set_vertical_policy(policy)` | Sets vertical policy for up/down movement |
| `up()` | Moves cursor up one position |
| `down()` | Moves cursor down one position |
| `left()` | Moves cursor left one position |
| `right()` | Moves cursor right one position |

## License

MIT License - see [LICENSE](LICENSE) for details.
