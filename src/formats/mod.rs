use std::path::{Path, PathBuf};

mod tar;
mod zip;

pub trait Format {
    /// extract the `archive` into the `target_dir`
    fn extract(self: Box<Self>, archive: PathBuf, target_dir: PathBuf) -> anyhow::Result<()>;
    /// list the contents of the `archive`
    fn list(self: Box<Self>, archive: PathBuf) -> anyhow::Result<Vec<PathBuf>>;
    /// create an `archive` containing the given `files` or add them if the archive already exists
    fn create(self: Box<Self>, archive: PathBuf, files: Vec<PathBuf>) -> anyhow::Result<()>;
}

pub fn from_file(file_name: impl AsRef<Path>) -> anyhow::Result<Box<dyn Format>> {
    let file_name = file_name.as_ref();

    let tmp = file_name.with_extension("");
    let sec_last_ext = tmp.extension().and_then(|s| s.to_str());

    let last_ext = file_name.extension().and_then(|s| s.to_str());

    match (sec_last_ext, last_ext) {
        (_, Some("zip")) => Ok(Box::new(zip::Format::new())),
        (None, Some("tar")) => Ok(Box::new(tar::Format::new())),
        (Some("tar"), _) => Ok(Box::new(tar::Format::new())),

        _ => Err(anyhow::anyhow!("unknown extension")),
    }
}
