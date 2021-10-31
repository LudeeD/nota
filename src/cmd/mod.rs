use clap::Clap;

mod init;
mod index;
mod build;

#[derive(Clap)]
pub enum SubCommand {
    Init(init::InitCommand),
    Index(index::IndexCommand),
    Build(build::BuildCommand)
}


pub fn execute(cmd: SubCommand) {
    match cmd {
        SubCommand::Init(t) => {
            init::execute(t);
        },
        SubCommand::Index(t) => {
            index::execute(t)
        }
        SubCommand::Build(t) => {
            build::execute(t)
        }
    }
}