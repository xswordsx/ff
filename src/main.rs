mod colors;

use chrono::{DateTime, FixedOffset};
use getopts;
use std::io::{self, BufRead};
use yaml_rust::{YamlEmitter, YamlLoader};

fn print_version(program: &str, as_json: bool) {
    let short_name = std::path::Path::new(program)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();

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
            short_name, version, hash, build_at,
        );
        return;
    }

    println!(
        "{} version {} build {} built at {}",
        short_name, version, hash, build_at,
    );
    return;
}

fn extract_date(obj: &json::JsonValue) -> Result<(DateTime<FixedOffset>, &'static str), &'static str> {
    let possible_keys = ["ts", "time", "timestamp"];

    let prop_key = possible_keys.iter().find(|x| obj.has_key(x));
    if prop_key.is_none() {
        return Err("no time field provided");
    }
    let prop = prop_key.unwrap();
    let ts = obj[*prop].as_str().unwrap();
    match DateTime::parse_from_rfc3339(ts) {
        Ok(v) => return Ok((v, prop)),
        Err(_) => return Err("could not parse time"),
    };
}

fn format_line(mut obj: json::JsonValue) -> String {
    let mut result = String::default();

    // TODO: Implement --short timestamp
    match extract_date(&obj) {
        Ok(t) => {
            result.push_str(format!("{}", t.0.format("%H:%M:%S2%.3f")).as_str());
            obj.remove(t.1);
        }
        Err(_) => result.push_str("00:00:000.000"),
    };

    if obj.has_key("severity") {
        let svrty = severity_fmt(obj["severity"].as_str().unwrap());
        result.push(' ');
        result.push_str(svrty.as_str());
    } else {
        result.push_str(" [UNKNOWN]");
    }

    if obj.has_key("component") {
        let cmpt = severity_fmt(obj["component"].as_str().unwrap());
        result.push(' ');
        result.push_str(colors::colorize(cmpt.as_str(), "cyan").as_str());
    }

    if obj.has_key("message") {
        result.push(' ');
        result.push_str(obj["message"].as_str().unwrap());
    } else {
        result.push_str(" <no message>");
    }

    obj.remove("severity");
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
        let colored = colors::colorize(formatted_data.as_str(), "gray");
        result.push_str(colored.as_str());
    }

    return result;
}

fn severity_fmt(s: &str) -> String {
    // TODO: Support short version
    match s {
        "trace" => return format!("[{}]", colors::colorize("TRACE", "")),
        "debug" => return format!("[{}]", colors::colorize("DEBUG", "bold")),
        "info" => return format!("[{} ]", colors::colorize("INFO", "green")),
        "warn" => return format!("[{} ]", colors::colorize("WARN", "yellow")),
        "warning" => return format!("[{} ]", colors::colorize("WARN", "yellow")),
        "error" => return format!("[{}]", colors::colorize("ERROR", "red")),
        "fatal" => return format!("[{}]", colors::colorize("FATAL", "bold")),

        _ => return String::from(s),
    }
}

fn print_usage(program: &str, opts: getopts::Options) {
    let short_name = std::path::Path::new(program)
        .file_name()
        .unwrap()
        .to_str()
        .unwrap();
    let brief = format!("Usage: {} [options]", short_name);
    print!("{}", opts.usage(&brief));
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let program = args[0].clone();

    let mut opts = getopts::Options::new();
    opts.optopt("o", "output", "set output mode", "([normal]|short)");
    opts.optflag("v", "version", "prints out the version and build information");
    opts.optflag("", "json", "output it as JSON (for --version flag only)");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("help") {
        print_usage(&program, opts);
        return;
    }
    if matches.opt_present("version") {
        print_version(&program, matches.opt_present("json"));
        return;
    }
    let output_mode: String;
    if matches.opt_present("output") {
        output_mode = match matches.opt_str("output") {
            Some(x) => x,
            None => String::from("normal"),
        };

        if output_mode != "normal" && output_mode != "short" {
            println!("unsupported output mode '{}'", output_mode);
            print_usage(&program, opts);
            std::process::exit(1);
        }
    }
    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let raw_line = line.unwrap();
        let parsed_line = json::parse(&raw_line).unwrap_or(json::object! {});
        if parsed_line == json::object! {} {
            println!("{}", &raw_line);
            continue;
        }
        println!("{}", format_line(parsed_line));
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn prints_zeroes_on_missing_timestamp() {
        let input_str ="{\"message\":\"hi\",\"severity\":\"info\"}";
        let input_json: json::JsonValue = json::parse(input_str).unwrap();
        let expected = String::from("00:00:000.000 [INFO ] hi");
        std::env::set_var("CLICOLOR", "0");
        let output = format_line(input_json);
        std::env::remove_var("CLICOLOR");
        assert_eq!(output, expected);
    }
}