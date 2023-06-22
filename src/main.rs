use clap::Parser;
use cmd::Action;

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

struct Executor {}

impl Executor {
    pub fn run(&self, program: hzlang_parser::Program) -> anyhow::Result<()> {
        for line in program.contents {
            match line {
                hzlang_parser::Line::Filled {
                    action_invocation,
                    comment,
                    attached,
                } => {
                    let parts = action_invocation.name.parts;
                    if parts[0]
                        == hzlang_parser::NamePart::Word(hzlang_parser::Word("print".to_owned()))
                    {
                        let hzlang_parser::NamePart::Filler(hzlang_parser::Filler::String(string)) = &parts[1] else { panic!() };
                        let mut raw_parts_combined = String::new();
                        for part in &string.parts {
                            match part {
                                hzlang_parser::HzStringPart::Name(_) => panic!(),
                                hzlang_parser::HzStringPart::Raw(
                                    hzlang_parser::RawHzStringPart(raw),
                                ) => raw_parts_combined.push_str(raw),
                            }
                        }
                        println!("{}", raw_parts_combined);
                    }
                }
                hzlang_parser::Line::Empty { .. } => (),
            }
        }
        Ok(())
    }
}

fn main() {
    let cmd_args = cmd::Args::parse();
    match cmd_args.action {
        Action::Run { script_file_path } => {
            let executor = Executor {};
            executor
                .run(
                    hzlang_parser::parse(&std::fs::read_to_string(script_file_path).unwrap())
                        .unwrap(),
                )
                .unwrap();
        }
    }
}
