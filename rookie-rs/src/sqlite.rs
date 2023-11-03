use rusqlite::{self, OpenFlags, Connection};
use url::Url;
use std::path::PathBuf;
use anyhow::{Result, anyhow};

pub fn connect(path: PathBuf) -> Result<Connection> {
    let flags = OpenFlags::SQLITE_OPEN_READ_ONLY | OpenFlags::SQLITE_OPEN_URI;
    let conn_str = format!("{}?mode=ro&immutable=1", Url::from_file_path(&path.canonicalize()?).or(Err(anyhow!("Error opening connection")))?);
    let connection = rusqlite::Connection::open_with_flags(&conn_str, flags)?;
    Ok(connection)
}
