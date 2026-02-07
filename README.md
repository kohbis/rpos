# rpos

[![Crates.io](https://img.shields.io/crates/v/rpos.svg)](https://crates.io/crates/rpos)
[![License: MIT](https://img.shields.io/badge/License-MIT-yellow.svg)](https://opensource.org/licenses/MIT)

A cursor manager on table for Rust.

## Overview

`rpos` provides a simple and intuitive way to manage cursor position on a 2D table. The cursor can navigate within the table using directional movements (up, down, left, right) or by setting specific positions. By default, movement wraps at edges, and clamp mode can be enabled when needed.

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
    let mut table = Table::new(3, 4).unwrap();

    table.cursor.set(0, 0).unwrap();
    table.cursor.up();
    table.cursor.left();
    assert_eq!(table.cursor.current(), (0, 0));
}
```


## API Reference

### Table

| Method | Description |
|--------|-------------|
| `Table::new(height, width)` | Creates a new table with specified dimensions |
| `Table::new_with_wrap_mode(height, width, wrap_mode)` | Creates a new table with a specified wrap mode |

### WrapMode

| Variant | Description |
|---------|-------------|
| `WrapMode::Wrap` | Wraps cursor movement at table edges |
| `WrapMode::Clamp` | Clamps cursor movement at table edges |

### Cursor

| Method | Description |
|--------|-------------|
| `current()` | Returns current position as `(line, column)` |
| `set(line, column)` | Sets cursor to specific position |
| `set_line(line)` | Sets cursor line (row) |
| `set_column(column)` | Sets cursor column |
| `wrap_mode()` | Returns current wrap mode |
| `set_wrap_mode(wrap_mode)` | Sets wrap mode for movement |
| `up()` | Moves cursor up one position |
| `down()` | Moves cursor down one position |
| `left()` | Moves cursor left one position |
| `right()` | Moves cursor right one position |

## License

MIT License - see [LICENSE](LICENSE) for details.
