extern crate clap;
use clap::Clap;

#[macro_use] extern crate log;
extern crate simple_logger;

const VERSION: &str = "0.3.0";
const AUTHOR:  &str = "Luís Sobral Silva <luiscomsnofim@gmail.com>";

#[derive(Clap)]
#[clap(version = VERSION, author = AUTHOR)]
struct Opts {
    /// sets the verbosity of the logger Error(1)-Warn-Info-Debug-Trace(5)
    #[clap(short, long, parse(from_occurrences))]
    verbose: i32,
    #[clap(subcommand)]
    subcmd: Option<SubCommand>,
}

#[derive(Clap)]
enum SubCommand {
    Init(InitCommand),
    New(NewCommand),
    Add(AddCommand),
    List(ListCommand),
    Update(UpdateCommand),
    Export(ExportCommand),
    Generate(GenerateCommand)
}

#[derive(Clap)]
#[clap(version = VERSION, author = AUTHOR)]
struct InitCommand{
    /// The folder where you want to initialize NOTA
    /// (defaults to create in the current folder)
    #[clap(long)]
    folder: Option<String>
}

#[derive(Clap)]
#[clap(version = VERSION, author = AUTHOR)]
struct NewCommand{
    /// Name to be provided to the new NOTA
    /// (default: using the current timestamp)
    #[clap(long)]
    _name: bool
}

#[derive(Clap)]
#[clap(version = VERSION, author = AUTHOR)]
struct AddCommand{
    /// Provide folder or file to be added. 
    /// Folder structure will not be preserved, only files inside will be copied
    #[clap(short, long)]
    input: String
}

/// lists indexed NOTAs
/// basically, prints the current index state
#[derive(Clap)]
#[clap(version = VERSION, author = AUTHOR)]
struct ListCommand{
}

/// updates the current NOTA folder
/// For instance, adding all the missing files to the index
#[derive(Clap)]
#[clap(version = VERSION, author = AUTHOR)]
struct UpdateCommand{
    /// clears the index and re adds everything
    #[clap(long)]
    _hard: bool
}

/// Book generation commands
#[derive(Clap, Debug)]
#[clap(version = VERSION, author = AUTHOR)]
struct ExportCommand{
    /// can be a file or a folder (default: indexed notes on current NOTA folder)
    #[clap(short, long)]
    input: Option<String>,
    /// output folder
    #[clap(short, long)]
    outfolder: Option<String>,
    /// folder where we can find templates to use
    #[clap(short, long)]
    templates: Option<String>,
}

#[derive(Clap, Debug)]
#[clap(version = VERSION, author = AUTHOR)]
struct GenerateCommand{

}

fn main() {
    let opts: Opts = Opts::parse();

    match opts.verbose {
        1 => simple_logger::init_with_level(log::Level::Error).unwrap(), 
        2 => simple_logger::init_with_level(log::Level::Warn).unwrap(), 
        3 => simple_logger::init_with_level(log::Level::Info).unwrap(), 
        4 => simple_logger::init_with_level(log::Level::Debug).unwrap(), 
        5 => simple_logger::init_with_level(log::Level::Trace).unwrap(), 
        _ => simple_logger::init_with_level(log::Level::Info).unwrap()
    }

    debug!("Logger initialized");

    let subcommand = match opts.subcmd {
        Some(subcommand) => subcommand,
        None => return
    };

    match subcommand {
        SubCommand::Generate(args) => {
            nota::generate()
        },
        _ => ()
    }

}
