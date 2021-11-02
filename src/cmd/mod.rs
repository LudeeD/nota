use clap::Parser;

mod init;
mod index;
mod build;
mod open;

#[derive(Parser)]
pub enum SubCommand {
    Init(init::InitCommand),
    Index(index::IndexCommand),
    Build(build::BuildCommand),
    Open(open::OpenCommand)
}


pub fn execute(cmd: SubCommand) {
    match cmd {
        SubCommand::Init(t) => {
            init::execute(t);
        },
        SubCommand::Index(t) => {
            index::execute(t)
        },
        SubCommand::Build(t) => {
            build::execute(t)
        },
        SubCommand::Open(t) => {
            open::execute(t)
        }
        SubCommand::Build(t) => {
            build::execute(t)
        }
    }
}
