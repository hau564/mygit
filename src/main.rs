use clap::{Parser, Subcommand};

pub(crate) mod commands;
pub(crate) mod objects;

#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    #[command(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// Doc comment
    Init,
    CatFile {
        #[clap(short = 'p')]
        pretty_print: bool,

        object_hash: String,
    },
}

fn main() -> anyhow::Result<()> {
    let args = Args::parse();
    match args.command {
        Command::Init => {
            commands::init::invoke()
        }
        Command::CatFile {
            pretty_print,
            object_hash,
        } => {
            commands::cat_file::invoke(pretty_print, &object_hash)?
        }
    }
    Ok(())
}