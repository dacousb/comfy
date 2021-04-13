use crate::{err, err_syntax, warning};
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

const EXTENSION: &str = "comfy";

pub fn parse(file: &Path, show_comments: bool) {
    let os = consts::OS;
    let pattern = ">";

    if check_file(file) {
        let parse_file = File::open(file).unwrap();
        let parse_reader = BufReader::new(parse_file);
        let mut line_os: String = "always".to_string();
        let mut _sysvar_contents: String = "undefined".to_string();

        for (index, line) in parse_reader.lines().enumerate() {
            let line = line.unwrap();
            let argument: Vec<&str> = line.split_whitespace().collect();

            if !line.is_empty() {
                if !(line.trim().is_empty()
                    || argument[0] == "//"
                    || argument[0] == "@"
                    || argument[0] == "#")
                {
                    if argument[0] == pattern {
                        line_os = argument[1].to_string();
                    } else if line_os == os || line_os == "always" {
                        exe_line(index, &line, os, &_sysvar_contents);
                    }
                } else if sysvar(&line) != "false" {
                    print_line(index, &line, "debug");
                    let sys = sysvar(&line);
                    _sysvar_contents = sys;
                } else if kword(&line, index, &_sysvar_contents) {
                    // kword does everything
                } else if show_comments {
                    print_line(index, &line, "sys");
                } else if &line[..2] != "//" {
                    warning!(&format!(
                        "syntax, line {} -> {} parser does not recognize it",
                        &(index + 1),
                        &line
                    ));
                }
            } else {
                warning!(&format!(
                    "syntax, line {} -> blank lines can originate errors",
                    &(index + 1)
                ));
            }
        }
    }
}

fn sysvar(line: &str) -> String {
    let argument: Vec<&str> = line.split_whitespace().collect();
    if argument[0] == "#" {
        let mut var = "".to_owned();
        for i in &argument[1..] {
            var.push_str(&format!("{} ", i));
        }
        var
    } else {
        "false".to_string()
    }
}

fn kword(line: &str, index: usize, _sysvar_contents: &str) -> bool {
    let argument: Vec<&str> = line.split_whitespace().collect();
    if argument[0] == "@" {
        match argument[1] {
            "sleep" => {
                print_line(index, &line, "non");
                if !argument[2].chars().all(char::is_numeric) {
                    err_syntax!(&format!(
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
            "print" => {
                print_line(index, &line, "non");
                for i in &argument[2..] {
                    if i == &"{sys}" {
                        print!("{} ", _sysvar_contents);
                    } else {
                        print!("{} ", i);
                    }
                }
                println!();
                true
            }
            _ => {
                err_syntax!(&format!(
                    "syntax error, line {} -> {} is not a comfy function",
                    &(index + 1),
                    &argument[1]
                ));
            }
        }
    } else {
        false
    }
}

fn check_file(file: &Path) -> bool {
    if file.is_file() && file.extension() == Some(OsStr::new(EXTENSION)) {
        true
    } else if file.is_file() {
        println!("{} is not a .comfy file, proceed? (y/N)", file.display());
        let mut input = String::new();

        match stdin().read_line(&mut input) {
            Ok(_) => input.trim_end().to_lowercase() == "y",
            Err(e) => {
                err!(&e.to_string());
            }
        }
    } else {
        err_syntax!(&format!("no such file named {}", file.display()));
    }
}

fn exe_line(index: usize, line: &str, os: &str, sysvar: &str) {
    print_line(index, &line, "sys");

    let to_parse_command: Vec<&str> = line.split_whitespace().collect();
    let mut to_exe = "".to_owned();
    for i in &to_parse_command {
        if i == &"{sys}" {
            to_exe.push_str(&format!("{} ", &sysvar));
        } else {
            to_exe.push_str(&format!("{} ", i));
        }
    }

    if os == "windows" {
        Command::new("cmd")
            .args(&["/C", &to_exe])
            .status()
            .unwrap_or_else(|_| panic!("err, line -> {}", &(index + 1)))
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&to_exe)
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
        // other color
        println!(
            "{}{} {}",
            (index + 1).to_string().truecolor(150, 150, 150),
            ":".truecolor(150, 150, 150),
            line.truecolor(150, 150, 150)
        );
    }

    if i == "debug" {
        println!(
            "{}{} {}{}",
            (index + 1).to_string().truecolor(150, 150, 150),
            ":".truecolor(150, 150, 150),
            line.truecolor(150, 150, 150),
            " [sysvar updated]".truecolor(150, 150, 150),
        );
    }
}
