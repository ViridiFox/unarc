use clap::{Parser, Subcommand};
use std::path::PathBuf;

mod formats;

/// Working with archives with one binary instead of many
#[derive(Debug, Parser)]
#[clap(author, version, about)]
struct Cli {
    #[clap(subcommand)]
    command: Command,
}

#[derive(Debug, Subcommand)]
enum Command {
    /// extract archive into target dir
    Extract {
        archive: PathBuf,
        /// defaults to a name derived from the archive
        target: Option<PathBuf>,
    },
    /// creates an archive containing the listed files
    Create {
        archive: PathBuf,
        files: Vec<PathBuf>,
    },
    /// list the files inside of an archive
    List { archive: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Extract { archive, target } => {
            let target = target.or_else(|| Some(archive.with_extension(""))).unwrap();

            let format_tool = formats::from_file(&archive)?;
            format_tool.extract(archive, target);
        }
        Command::Create { archive, files } => {
            let format_tool = formats::from_file(&archive)?;
            format_tool.create(archive, files);
        }
        Command::List { archive } => {
            let format_tool = formats::from_file(&archive)?;
            let list = format_tool.list(archive.clone())?;

            for entry in list {
                println!("{}", entry.to_string_lossy());
            }
        }
    }

    Ok(())
}
