use std::path::PathBuf;
use crate::error::{Result, StorageError};

const APP_DIR: &str = "com.vspat.stark";

pub fn data_dir() -> Result<PathBuf> {
    let base = std::env::var_os("APPDATA")
        .map(PathBuf::from)
        .ok_or(StorageError::NoDataDir)?;
    Ok(base.join(APP_DIR))
}

pub fn db_path() -> Result<PathBuf> {
    Ok(data_dir()?.join("stark.db"))
}

pub fn backup_dir() -> Result<PathBuf> {
    Ok(data_dir()?.join("backups"))
}

pub fn ensure_dirs() -> Result<()> {
    for dir in [data_dir()?, backup_dir()?] {
        std::fs::create_dir_all(&dir).map_err(|source| StorageError::Io {
            path: dir,
            source,
        })?;
    }
    Ok(())
}