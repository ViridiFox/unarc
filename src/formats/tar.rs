use std::path::PathBuf;
use std::process::Command;

pub struct Format;

impl Format {
    pub fn new() -> Format {
        Format {}
    }
}

impl super::Format for Format {
    fn extract(self: Box<Self>, archive: PathBuf, target_dir: PathBuf) -> anyhow::Result<()> {
        let archive = archive
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;
        let target_dir_s = target_dir
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;

        if !target_dir.is_dir() {
            std::fs::create_dir_all(&target_dir)?;
        }

        Command::new("tar")
            .arg("-xaf")
            .arg(archive)
            .arg(format!("-C={target_dir_s}"))
            .status()?;

        Ok(())
    }

    fn list(self: Box<Self>, archive: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
        let archive = archive
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;

        let output = Command::new("tar").arg("-tf").arg(archive).output()?;

        Ok(String::from_utf8(output.stdout)?
            .lines()
            .map(PathBuf::from)
            .collect())
    }

    fn create(self: Box<Self>, archive: PathBuf, files: Vec<PathBuf>) -> anyhow::Result<()> {
        let archive_s = archive
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;

        let mut cmd = Command::new("tar");

        if archive.exists() {
            cmd.arg("-Af");
        } else {
            cmd.arg("-cf");
        }

        cmd.arg(archive_s);

        for file in files {
            let file = file
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;

            cmd.arg(file);
        }

        cmd.status()?;

        Ok(())
    }
}
