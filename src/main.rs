use std::collections::HashMap;

use clap::Parser;

mod actions_registry;

mod cmd {
    use clap::{Parser, Subcommand};

    #[derive(Subcommand)]
    pub enum Action {
        Run { script_file_path: String },
    }

    #[derive(Parser)]
    #[command(author, version, about)]
    pub struct Args {
        #[command(subcommand)]
        action: Action,
    }
}


#[derive(PartialEq, Eq, Hash)]
struct Name {
    contents: Vec<NamePart>,
}

struct Action {}

struct Executor {
    execute: HashMap<Name, Action>,
}

impl Executor {
    fn run(&self, program: hzlang_parser::Program) -> Result<(), ()> {
        for action_invocation in program.contents {

        }
    }
}

fn main() {
    let cmd_args = cmd::Args::parse();
    match cmd_args.action {
        Action::Run { script_file_path } => println!("{}", script_file_path),
    }
}
