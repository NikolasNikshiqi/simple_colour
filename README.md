
# simple_colour 🎨

`simple_colour` is a lightweight, zero-dependency Rust library that provides a convenient Trait-based approach to styling terminal output.

## Features

- `Easy Integration:`: Just import the `Colour` trait and use methods directly on `&str` or `String`.
- `Chainable:` Combine styles, foregrounds, and backgrounds easily.
- `TrueColour Support:` Access the full 24-bit colour range using RGB or 8-bit (256-colour) palettes.
- `Specialty Effects:` Includes a built-in `rainbow()` method for easy text effects.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
simple_colour = "1.0.4"

```

## Usage

```rust
use simple_colour::Colour;

fn main() {
    // Basic Styling
    println!("{}", "Hello World Italic".italic());
    println!("{}", "Hello World Red".red());

    // Combining styles
    println!("{}", "Hello World Bold Red".red().bold());

    // Bright Colours
    println!("{}", "Hello World Bright Blue".bright_blue());

    // 8-bit Truecolour (0-255)
    println!("{}", "Yellow-Green".truecolour(192));
    println!("{}", "Yellow-Green BG".bg_truecolour(192));

    // 24-bit RGB Truecolour
    println!("{}", "Cyan Text".truecolour_rgb(0, 170, 170));
    println!("{}", "Cyan Background".bg_truecolour_rgb(0, 170, 170));

    // Rainbow Effect
    println!("{}", "Full spectrum rainbow text!".rainbow());
    
    // Reset
    let str = "Hello world".red().bold();
    let str = str.reset();
    println!("{}",str);
}

```

## Available Methods

### Styles & Effects

| Method | Description |
| --- | --- |
| `.bold()` | Makes text **bold** |
| `.italic()` | Makes text *italic* |
| `.rainbow()` | Cycles through colours for each character |

### Foreground Colours

* **Standard**: `.black()`, `.red()`, `.green()`, `.yellow()`, `.blue()`, `.magenta()`, `.cyan()`, `.white()`
* **Bright**: `.bright_black()`, `.bright_red()`, `.bright_green()`, `.bright_yellow()`, `.bright_blue()`, `.bright_magenta()`, `.bright_cyan()`, `.bright_white()`
* **Special**: `.orange()`, `.indigo()`, `.violet()`

### Background Colours

* **Standard**: `.bg_black()`, `.bg_red()`, `.bg_green()`, `.bg_yellow()`, `.bg_blue()`, `.bg_magenta()`, `.bg_cyan()`, `.bg_white()`
* **Bright**: `.bg_bright_black()`, `.bg_bright_red()`, `.bg_bright_green()`, `.bg_bright_yellow()`, `.bg_bright_blue()`, `.bg_bright_magenta()`, `.bg_bright_cyan()`, `.bg_bright_white()`

### Custom Palette (TrueColour)

* `.truecolour(u8)` / `.bg_truecolour(u8)`: Sets colour using 8-bit ANSI codes (0-255).
* `.truecolour_rgb(r, g, b)` / `.bg_truecolour_rgb(r, g, b)`: Sets colour using 24-bit RGB values.

## License

This project is licensed under the MIT License.