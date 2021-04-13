use structopt::StructOpt;

use std::path::PathBuf;
mod base;

const DEFAULT_SCRIPT: &str = "./run.comfy";

#[derive(StructOpt, Debug)]
#[structopt(
    about = "Cross-platform script/command manager and runner\nBy default tries to run script ./run.comfy"
)]
struct Arguments {
    #[structopt(subcommand)]
    pub subcommand: Option<Command>,
}

#[derive(StructOpt, Debug)]
enum Command {
    /// Scripting help
    #[structopt(name = "helpf")]
    HelpF,

    /// Runs a script
    Run {
        /// Path to Comfy Script to run
        file: Option<PathBuf>,
        /// Shows comments from source while running
        #[structopt(short, long = "c")]
        comments: bool,
    },
}

fn main() {
    let args = Arguments::from_args();

    match args.subcommand.unwrap_or(Command::Run {
        file: None,
        comments: false,
    }) {
        Command::HelpF => print_helpf(),
        Command::Run { file, comments } => base::parse(
            &file.unwrap_or_else(|| PathBuf::from(DEFAULT_SCRIPT)),
            comments,
        ),
    }
}

fn print_helpf() {
    println!("comfy {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("  @[space]function         is how you call a function       ");
    println!("  sleep [int]              sleeps your program for [int] ms ");
    println!("  print [str]              prints given text                ");
    println!("  # [str]                  saves a variable in the sysvar   ");
    println!("  print {{sys}}            prints sysvar                  ");
    println!();
}
