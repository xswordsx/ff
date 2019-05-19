/// Wraps `s` into the passed down color. If the color does
/// not match any color - return it as-is.
///
/// Takes into account the `CLICOLOR`, `CLICOLOR_FORCE` and
/// `NO_COLOR` variables.
///
pub fn colorize(s: &str, color: &str) -> std::string::String {
    if !should_color() {
        return String::from(s);
    }
    match color {
        "black" => return black(s),
        "red" => return red(s),
        "green" => return green(s),
        "yellow" => return yellow(s),
        "blue" => return blue(s),
        "magenta" => return magenta(s),
        "cyan" => return cyan(s),
        "white" => return white(s),
        "gray" => return gray(s),

        "bold" => return bold(s),

        _ => return String::from(s),
    }
}

// Implements https://bixense.com/clicolors/
fn clicolor() -> bool {
    let force_color = match std::env::var("CLICOLOR_FORCE") {
        Ok(val) => val != "0",
        Err(_) => false,
    };

    if force_color {
        return true;
    }

    if atty::is(atty::Stream::Stdout) {
        match std::env::var("CLICOLOR") {
            Ok(val) => return val != "0",
            Err(_) => return true,
        };
    }

    return false;
}

fn should_color() -> bool {
    match std::env::var("NO_COLOR") {
        // If set (regardless to what) -> don't color output.
        Ok(_) => return false,

        // Var is not present, bad unicode
        Err(_) => return clicolor(),
    }
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
    use super::*;

    #[test]
    fn color_on_clicolor() {
        std::env::set_var("CLICOLOR", "1");
        assert_eq!(colorize("test", "blue"), "\u{1b}[34mtest\u{1b}[0m");

        std::env::set_var("CLICOLOR", "0");
        assert_eq!(colorize("test", "blue"), "test");

        std::env::remove_var("CLICOLOR");
    }

    #[test]
    fn do_not_color_when_nocolor_set() {
        std::env::set_var("NO_COLOR", "0");
        assert_eq!(colorize("test", "green"), "test");

        std::env::set_var("NO_COLOR", "1");
        assert_eq!(colorize("test", "green"), "test");

        std::env::remove_var("NO_COLOR");
    }

    #[test]
    fn color_on_clicolor_force() {
        std::env::set_var("CLICOLOR_FORCE", "1");
        assert_eq!(colorize("test", "white"), "\u{1b}[37mtest\u{1b}[0m");

        std::env::remove_var("CLICOLOR_FORCE");
    }

    #[test]
    fn general_usecase() {
        // Forciing color output in non-tty environment
        std::env::set_var("CLICOLOR_FORCE", "1");

        assert_eq!(colorize("Lorem ipsum", "black"),   "\x1B[30mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", "red"),     "\x1B[31mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", "green"),   "\x1B[32mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", "yellow"),  "\x1B[33mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", "blue"),    "\x1B[34mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", "magenta"), "\x1B[35mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", "cyan"),    "\x1B[36mLorem ipsum\x1B[0m");
        assert_eq!(colorize("Lorem ipsum", "white"),   "\x1B[37mLorem ipsum\x1B[0m");

        assert_eq!(colorize("Lorem ipsum", "bold"), "\x1B[97mLorem ipsum\x1B[0m");

        assert_eq!(colorize("Lorem ipsum", "gray"), "\x1B[90mLorem ipsum\x1B[0m");
    }

    #[test]
    fn unknown_color() {
        assert_eq!(colorize("test", "blood_of_satan"), "test");
    }
}
