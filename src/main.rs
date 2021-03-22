use std::env;
mod base;

static VERSION: &str = env!("CARGO_PKG_VERSION");

fn main() {
    let args: Vec<String> = env::args().collect();
    match args.len() {
        1 => {
            base::err("no arguments");
        }
        2 => {
            if args[1] == "help" || args[1] == "--help" {
                print_help();
            } else if args[1] == "helpf" || args[1] == "--helpf" {
                print_helpf();
            } else {
                err_syntax()
            }
        }
        3 => {
            if args[1] == "file" || args[1] == "--file" {
                base::parse(&args[2], false);
            } else {
                err_syntax()
            }
        }
        4 => {
            if (args[1] == "file" || args[1] == "--file") && args[3] == "--c" {
                base::parse(&args[2], true);
            } else if (args[2] == "file" || args[2] == "--file") && args[1] == "--c" {
                base::parse(&args[3], true);
            }
        }
        _ => {
            err_syntax();
        }
    }
}

fn print_help() {
    println!("comfy {}", VERSION);
    println!();
    println!("  --help                   shows this help page and exits                   ");
    println!("  --helpf                  shows help about comfy functions                 ");
    println!("  --file <foo.comfy>       runs comfy script contents                       ");
    println!("  --file <foo.comfy> --c   runs comfy script contents, also prints comments ");
    println!();
}

fn print_helpf() {
    println!("comfy {}", VERSION);
    println!();
    println!("  @[space]function         is how you call a function       ");
    println!("  sleep [int]              sleeps your program for [int] ms ");
    println!();
}

fn err_syntax() {
    base::err("comfy cannot understand that command");
}
