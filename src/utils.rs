/// An implementation of https://bixense.com/clicolors/
fn clicolor() -> bool {
    let color_var = std::env::var("CLICOLOR").unwrap_or(String::from("1"));
    let force_color = std::env::var("CLICOLOR_FORCE").unwrap_or(String::from("0"));

    if (color_var != "0" && atty::is(atty::Stream::Stdout)) || force_color != "0" {
        return true;
    }
    return false;
}

/// Returns whether the output should be colored based on the environment.
pub fn color_from_env() -> bool {
    match std::env::var("NO_COLOR") {
        // If set (regardless to what) -> don't color output.
        Ok(_) => return false,

        // Var is not present, bad unicode
        Err(_) => return clicolor(),
    }
}

#[cfg(test)]
mod test {
    use super::color_from_env;

    #[test]
    #[ignore]
    fn color_on_clicolor() {
        std::env::set_var("CLICOLOR", "1");
        assert_eq!(color_from_env(), true);
        std::env::remove_var("CLICOLOR");
    }

    #[test]
    fn do_not_color_when_nocolor_set() {
        std::env::set_var("NO_COLOR", "1");
        assert_eq!(color_from_env(), false);
        std::env::remove_var("NO_COLOR");
    }

    #[test]
    fn color_on_clicolor_force() {
        std::env::set_var("CLICOLOR_FORCE", "1");
        assert_eq!(color_from_env(), true);
        std::env::remove_var("CLICOLOR_FORCE");
    }
}