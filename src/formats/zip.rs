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
        let target_dir = target_dir
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;

        Command::new("unzip")
            .arg(archive)
            .arg("-d")
            .arg(target_dir)
            .status()?;

        Ok(())
    }

    fn list(self: Box<Self>, archive: PathBuf) -> anyhow::Result<Vec<PathBuf>> {
        let archive = archive
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;

        let output = Command::new("unzip").arg("-l").arg(archive).output()?;

        Ok(String::from_utf8(output.stdout)?
            .lines()
            .skip(3)
            .flat_map(|line| {
                let line = line
                    .split(' ')
                    .filter(|s| !s.is_empty())
                    .collect::<Vec<_>>();

                if line.len() != 4 || line.is_empty() {
                    None
                } else {
                    Some(PathBuf::from(line.last().unwrap()))
                }
            })
            .collect())
    }

    fn create(self: Box<Self>, archive: PathBuf, files: Vec<PathBuf>) -> anyhow::Result<()> {
        let archive = archive
            .to_str()
            .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;

        let mut command = Command::new("zip");
        command.arg("-r").arg(archive);

        for file in files {
            let file = file
                .to_str()
                .ok_or_else(|| anyhow::anyhow!("valid utf-8 name"))?;

            command.arg(file);
        }

        command.status()?;

        Ok(())
    }
}
