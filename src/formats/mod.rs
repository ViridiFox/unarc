use std::path::{Path, PathBuf};

mod bzip2;
mod gzip;
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

pub fn from_file(file_name: impl AsRef<Path>) -> anyhow::Result<(Box<dyn Format>, PathBuf)> {
    let file_name = file_name.as_ref();

    let tmp = file_name.with_extension("");
    let sec_last_ext = tmp
        .extension()
        .and_then(|s| s.to_str().map(|s| s.to_string()));
    let target_1 = tmp.clone();
    let target_2 = tmp.with_extension("");

    let last_ext = file_name.extension().and_then(|s| s.to_str());

    match (sec_last_ext.as_deref(), last_ext) {
        (Some("tar"), Some("bz2")) => Ok((Box::new(tar::Format::new()), target_2)),
        (Some("tar"), Some("gz")) => Ok((Box::new(tar::Format::new()), target_2)),

        (_, Some("zip")) => Ok((Box::new(zip::Format::new()), target_1)),
        (_, Some("tar")) => Ok((Box::new(tar::Format::new()), target_1)),
        (_, Some("tbz2")) => Ok((Box::new(tar::Format::new()), target_1)),
        (_, Some("tgz")) => Ok((Box::new(tar::Format::new()), target_1)),
        (_, Some("bz2")) => Ok((Box::new(bzip2::Format::new()), target_1)),
        (_, Some("gz")) => Ok((Box::new(gzip::Format::new()), target_1)),

        _ => Err(anyhow::anyhow!("unknown extension")),
    }
}
