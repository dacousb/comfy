use crate::base::error_handler::{err, err_syntax, warning};
use colored::Colorize;
use std::{
    env::consts,
    ffi::OsStr,
    fs::File,
    io::{stdin, BufRead, BufReader},
    path::Path,
    process::Command,
    thread, time,
};

pub fn parse(file: &str, show_comments: bool) {
    let os = consts::OS;
    let pattern = ">";

    if check_file(file) {
        let parse_file = File::open(file).unwrap();
        let parse_reader = BufReader::new(parse_file);
        let mut line_os: String = "default".to_string();

        for (index, line) in parse_reader.lines().enumerate() {
            let line = line.unwrap();
            let argument: Vec<&str> = line.split_whitespace().collect();

            if !line.is_empty() {
                if !(argument[0] == "//" || line.trim().is_empty() || argument[0] == "@") {
                    if argument[0] == pattern {
                        line_os = argument[1].to_string();
                    } else if line_os == os || line_os == "always" {
                        exe_line(index, &line, os);
                    }
                } else if kword(&line, index) {
                    // kword does everything
                } else if show_comments {
                    print_line(index, &line, "sys");
                } else if &line[..2] != "//" {
                    warning(&format!(
                        "syntax, line {} -> {} parser does not recognize it",
                        &(index + 1),
                        &line
                    ));
                }
            } else {
                warning(&format!(
                    "syntax, line {} -> blank lines can originate errors",
                    &(index + 1)
                ));
            }
        }
    }
}

fn kword(line: &str, index: usize) -> bool {
    let argument: Vec<&str> = line.split_whitespace().collect();
    if argument[0] == "@" {
        match argument[1] {
            "sleep" => {
                print_line(index, &line, "non");
                if !argument[2].chars().all(char::is_numeric) {
                    err_syntax(&format!(
                        "syntax error, line {} -> {} is not [int]",
                        &(index + 1),
                        &argument[2]
                    ));
                }
                thread::sleep(time::Duration::from_millis(
                    (argument[2]).parse::<u64>().unwrap(),
                ));
                true
            }
            _ => {
                err_syntax(&format!(
                    "syntax error, line {} -> {} is not a comfy function",
                    &(index + 1),
                    &argument[1]
                ));
                true
            }
        }
    } else {
        false
    }
}

fn check_file(file: &str) -> bool {
    if Path::new(file).is_file() && Path::new(file).extension() == Some(OsStr::new("comfy")) {
        true
    } else if Path::new(file).is_file() {
        println!("{} is not a .comfy file, proceed? (y/N)", file);
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => input.trim_end().to_lowercase() == "y",
            Err(e) => {
                err(&e.to_string());
                false
            }
        }
    } else {
        err_syntax(&format!("no such file named {}", &file));
        false
    }
}

fn exe_line(index: usize, line: &str, os: &str) {
    print_line(index, &line, "sys");
    if os == "windows" {
        Command::new("cmd")
            .args(&["/C", &line])
            .status()
            .unwrap_or_else(|_| panic!("err, line -> {}", &(index + 1)))
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&line)
            .status()
            .unwrap_or_else(|_| panic!("err, line -> {}", &(index + 1)))
    };
}

fn print_line(index: usize, line: &str, i: &str) {
    if i == "sys" {
        println!(
            "{}{} {}",
            (index + 1).to_string().truecolor(150, 150, 150),
            ":".truecolor(150, 150, 150),
            line.truecolor(150, 150, 150)
        );
    }

    if i == "non" {
        // changable color
        println!(
            "{}{} {}",
            (index + 1).to_string().truecolor(150, 150, 150),
            ":".truecolor(150, 150, 150),
            line.truecolor(150, 150, 150)
        );
    }
}
