use std::env;
use std::path::{PathBuf};
use clap::{AppSettings, Clap};
use log::{debug, error, log_enabled, info, Level};


mod cmd;
use cmd::SubCommand;


#[derive(Clap)]
#[clap(version = "0.5.0", author = "Luís Sobral Silva <luiscomsnofim@gmail.com>")]
struct Opts {
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcommand: SubCommand,
}

fn main() {
    env_logger::init();

    let opts: Opts = Opts::parse();

    cmd::execute(opts.subcommand);
}
