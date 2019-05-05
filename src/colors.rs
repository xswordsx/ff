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
    return format!("\x1B[90m{}\x1B[39m", s);
}
fn bold(s: &str) -> std::string::String {
    return format!("\x1B[97m{}\x1B[0m", s);
}
