use clap::{Parser, Subcommand};

#[derive(Subcommand)]
enum Action {
    Run { script_file_path: String },
}

#[derive(Parser)]
#[command(author, version, about)]
struct CmdArgs {
    #[command(subcommand)]
    action: Action,
}

fn main() {
    let cmd_args = CmdArgs::parse();
    match cmd_args.action {
        Action::Run { script_file_path } => println!("{}", script_file_path),
    }
}
