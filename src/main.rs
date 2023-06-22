use std::sync::Arc;

use action_storage::ActionStorage;
use clap::Parser;
use cmd::Action;

mod action_storage;

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
        pub action: Action,
    }
}

mod object {
    use std::fmt::Display;

    pub trait Object: Display {}
}

struct Executor {
    action_storage: ActionStorage<Arc<dyn object::Object>>,
}

impl Executor {
    pub fn run(&self, program: hzlang_parser::Program) -> anyhow::Result<()> {
        for action_invocation in program.contents {
            if let Some(action_invocation) = action_invocation.contents.action_invocation {
                self.action_storage.call()?
                action_invocation.name
            }
        }
    }
}

fn main() {
    let cmd_args = cmd::Args::parse();
    match cmd_args.action {
        Action::Run { script_file_path } => println!("{}", script_file_path),
    }
}
