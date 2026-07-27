use std::path::PathBuf;

#[derive(Debug, thiserror::Error)]
pub enum StorageError {
    #[error("database error: {0}")]
    Sqlite(#[from] rusqlite::Error),

    #[error("io error at {path}: {source}")]
    Io {
        path: PathBuf,
        #[source]
        source: std::io::Error,
    },

    #[error("could not determine application data directory")]
    NoDataDir,

    #[error("database schema version {found} is newer than supported version {supported}")]
    SchemaTooNew { found: i32, supported: i32 },
}

pub type Result<T> = std::result::Result<T, StorageError>;