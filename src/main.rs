mod colors;

use chrono::DateTime;
use getopts;
use std::io::{self, BufRead};
use yaml_rust::{YamlEmitter, YamlLoader};

fn format_line(mut obj: json::JsonValue) -> String {
    let mut result = String::default();

    // TODO: Support short version
    if obj.has_key("time") {
        let ts = obj["time"].as_str().unwrap();
        let t = DateTime::parse_from_rfc3339(&ts).unwrap();
        result.push_str(format!("{}", t.format("%H:%M:%S2%.3f")).as_str());
    } else {
        result.push_str("??:??:??.???");
    }

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

    obj.remove("time");
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
    opts.optopt("o", "", "set output mode", "([normal]|short)");
    opts.optflag("h", "help", "print this help menu");
    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => panic!(f.to_string()),
    };
    if matches.opt_present("h") {
        print_usage(&program, opts);
        return;
    }
    let output_mode: String;
    if matches.opt_present("o") {
        output_mode = match matches.opt_str("o") {
            Some(x) => x,
            None => String::from("normal"),
        };

        if output_mode != "normal" && output_mode != "short" {
            println!("unsupported output mode '{}'", output_mode);
            print_usage(&program, opts);
            return;
        }
    }
    // TODO: Support --short & --version & --help flags
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
