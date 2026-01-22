#[cfg(test)]
mod tests {
    use colour::*;

    #[test]
    fn test_styles() {
        assert_eq!("test".bold(), format!("{}{}{}", BOLD, "test", RESET_BOLD));
        assert_eq!(
            "test".italic(),
            format!("{}{}{}", ITALIC, "test",RESET_ITALIC)
        );
    }

    #[test]
    fn test_standard_colors() {
        assert_eq!("test".black(), format!("{}{}{}", BLACK, "test", RESET));
        assert_eq!("test".red(), format!("{}{}{}", RED, "test", RESET));
        assert_eq!("test".green(), format!("{}{}{}", GREEN, "test", RESET));
        assert_eq!("test".yellow(), format!("{}{}{}", YELLOW, "test", RESET));
        assert_eq!("test".blue(), format!("{}{}{}", BLUE, "test", RESET));
        assert_eq!("test".magenta(), format!("{}{}{}", MAGENTA, "test", RESET));
        assert_eq!("test".cyan(), format!("{}{}{}", CYAN, "test", RESET));
        assert_eq!("test".white(), format!("{}{}{}", WHITE, "test", RESET));
    }

    #[test]
    fn test_bright_colors() {
        assert_eq!(
            "hi".bright_black(),
            format!("{}{}{}", BRIGHT_BLACK, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_red(),
            format!("{}{}{}", BRIGHT_RED, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_green(),
            format!("{}{}{}", BRIGHT_GREEN, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_yellow(),
            format!("{}{}{}", BRIGHT_YELLOW, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_blue(),
            format!("{}{}{}", BRIGHT_BLUE, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_magenta(),
            format!("{}{}{}", BRIGHT_MAGENTA, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_cyan(),
            format!("{}{}{}", BRIGHT_CYAN, "hi", RESET)
        );
        assert_eq!(
            "hi".bright_white(),
            format!("{}{}{}", BRIGHT_WHITE, "hi", RESET)
        );
    }

    #[test]
    fn test_background_standard() {
        assert_eq!("bg".bg_black(),   format!("{}{}{}", BG_BLACK, "bg", RESET));
        assert_eq!("bg".bg_red(),     format!("{}{}{}", BG_RED, "bg", RESET));
        assert_eq!("bg".bg_green(),   format!("{}{}{}", BG_GREEN, "bg", RESET));
        assert_eq!("bg".bg_yellow(),  format!("{}{}{}", BG_YELLOW, "bg", RESET));
        assert_eq!("bg".bg_blue(),    format!("{}{}{}", BG_BLUE, "bg", RESET));
        assert_eq!("bg".bg_magenta(), format!("{}{}{}", BG_MAGENTA, "bg", RESET));
        assert_eq!("bg".bg_cyan(),    format!("{}{}{}", BG_CYAN, "bg", RESET));
        assert_eq!("bg".bg_white(),   format!("{}{}{}", BG_WHITE, "bg", RESET));
    }

    #[test]
    fn test_background_bright() {
        assert_eq!("bg".bg_bright_black(),   format!("{}{}{}", BG_BRIGHT_BLACK, "bg", RESET));
        assert_eq!("bg".bg_bright_red(),     format!("{}{}{}", BG_BRIGHT_RED, "bg", RESET));
        assert_eq!("bg".bg_bright_green(),   format!("{}{}{}", BG_BRIGHT_GREEN, "bg", RESET));
        assert_eq!("bg".bg_bright_yellow(),  format!("{}{}{}", BG_BRIGHT_YELLOW, "bg", RESET));
        assert_eq!("bg".bg_bright_blue(),    format!("{}{}{}", BG_BRIGHT_BLUE, "bg", RESET));
        assert_eq!("bg".bg_bright_magenta(), format!("{}{}{}", BG_BRIGHT_MAGENTA, "bg", RESET));
        assert_eq!("bg".bg_bright_cyan(),    format!("{}{}{}", BG_BRIGHT_CYAN, "bg", RESET));
        assert_eq!("bg".bg_bright_white(),   format!("{}{}{}", BG_BRIGHT_WHITE, "bg", RESET));
    }

    #[test]
    fn test_extended_colors() {
        assert_eq!("!".orange(), format!("{}{}{}", ORANGE, "!", RESET));
        assert_eq!("!".indigo(), format!("{}{}{}", INDIGO, "!", RESET));
        assert_eq!("!".violet(), format!("{}{}{}", VIOLET, "!", RESET));
    }

    #[test]
    fn test_truecolor_logic() {
        println!("{}","text".bg_truecolour(150));
        println!("{}","text".bg_truecolour_rgb(255, 100, 50));


        // Testing 256-color mode (\x1b[38;5;Nm)
        let custom_code = "text".truecolour(150);
        assert_eq!(custom_code, "\x1b[38;5;150mtext\x1b[0m");

        // Testing RGB mode (\x1b[38;2;R;G;Bm)
        let rgb_code = "text".truecolour_rgb(255, 100, 50);
        assert_eq!(rgb_code, "\x1b[38;2;255;100;50mtext\x1b[0m");

        let bg_custom_code = "text".bg_truecolour(150);
        assert_eq!(bg_custom_code, "\x1b[48;5;150mtext\x1b[0m");
        
        let bg_rgb_code = "text".bg_truecolour_rgb(255, 100, 50);
        assert_eq!(bg_custom_code, "\x1b[48;2;255;100;50mtext\x1b[0m");
    }

    #[test]
    fn test_rainbow() {
        let input = "ABC";
        let result = input.rainbow();

        // "A" should be Bright Red, "B" Orange, "C" Bright Yellow
        let expected = format!(
            "{}{}{}{}{}{}{}{}{}",
            BRIGHT_RED, "A", RESET, ORANGE, "B", RESET, BRIGHT_YELLOW, "C", RESET
        );
        assert_eq!(result, expected);
    }

    #[test]
    fn test_generic_support() {
        // Test that it works on an owned String as well as &str
        let owned = String::from("owned");
        assert_eq!(owned.red(), format!("{}{}{}", RED, "owned", RESET));
    }
}
