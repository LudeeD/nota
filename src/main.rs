#[macro_use]
extern crate clap;
use clap::Clap;

#[macro_use]
extern crate log;

extern crate simple_logger;

use std::path::{PathBuf};

const VERSION: &str = "0.2.1";
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
    Export(ExportCommand)
}

#[derive(Clap)]
#[clap(version = VERSION, author = AUTHOR)]
struct InitCommand{
    /// The folder where you want to initialize NOTA
    /// (defaults to create in the current folder)
    #[clap(long)]
    folder: String
}

#[derive(Clap)]
#[clap(version = VERSION, author = AUTHOR)]
struct NewCommand{
    /// Name to be provided to the new NOTA
    /// (default: using the current timestamp)
    #[clap(long)]
    name: bool
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
    hard: bool
}

/// Book generation commands
#[derive(Clap)]
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

fn assert_nota_folder() {
    if ! nota::assert_nota_folder() {
        info!("Not a NOTA folder, stoping...");
        std::process::exit(1);
    }
}

fn process_command_init() {
    if ! nota::command_init() {
        info!("It was not possible to initialize NOTA folder, maybe this is a NOTA folder already");
        std::process::exit(1);
    }
    std::process::exit(0);
}

fn process_command_new(args: NewCommand) {
    assert_nota_folder();
    //if let Some(matches_new) = matches.subcommand_matches("new") {
    //    let new_nota_name = matches_new.value_of("NAME").unwrap();
    //    nota::command_new(Some(new_nota_name)); 
    //}
}

fn process_command_add(args: AddCommand){
    assert_nota_folder();

    nota::command_add(PathBuf::from(args.input));

    std::process::exit(0);
}

fn process_command_list(args: ListCommand){
    assert_nota_folder();

    nota::command_list(); 

    std::process::exit(0);
}

fn process_command_update(args: UpdateCommand){
    debug!("Update Command");
    assert_nota_folder();

    nota::command_update(); 

    std::process::exit(0);
}

fn process_command_export(args: ExportCommand){

    //if let Some(matches_export) = matches.subcommand_matches("export") {
    //    let file = match matches_export.value_of("PATH") {
    //        Some(path) => Some(PathBuf::from(path)),
    //        None => None
    //    };
    //    let outfolder = match matches_export.value_of("outfolder") {
    //        Some(path) => PathBuf::from(path),
    //        None => {
    //            println!("FUCk");
    //            return
    //        }
    //    };
    //    nota::command_export(file, outfolder); 
    //    return
    //}
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

    //nota::demo();

    nota::init_envs();

    let subcommand = match opts.subcmd {
        Some(subcommand) => subcommand,
        None => return
    };

    match subcommand {
        SubCommand::Add(args) => {
            process_command_add(args);
        },
        SubCommand::Export(args) => {
            process_command_export(args);
        },
        SubCommand::Init(_args) => {
            process_command_init();
        },
        SubCommand::List(args) => {
            process_command_list(args);
        },
        SubCommand::New(args) => {
            process_command_new(args);
        },
        SubCommand::Update(args) => {
            process_command_update(args);
        }
    }

}
