mod colors;
mod utils;

use chrono::{DateTime, FixedOffset};
use getopts;
use std::io::{self, BufRead, Write};
use yaml_rust::{YamlEmitter, YamlLoader};

fn print_version(program: &str, as_json: bool) {
    let fallback_version = format!("{}-dev", env!("CARGO_PKG_VERSION"));
    let version = option_env!("VERSION").unwrap_or(fallback_version.as_str());
    let hash = option_env!("HASH").unwrap_or("0000000");
    let build_at = option_env!("BUILD_AT").unwrap_or("1970-01-01");

    if as_json {
        println!(
            "{{\
            \"program\":\"{}\",\
            \"version\":\"{}\",\
            \"hash\":\"{}\",\
            \"build_at\":\"{}\"\
            }}",
            program, version, hash, build_at,
        );
        return;
    }

    println!(
        "{} version {} build {} built at {}",
        program, version, hash, build_at,
    );
    return;
}

/// Returns a tuple of `<value, key>` with the date from a
/// JSON object.
fn extract_date(obj: &json::JsonValue) -> Option<(DateTime<FixedOffset>, &'static str)> {
    let possible_keys = ["ts", "time", "timestamp"];

    let prop_key = possible_keys.iter().find(|x| obj.has_key(x));
    if prop_key.is_none() {
        return None;
    }
    let prop = prop_key.unwrap();
    let ts = obj[*prop].as_str().unwrap();
    match DateTime::parse_from_rfc3339(ts) {
        Ok(v) => return Some((v, prop)),
        Err(_) => return None,
    };
}

/// Returns a (value, key) pair of the object's severity.
fn extract_severity(obj: &json::JsonValue) -> Option<(&str, &'static str)> {
    let possible_keys = ["severity", "level", "lvl"];

    let prop_key = possible_keys.iter().find(|x| obj.has_key(x));

    if prop_key.is_none() {
        return None;
    }
    let prop = prop_key.unwrap();
    let lvl = obj[*prop].as_str().clone();
    Some((lvl?, prop))
}

fn format_line(mut obj: json::JsonValue, _mode: &Output, color_output: bool) -> String {
    let mut result = String::default();

    let colorizer: fn(&str, colors::Color) -> String = match color_output {
        true => colors::colorize,
        false => |x, _| String::from(x),
    };

    // TODO: Implement --short timestamp
    match extract_date(&obj) {
        Some(t) => {
            result.push_str(format!("{}", t.0.format("%H:%M:%S%.3f")).as_str());
            obj.remove(t.1);
        }
        None => result.push_str("00:00:000.000"),
    };

    match extract_severity(&obj) {
        Some(t) => {
            result.push(' ');
            result.push_str(severity_fmt(t.0, colorizer).as_str());
            obj.remove(t.1);
        }
        None => result.push_str(" [UNKNOWN]"),
    };

    if obj.has_key("component") {
        let cmpt = obj["component"].as_str().unwrap();
        result.push(' ');
        result.push_str(colorizer(cmpt, colors::Color::Cyan).as_str());
    }

    if obj.has_key("message") {
        result.push(' ');
        result.push_str(obj["message"].as_str().unwrap());
    } else {
        result.push_str(" <no message>");
    }

    obj.remove("message");
    obj.remove("component");

    // Dump the YAML object
    let mut out_str = String::new();
    {
        let doc = YamlLoader::load_from_str(obj.dump().as_str()).unwrap();
        let mut emitter = YamlEmitter::new(&mut out_str);
        emitter.dump(&doc[0]).unwrap(); // dump the YAML object to a String
    }
    if out_str != "---\n{}" && out_str != "---" && out_str != "" {
        let formatted_data = out_str.trim_start_matches("---").replace('\n', "\n  ");
        let colored = colorizer(formatted_data.as_str(), colors::Color::Gray);
        result.push_str(colored.as_str());
    }

    return result;
}

fn severity_fmt(s: &str, colorizer: fn(&str, colors::Color) -> String) -> String {
    // TODO: Support short version
    match s {
        "trace" => return format!("[{}]", colorizer("TRACE", colors::Color::None)),
        "debug" => return format!("[{}]", colorizer("DEBUG", colors::Color::Bold)),
        "info" => return format!("[{} ]", colorizer("INFO", colors::Color::Green)),
        "warn" => return format!("[{} ]", colorizer("WARN", colors::Color::Yellow)),
        "warning" => return format!("[{} ]", colorizer("WARN", colors::Color::Yellow)),
        "error" => return format!("[{}]", colorizer("ERROR", colors::Color::Red)),
        "fatal" => return format!("[{}]", colorizer("FATAL", colors::Color::Bold)),

        _ => return String::from(s),
    }
}

fn print_usage(program: &str, opts: getopts::Options) {
    let brief = format!("Usage: {} [options]", program);
    print!("{}", opts.usage(&brief));
}

#[derive(Debug)]
enum Output {
    Unknown,
    Normal,
    Short,
}

#[derive(Debug)]
struct Args {
    program_name: std::string::String,
    output: Output,
    help: bool,
    json: bool,
    color: bool,
    version: bool,
}

type ArgsResult = Result<Args, std::string::String>;

fn parse_args(opts: &getopts::Options) -> ArgsResult {
    let cmd_args: Vec<String> = std::env::args().collect();
    let matches = opts.parse(&cmd_args[1..]).unwrap();

    Ok(Args {
        program_name: std::string::String::from(
            std::path::Path::new(&cmd_args[0])
                .file_name().unwrap()
                .to_str().unwrap(),
        ),
        output: match matches.opt_default("output", "normal") {
            Some(x) if x == "normal" => Output::Normal,
            Some(x) if x == "short" => Output::Short,
            Some(x) => return Err(format!("unsupported output mode '{}'", x)),
            _ => Output::Unknown,
        },
        json: matches.opt_present("json"),
        help: matches.opt_present("help"),
        version: matches.opt_present("version"),
        color: utils::color_from_env(),
    })
}

fn main() {
    let mut opts = getopts::Options::new();
    opts.optopt("o", "output", "set output mode", "([normal]|short)");
    opts.optflag(
        "v",
        "version",
        "prints out the version and build information",
    );
    opts.optflag("", "json", "output it as JSON (for --version flag only)");
    opts.optflag("h", "help", "print this help menu");

    let args = match parse_args(&opts) {
        Ok(val) => val,
        Err(x) => {
            eprintln!("{}", x);
            print_usage("ff", opts);
            std::process::exit(1);
        }
    };

    if args.help {
        print_usage(args.program_name.as_str(), opts);
        return;
    }

    if args.version {
        print_version(args.program_name.as_str(), args.json);
        return;
    }

    // Acquiring a lock on stdout is faster than invoking
    // println! for each line.
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    for line in io::stdin().lock().lines() {
        let raw_line = line.unwrap();
        let _ = match json::parse(&raw_line) {
            Ok(val) if val.is_object() => {
                writeln!(handle, "{}", format_line(val, &args.output, args.color))
            }
            _ => {
                writeln!(handle, "{}", &raw_line)
            }
        };
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_date_extraction() {
        let kv_pairs: [(&str, &str); 3] = [
            ("time", "1970-01-01T01:23:45.000Z"),
            ("ts", "2020-02-03T12:34:56.000Z"),
            ("timestamp", "1234-05-06T07:08:09.000Z"),
        ];
        for tup in &kv_pairs {
            let [key, value] = [tup.0, tup.1];
            let input_str = format!("{{\"{}\":\"{}\"}}", key, value);

            let input = json::parse(input_str.as_str()).unwrap();
            let result = extract_date(&input).unwrap();

            let result_key = result.1;
            let result_value = result.0.to_rfc3339().replace("+00:00", ".000Z");
            assert_eq!(result_key, key);
            assert_eq!(result_value, String::from(value));
        }
    }

    #[test]
    fn prints_zeroes_on_missing_timestamp() {
        let input_str = r#"{"message":"hi","severity":"info"}"#;
        let input_json: json::JsonValue = json::parse(input_str).unwrap();
        let expected = String::from("00:00:000.000 [INFO ] hi");
        let output = format_line(input_json, &Output::Normal, false);
        assert_eq!(output, expected);
    }
}
