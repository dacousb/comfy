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
const SYS: &str = "{sys}";
const PATTERN: &str = ">";

pub fn parse(file: &Path, show_comments: bool) {
    let os = consts::OS;

    if check_file(file) {
        let parse_file = File::open(file).unwrap();
        let parse_reader = BufReader::new(parse_file);
        let mut line_os: String = "always".to_string();
        let mut _sysvar_contents: String = "undefined".to_string();
        let mut if_true = true;
        let mut inside_if = false;

        for (index, line) in parse_reader.lines().enumerate() {
            let line = line.unwrap();
            let argument: Vec<&str> = line.split_whitespace().collect();
            let mut m_argument = argument.to_owned();

            if !line.trim().is_empty() && argument[0] == PATTERN {
                line_os = argument[1].to_string();
            } else if line_os == os || line_os == "always" {
                if !line.trim().is_empty() && argument[0] == "_endif" {
                    print_line(index, &line, "sys");
                    if_true = true;
                    inside_if = false;
                } else if !line.trim().is_empty() && if_true {
                    match argument[0] {
                        "_if" => {
                            if argument.len() == 4 {
                                if inside_if {
                                    err_syntax!(&format!(
                                    "syntax, line {} -> {} nested _if is illegal, use _endif before starting another comparison",
                                    &(index + 1),
                                    &line.italic()
                                ));
                                }
                                print_line(index, &line, "sys");
                                for (i, l) in argument.iter().enumerate() {
                                    if l == &SYS {
                                        m_argument[i] = &_sysvar_contents;
                                    }
                                }
                                inside_if = true;
                                if_true = compare(m_argument);
                            } else {
                                err_syntax!(&format!(
                                    "syntax, line {} -> {} must follow the syntax x [comparison] x",
                                    &(index + 1),
                                    &line.italic()
                                ));
                            }
                        }
                        "_endif" => {
                            print_line(index, &line, "sys");
                            inside_if = false;
                        }
                        "#" | "#->" => {
                            print_line(index, &line, "debug");
                            _sysvar_contents = sysvar(&line);
                        }
                        "//" => {
                            if show_comments {
                                print_line(index, &line, "sys");
                            }
                        }
                        "@" => {
                            kword(&line, index, &_sysvar_contents);
                        }
                        _ => {
                            exe_line(index, &line, os, &_sysvar_contents);
                        }
                    }
                } else if !inside_if || if_true {
                    warning!(&format!(
                        "syntax, line {} -> blank lines can originate errors",
                        &(index + 1)
                    ));
                }
            }
        }
    }
}

fn compare(m_argument: Vec<&str>) -> bool {
    if m_argument[2] == "=" {
        m_argument[1] == m_argument[3]
    } else if m_argument[2] == "!=" {
        m_argument[1] != m_argument[3]
    } else if m_argument[2] == "contains" {
        m_argument[1].contains(m_argument[3])
    } else {
        err_syntax!(format!(
            "malformed _if statement -> you cannot compare with {}",
            m_argument[2]
        ));
    }
}

fn sysvar(line: &str) -> String {
    let argument: Vec<&str> = line.split_whitespace().collect();
    if argument[0] == "#" {
        let mut var = "".to_owned();
        for i in &argument[1..] {
            var.push_str(&format!("{} ", i));
        }
        var.pop();
        var
    } else if argument[0] == "#->" {
        result_exe_line(line)
    } else {
        "undefined".to_string()
    }
}

fn result_exe_line(line: &str) -> String {
    let argument: Vec<&str> = line.split_whitespace().collect();
    let mut to_exe = "".to_owned();
    for i in &argument[1..] {
        to_exe.push_str(&format!("{} ", i));
    }
    to_exe.pop();

    let output = if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", &to_exe])
            .output()
            .expect("failed to execute process")
    } else {
        Command::new("sh")
            .arg("-c")
            .arg(&to_exe)
            .output()
            .expect("failed to execute process")
    };
    let mut result = String::from_utf8(output.stdout).unwrap();
    if result.ends_with('\n') {
        result.pop();
        if result.ends_with('\r') {
            result.pop();
        }
    }
    result
}

fn kword(line: &str, index: usize, _sysvar_contents: &str) {
    let argument: Vec<&str> = line.split_whitespace().collect();
    match argument[1] {
        "sleep" => {
            print_line(index, line, "non");
            if !argument[2].chars().all(char::is_numeric) {
                err_syntax!(&format!(
                    "syntax error, line {} -> {} is not [int]",
                    &(index + 1),
                    &argument[2].italic()
                ));
            }
            thread::sleep(time::Duration::from_millis(
                (argument[2]).parse::<u64>().unwrap(),
            ));
        }
        "print" => {
            print_line(index, line, "non");
            for i in &argument[2..] {
                if i == &SYS {
                    print!("{} ", _sysvar_contents);
                } else if i == &"\\n" {
                    println!();
                } else {
                    print!("{} ", i);
                }
            }
            println!();
        }
        _ => {
            err_syntax!(&format!(
                "syntax error, line {} -> {} is not a comfy function",
                &(index + 1),
                &argument[1].italic()
            ));
        }
    }
}

pub fn check_file(file: &Path) -> bool {
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
    print_line(index, line, "sys");

    let to_parse_command: Vec<&str> = line.split_whitespace().collect();
    let mut to_exe = "".to_owned();
    for i in &to_parse_command {
        if i == &SYS {
            to_exe.push_str(&format!("{} ", &sysvar));
        } else {
            to_exe.push_str(&format!("{} ", i));
        }
    }
    to_exe.pop();

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
    } else if i == "non" {
        println!(
            "{}{} {} {}",
            (index + 1).to_string().truecolor(150, 150, 150),
            ":".truecolor(150, 150, 150),
            line.truecolor(150, 150, 150),
            "-> comfy".truecolor(150, 150, 150)
        );
    } else if i == "debug" {
        println!(
            "{}{} {}{}",
            (index + 1).to_string().truecolor(150, 150, 150),
            ":".truecolor(150, 150, 150),
            line.truecolor(150, 150, 150),
            " (sysvar updated)".truecolor(150, 150, 150)
        );
    } else {
        warning!("comfy internal error -> print_line");
    }
}
