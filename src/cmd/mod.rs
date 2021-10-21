use clap::Clap;

mod init;
mod index;

#[derive(Clap)]
pub enum SubCommand {
    Init(init::InitCommand),
    Index(index::IndexCommand)
}

pub fn execute(cmd: SubCommand) {
    match cmd {
        SubCommand::Init(t) => {
            init::execute(t);
        },
        SubCommand::Index(t) => {
            index::execute(t)
        }
    }
}