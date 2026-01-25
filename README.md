# rpos

[![Crates.io](https://img.shields.io/crates/v/rpos.svg)](https://crates.io/crates/rpos)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A cursor manager on table for Rust.

## Overview

`rpos` provides a simple and intuitive way to manage cursor position on a 2D table with bounds checking. The cursor can navigate within the table using directional movements (up, down, left, right) or by setting specific positions.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
rpos = "0.3.1"
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

### Boundary Safety

The cursor automatically stays within table bounds:

```rust
use rpos::table::Table;

fn main() {
    let mut table = Table::new(3, 4).unwrap();

    // Cursor won't go beyond boundaries
    table.cursor.up(); // Already at top, stays at (0, 0)
    table.cursor.left(); // Already at left edge, stays at (0, 0)
    assert_eq!(table.cursor.current(), (0, 0));

    // Setting out-of-bounds position returns an error
    let result = table.cursor.set(10, 10);
    assert!(result.is_err());
}
```

## API Reference

### Table

| Method | Description |
|--------|-------------|
| `Table::new(height, width)` | Creates a new table with specified dimensions |

### Cursor

| Method | Description |
|--------|-------------|
| `current()` | Returns current position as `(line, column)` |
| `set(line, column)` | Sets cursor to specific position |
| `set_line(line)` | Sets cursor line (row) |
| `set_column(column)` | Sets cursor column |
| `up()` | Moves cursor up one position |
| `down()` | Moves cursor down one position |
| `left()` | Moves cursor left one position |
| `right()` | Moves cursor right one position |

## License

MIT License - see [LICENSE](LICENSE) for details.
