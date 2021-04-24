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

    /// Formats a script
    #[structopt(name = "fmt")]
    Fmt {
        /// Path to Comfy Script to format
        path: PathBuf,
    },
}

fn main() {
    let args = Arguments::from_args();

    match args.subcommand.unwrap_or(Command::Run {
        file: None,
        comments: false,
    }) {
        Command::HelpF => print_helpf(),
        Command::Fmt { path } => base::formater(&path),
        Command::Run { file, comments } => base::parse(
            &file.unwrap_or_else(|| PathBuf::from(DEFAULT_SCRIPT)),
            comments,
        ),
    }
}

fn print_helpf() {
    println!("comfy {}", env!("CARGO_PKG_VERSION"));
    println!();
    println!("  functions:                                                              ");
    println!("  @ sleep [int]              sleeps your program for [int] ms             ");
    println!("  @ print [str]              prints given text                            ");
    println!();
    println!("  sysvar:                                                                 ");
    println!("  # [str]                    saves a variable in the sysvar               ");
    println!("  #-> [cmd]                  saves the output of a command in the sysvar  ");
    println!();
    println!("  conditional clauses:                                                    ");
    println!("  _if [str] = [str]          if true, executes everything until _endif    ");
    println!("            ├!=                                                           ");
    println!("            └contains                                                     ");
    println!("  _endif                     exits the if conditional clause              ");
    println!();
    println!("  examples:                                                               ");
    println!("  @ print {{sys}}              prints sysvar                              ");
    println!();
}
