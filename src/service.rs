use std::path::{Path, PathBuf};

use anyhow::Context;
use log::info;
use thiserror::Error;

#[derive(Debug, Error)]
pub enum ServiceError {
    #[error("permission denied - have you tried sudo?")]
    PermissionError,
    #[error("systemd is required to write service files")]
    SystemdNotFound,
    #[error("failed to write file")]
    FileWrite(#[source] std::io::Error),
    #[error("failed to create directory")]
    DirectoryCreation(#[source] std::io::Error),
    #[error("unknown error: {0}")]
    Unknown(String),
}

fn validate() -> anyhow::Result<()> {
    match std::fs::read_to_string("/proc/1/comm") {
        Ok(contents) => {
            if contents.trim() != "systemd" {
                return Err(ServiceError::SystemdNotFound.into());
            }
        }
        Err(e) => return Err(ServiceError::Unknown(e.to_string()).into()),
    }

    Ok(())
}

pub fn write<T>(data: &[u8], path: T) -> anyhow::Result<()>
where
    T: AsRef<Path>,
{
    validate()?;

    let base_path = Path::new("/etc/systemd/system");
    let write_path = PathBuf::from(base_path.join(path));

    if !std::fs::exists(&base_path).context(format!("Failed to read {}", &base_path.display()))? {
        std::fs::create_dir_all(&base_path).map_err(|e| match e.kind() {
            std::io::ErrorKind::PermissionDenied => ServiceError::PermissionError,
            _ => ServiceError::DirectoryCreation(e),
        })?;
    }

    std::fs::write(&write_path, data).map_err(|e| match e.kind() {
        std::io::ErrorKind::PermissionDenied => ServiceError::PermissionError,
        _ => ServiceError::FileWrite(e),
    })?;

    info!("Wrote {} bytes to {}", data.len(), write_path.display());

    Ok(())
}
