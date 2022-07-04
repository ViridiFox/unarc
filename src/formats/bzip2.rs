use std::fs::File;
use std::io::Write;
use std::path::PathBuf;
use std::process::Command;

pub struct Format;

impl Format {
    pub fn new() -> Format {
        Format {}
    }
}

impl super::Format for Format {
    fn extract(self: Box<Self>, archive: PathBuf, target_file: PathBuf) -> anyhow::Result<()> {
        let archive = archive
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;

        let decompressed = Command::new("bzip2")
            .arg("-c")
            .arg("-d")
            .arg(archive)
            .output()?;

        let mut outfile = File::create(target_file)?;
        outfile.write_all(&decompressed.stdout)?;
        outfile.sync_all()?;

        Ok(())
    }

    fn list(self: Box<Self>, _archive: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
        anyhow::bail!("can't list the contained files of a compressed file");
    }

    fn create(self: Box<Self>, archive: PathBuf, files: Vec<PathBuf>) -> anyhow::Result<()> {
        if files.len() != 1 {
            anyhow::bail!("can only compress exactly 1 file");
        }

        let file = files[0]
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;

        let compressed = Command::new("bzip2")
            .arg("-c")
            .arg("-z")
            .arg(file)
            .output()?;

        let mut outfile = File::create(archive)?;
        outfile.write_all(&compressed.stdout)?;
        outfile.sync_all()?;

        Ok(())
    }
}
