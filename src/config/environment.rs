use std::{env, path::PathBuf};

pub const CONFIG_FILE_NAME: &str = "config.yaml";
pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

pub fn config_file() -> Result<PathBuf> {
    let dir = match exe_dir()? {
        Some(dir) => dir,
        None => platform_specific_config_dir()?,
    };

    if !dir.exists() {
        std::fs::create_dir_all(dir.as_path())?;
    };

    Ok(dir.join(CONFIG_FILE_NAME))
}

pub fn platform_specific_config_dir() -> Result<PathBuf> {
    let dir = dirs_next::config_dir();
    match dir {
        Some(dir) => return Ok(dir.join("raiti").join(CONFIG_FILE_NAME)),
        None => Err("Could not find config directory".into()),
    }
}

fn exe_dir() -> Result<Option<PathBuf>> {
    let exe = env::current_exe()?;
    let dir = exe
        .parent()
        .ok_or("Could not get parent directory of executable")?;

    if !dir.join(CONFIG_FILE_NAME).is_file() {
        return Ok(None);
    }
    Ok(Some(dir.to_path_buf()))
}
