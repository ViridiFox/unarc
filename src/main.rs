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
    #[clap(alias = "x")]
    /// extract archive [alias: x]
    Extract {
        archive: PathBuf,
        /// defaults to a name derived from the archive
        target: Option<PathBuf>,
    },
    #[clap(alias = "c")]
    /// creates an archive containing the listed files
    /// or add the files to it, if it already exists [alias: c]
    Create {
        archive: PathBuf,
        files: Vec<PathBuf>,
    },
    #[clap(alias = "l")]
    /// list the files inside of an archive [alias: l]
    List { archive: PathBuf },
}

fn main() -> anyhow::Result<()> {
    let args = Cli::parse();

    match args.command {
        Command::Extract { archive, target } => {
            let (format_tool, target_derive) = formats::from_file(&archive)?;

            let target = target.unwrap_or(target_derive);

            format_tool.extract(archive, target)?;
        }
        Command::Create { archive, files } => {
            let (format_tool, _) = formats::from_file(&archive)?;
            format_tool.create(archive, files)?;
        }
        Command::List { archive } => {
            let (format_tool, _) = formats::from_file(&archive)?;
            let list = format_tool.list(archive.clone())?;

            for entry in list {
                println!("{}", entry.to_string_lossy());
            }
        }
    }

    Ok(())
}
