use std::path::{Path, PathBuf};

const BASE_PATH: Path = Path::new("/etc/systemd/system");

pub fn write(data: &[u8], path: &Path) -> anyhow::Result<()> {
    let write_path = PathBuf::from(BASE_PATH.join(path));
    Ok(())
}
