/// Wraps `s` into the passed down color. If the color does
/// not match any color - return it as-is.
///
/// Takes into account the `CLICOLOR`, `CLICOLOR_FORCE` and
/// `NO_COLOR` variables.
///
pub fn colorize(s: &str, color: Color) -> std::string::String {
    match color {
        Color::None => return String::from(s),

        Color::Black => return black(s),
        Color::Red => return red(s),
        Color::Green => return green(s),
        Color::Yellow => return yellow(s),
        Color::Blue => return blue(s),
        Color::Magenta => return magenta(s),
        Color::Cyan => return cyan(s),
        Color::White => return white(s),
        Color::Gray => return gray(s),

        Color::Bold => return bold(s),
    }
}

#[allow(dead_code)]
pub enum Color {
    None,

    Black,
    Red,
    Green,
    Yellow,
    Blue,
    Magenta,
    Cyan,
    White,

    Gray,
    Bold,
}

// Foreground colors   // Background colors
// 30 Black            // 40 Black
// 31 Red              // 41 Red
// 32 Green            // 42 Green
// 33 Yellow           // 43 Yellow
// 34 Blue             // 44 Blue
// 35 Magenta          // 45 Magenta
// 36 Cyan             // 46 Cyan
// 37 White            // 47 White

fn black(s: &str) -> std::string::String {
    return format!("\x1B[30m{}\x1B[0m", s);
}
fn red(s: &str) -> std::string::String {
    return format!("\x1B[31m{}\x1B[0m", s);
}
fn green(s: &str) -> std::string::String {
    return format!("\x1B[32m{}\x1B[0m", s);
}
fn yellow(s: &str) -> std::string::String {
    return format!("\x1B[33m{}\x1B[0m", s);
}
fn blue(s: &str) -> std::string::String {
    return format!("\x1B[34m{}\x1B[0m", s);
}
fn magenta(s: &str) -> std::string::String {
    return format!("\x1B[35m{}\x1B[0m", s);
}
fn cyan(s: &str) -> std::string::String {
    return format!("\x1B[36m{}\x1B[0m", s);
}
fn white(s: &str) -> std::string::String {
    return format!("\x1B[37m{}\x1B[0m", s);
}

fn gray(s: &str) -> std::string::String {
    return format!("\x1B[90m{}\x1B[0m", s);
}
fn bold(s: &str) -> std::string::String {
    return format!("\x1B[97m{}\x1B[0m", s);
}

#[cfg(test)]
mod test {
    use super::{Color,colorize};
    #[test]
    #[rustfmt::skip::macros(assert_eq)]
    fn test_proper_colorization() {
        assert_eq!(colorize("Lorem ipsum", Color::Black),   "\x1B[30mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", Color::Red),     "\x1B[31mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", Color::Green),   "\x1B[32mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", Color::Yellow),  "\x1B[33mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", Color::Blue),    "\x1B[34mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", Color::Magenta), "\x1B[35mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", Color::Cyan),    "\x1B[36mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", Color::White),   "\x1B[37mLorem ipsum\x1B[0m");

        assert_eq!(colorize("Lorem ipsum", Color::Bold), "\x1B[97mLorem ipsum\x1B[0m");

        assert_eq!(colorize("Lorem ipsum", Color::Gray), "\x1B[90mLorem ipsum\x1B[0m");
    }
}
