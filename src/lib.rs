//! # colour
//! `colour` is a collection of functions providing a more conveniet
//! process to change output styles/colouring in the terminal
//!
//! It accepts both &str and String
//!
//! Use the [`Colour`] trait to style your strings.
//! ## Usage
//!
//! Add the trait to your scope:
//! ```rust
//! use colour::Colour;
//!
//! println!("{}", "Hello World Italic".italic()); //Style
//! println!("{}", "Hello World Red".red()); //Colour
//! println!("{}", "Hello World Bold Red".red().bold()); // Can combine colours and styles
//! println!("{}", "Hello World Bright Blue".bright_blue());
//! println!("{}", "Hello World".truecolour(192)); //Yellowish green (Truecolour 8-bit format)
//! println!("{}", "Hello World".truecolour_rgb(0, 170, 170)); //Cyan (Truecolour rgb format)
//! println!("{}", "Hello World".bg_truecolour(192)); //Yellowish green background (Truecolour 8-bit format)
//! println!("{}", "Hello World".bg_truecolour_rgb(0, 170, 170)); //Cyan background (Truecolour rgb format)
//! println!("{}", "Hello Red Background".bg_red()); //Red background
//! println!("{}", "Hello Rainbow".rainbow()); // Produces rainbow text
//! ```
//!
//! See the [`Colour`] trait for all the methods. All methods return a String.
use std::{borrow::Borrow, fmt::Display};

pub(crate) const RESET: &str = "\x1b[0m";
pub(crate) const RESET_BOLD: &str = "\x1b[22m";
pub(crate) const RESET_ITALIC: &str = "\x1b[23m";

// Style
pub(crate) const BOLD: &str = "\x1b[1m";
pub(crate) const ITALIC: &str = "\x1b[3m";

// Standard Colours
pub(crate) const BLACK: &str = "\x1b[30m";
pub(crate) const RED: &str = "\x1b[31m";
pub(crate) const GREEN: &str = "\x1b[32m";
pub(crate) const YELLOW: &str = "\x1b[33m";
pub(crate) const BLUE: &str = "\x1b[34m";
pub(crate) const MAGENTA: &str = "\x1b[35m";
pub(crate) const CYAN: &str = "\x1b[36m";
pub(crate) const WHITE: &str = "\x1b[37m";

// Bright Colours
pub(crate) const BRIGHT_BLACK: &str = "\x1b[90m";
pub(crate) const BRIGHT_RED: &str = "\x1b[91m";
pub(crate) const BRIGHT_GREEN: &str = "\x1b[92m";
pub(crate) const BRIGHT_YELLOW: &str = "\x1b[93m";
pub(crate) const BRIGHT_BLUE: &str = "\x1b[94m";
pub(crate) const BRIGHT_MAGENTA: &str = "\x1b[95m";
pub(crate) const BRIGHT_CYAN: &str = "\x1b[96m";
pub(crate) const BRIGHT_WHITE: &str = "\x1b[97m";

// Background colours
pub(crate) const BG_BLACK: &str = "\x1b[40m";
pub(crate) const BG_RED: &str = "\x1b[41m";
pub(crate) const BG_GREEN: &str = "\x1b[42m";
pub(crate) const BG_YELLOW: &str = "\x1b[43m";
pub(crate) const BG_BLUE: &str = "\x1b[44m";
pub(crate) const BG_MAGENTA: &str = "\x1b[45m";
pub(crate) const BG_CYAN: &str = "\x1b[46m";
pub(crate) const BG_WHITE: &str = "\x1b[47m";

pub(crate) const BG_BRIGHT_BLACK: &str = "\x1b[100m";
pub(crate) const BG_BRIGHT_RED: &str = "\x1b[101m";
pub(crate) const BG_BRIGHT_GREEN: &str = "\x1b[102m";
pub(crate) const BG_BRIGHT_YELLOW: &str = "\x1b[103m";
pub(crate) const BG_BRIGHT_BLUE: &str = "\x1b[104m";
pub(crate) const BG_BRIGHT_MAGENTA: &str = "\x1b[105m";
pub(crate) const BG_BRIGHT_CYAN: &str = "\x1b[106m";
pub(crate) const BG_BRIGHT_WHITE: &str = "\x1b[107m";

// True Colours
pub(crate) const ORANGE: &str = "\x1b[38;5;208m";
pub(crate) const INDIGO: &str = "\x1b[38;5;63m";
pub(crate) const VIOLET: &str = "\x1b[38;5;129m";

#[allow(dead_code)]

pub trait Colour {
    // --- Style ---

    /// Sets the text style to **bold**.
    /// ```
    /// # use colour::Colour;
    /// println!("{}", "Bold Text".bold());
    /// ```
    fn bold(&self) -> String;

    /// Sets the text style to *italic*.
    /// ```
    /// # use colour::Colour;
    /// println!("{}", "Italic Text".italic());
    /// ```
    fn italic(&self) -> String;

    // --- Standard Foreground ---

    /// Changes the text colour to black.
    fn black(&self) -> String;

    /// Changes the text colour to red.
    fn red(&self) -> String;

    /// Changes the text colour to green.
    fn green(&self) -> String;

    /// Changes the text colour to yellow.
    fn yellow(&self) -> String;

    /// Changes the text colour to blue.
    fn blue(&self) -> String;

    /// Changes the text colour to magenta.
    fn magenta(&self) -> String;

    /// Changes the text colour to cyan.
    fn cyan(&self) -> String;

    /// Changes the text colour to white.
    fn white(&self) -> String;

    // --- Bright Foreground ---

    /// Changes the text colour to bright black (gray).
    fn bright_black(&self) -> String;

    /// Changes the text colour to bright red.
    fn bright_red(&self) -> String;

    /// Changes the text colour to bright green.
    fn bright_green(&self) -> String;

    /// Changes the text colour to bright yellow.
    fn bright_yellow(&self) -> String;

    /// Changes the text colour to bright blue.
    fn bright_blue(&self) -> String;

    /// Changes the text colour to bright magenta.
    fn bright_magenta(&self) -> String;

    /// Changes the text colour to bright cyan.
    fn bright_cyan(&self) -> String;

    /// Changes the text colour to bright white.
    fn bright_white(&self) -> String;

    // --- Standard Background ---

    /// Changes the background colour to black.
    fn bg_black(&self) -> String;

    /// Changes the background colour to red.
    fn bg_red(&self) -> String;

    /// Changes the background colour to green.
    fn bg_green(&self) -> String;

    /// Changes the background colour to yellow.
    fn bg_yellow(&self) -> String;

    /// Changes the background colour to blue.
    fn bg_blue(&self) -> String;

    /// Changes the background colour to magenta.
    fn bg_magenta(&self) -> String;

    /// Changes the background colour to cyan.
    fn bg_cyan(&self) -> String;

    /// Changes the background colour to white.
    fn bg_white(&self) -> String;

    // --- Bright Background ---

    /// Changes the background colour to bright black.
    fn bg_bright_black(&self) -> String;

    /// Changes the background colour to bright red.
    fn bg_bright_red(&self) -> String;

    /// Changes the background colour to bright green.
    fn bg_bright_green(&self) -> String;

    /// Changes the background colour to bright yellow.
    fn bg_bright_yellow(&self) -> String;

    /// Changes the background colour to bright blue.
    fn bg_bright_blue(&self) -> String;

    /// Changes the background colour to bright magenta.
    fn bg_bright_magenta(&self) -> String;

    /// Changes the background colour to bright cyan.
    fn bg_bright_cyan(&self) -> String;

    /// Changes the background colour to bright white.
    fn bg_bright_white(&self) -> String;

    // --- True Colours ---

    /// Changes the text colour to orange.
    fn orange(&self) -> String;

    /// Changes the text colour to indigo.
    fn indigo(&self) -> String;

    /// Changes the text colour to violet.
    fn violet(&self) -> String;

    // --- Custom Colours ---

    /// Applies a custom colour using RGB values.
    /// ```
    /// # use colour::Colour;
    /// "Custom".truecolour_rgb(100, 200, 50);
    /// ```
    fn truecolour_rgb(&self, r: u8, g: u8, b: u8) -> String;

    /// Applies a custom colour using an 8-bit ANSI colour code (0-255).
    /// ```
    /// # use colour::Colour;
    /// "ANSI 150".truecolour(150);
    /// ```
    fn truecolour(&self, code: u8) -> String;

    /// Applies a custom background-colour using RGB values.
    /// ```
    /// # use colour::Colour;
    /// "Custom".bg_truecolour_rgb(100, 200, 50);
    /// ```
    fn bg_truecolour_rgb(&self, r: u8, g: u8, b: u8) -> String;

    /// Applies a custom background-colour using an 8-bit ANSI colour code (0-255).
    /// ```
    /// # use colour::Colour;
    /// "ANSI 150".bg_truecolour(150);
    /// ```
    fn bg_truecolour(&self, code: u8) -> String;

    /// Cycles through a spectrum of colours for each character in the string.
    /// ```
    /// # use colour::Colour;
    /// println!("{}", "Rainbow!".rainbow());
    /// ```
    fn rainbow(&self) -> String;
}
impl<T> Colour for T
where
    T: Borrow<str> + Display,
{
    fn bold(&self) -> String {
        format!("{}{}{}", BOLD, self, RESET_BOLD)
    }

    fn italic(&self) -> String {
        format!("{}{}{}", ITALIC, self, RESET_ITALIC)
    }

    fn black(&self) -> String {
        format!("{}{}{}", BLACK, self, RESET)
    }
    fn red(&self) -> String {
        format!("{}{}{}", RED, self, RESET)
    }

    fn green(&self) -> String {
        format!("{}{}{}", GREEN, self, RESET)
    }

    fn yellow(&self) -> String {
        format!("{}{}{}", YELLOW, self, RESET)
    }

    fn blue(&self) -> String {
        format!("{}{}{}", BLUE, self, RESET)
    }

    fn magenta(&self) -> String {
        format!("{}{}{}", MAGENTA, self, RESET)
    }

    fn cyan(&self) -> String {
        format!("{}{}{}", CYAN, self, RESET)
    }

    fn white(&self) -> String {
        format!("{}{}{}", WHITE, self, RESET)
    }

    fn bright_black(&self) -> String {
        format!("{}{}{}", BRIGHT_BLACK, self, RESET)
    }

    fn bright_red(&self) -> String {
        format!("{}{}{}", BRIGHT_RED, self, RESET)
    }

    fn bright_green(&self) -> String {
        format!("{}{}{}", BRIGHT_GREEN, self, RESET)
    }

    fn bright_yellow(&self) -> String {
        format!("{}{}{}", BRIGHT_YELLOW, self, RESET)
    }

    fn bright_blue(&self) -> String {
        format!("{}{}{}", BRIGHT_BLUE, self, RESET)
    }

    fn bright_magenta(&self) -> String {
        format!("{}{}{}", BRIGHT_MAGENTA, self, RESET)
    }

    fn bright_cyan(&self) -> String {
        format!("{}{}{}", BRIGHT_CYAN, self, RESET)
    }

    fn bright_white(&self) -> String {
        format!("{}{}{}", BRIGHT_WHITE, self, RESET)
    }

    // --- Background Standard ---
    fn bg_black(&self) -> String {
        format!("{}{}{}", BG_BLACK, self, RESET)
    }
    fn bg_red(&self) -> String {
        format!("{}{}{}", BG_RED, self, RESET)
    }
    fn bg_green(&self) -> String {
        format!("{}{}{}", BG_GREEN, self, RESET)
    }
    fn bg_yellow(&self) -> String {
        format!("{}{}{}", BG_YELLOW, self, RESET)
    }
    fn bg_blue(&self) -> String {
        format!("{}{}{}", BG_BLUE, self, RESET)
    }
    fn bg_magenta(&self) -> String {
        format!("{}{}{}", BG_MAGENTA, self, RESET)
    }
    fn bg_cyan(&self) -> String {
        format!("{}{}{}", BG_CYAN, self, RESET)
    }
    fn bg_white(&self) -> String {
        format!("{}{}{}", BG_WHITE, self, RESET)
    }

    // --- Background Bright ---
    fn bg_bright_black(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_BLACK, self, RESET)
    }
    fn bg_bright_red(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_RED, self, RESET)
    }
    fn bg_bright_green(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_GREEN, self, RESET)
    }
    fn bg_bright_yellow(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_YELLOW, self, RESET)
    }
    fn bg_bright_blue(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_BLUE, self, RESET)
    }
    fn bg_bright_magenta(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_MAGENTA, self, RESET)
    }
    fn bg_bright_cyan(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_CYAN, self, RESET)
    }
    fn bg_bright_white(&self) -> String {
        format!("{}{}{}", BG_BRIGHT_WHITE, self, RESET)
    }

    // true colours

    fn orange(&self) -> String {
        format!("{}{}{}", ORANGE, self, RESET)
    }

    fn indigo(&self) -> String {
        format!("{}{}{}", INDIGO, self, RESET)
    }

    fn violet(&self) -> String {
        format!("{}{}{}", VIOLET, self, RESET)
    }

    fn truecolour_rgb(&self, r: u8, g: u8, b: u8) -> String {
        let true_colour: String = format!("{};{};{};{}m", "\x1b[38;2", r, g, b);
        format!("{}{}{}", true_colour, self, RESET)
    }

    fn truecolour(&self, code: u8) -> String {
        let true_colour: String = format!("{};{}m", "\x1b[38;5", code);
        format!("{}{}{}", true_colour, self, RESET)
    }

    fn bg_truecolour_rgb(&self, r: u8, g: u8, b: u8) -> String {
        let bg_true_colour: String = format!("{};{};{};{}m", "\x1b[48;2", r, g, b);
        format!("{}{}{}", bg_true_colour, self, RESET)
    }

    fn bg_truecolour(&self, code: u8) -> String {
        let bg_true_colour: String = format!("{};{}m", "\x1b[48;5", code);
        format!("{}{}{}", bg_true_colour, self, RESET)
    }

    fn rainbow(&self) -> String {
        let rainbow_colours = vec![
            BRIGHT_RED,
            ORANGE,
            BRIGHT_YELLOW,
            BRIGHT_GREEN,
            BRIGHT_BLUE,
            INDIGO,
            VIOLET,
        ];
        self.borrow()
            .chars()
            .enumerate()
            .map(|(i, c)| {
                let colour = rainbow_colours[i % rainbow_colours.len()];
                format!("{}{}{}", colour, c, RESET)
            })
            .collect()
    }
}
