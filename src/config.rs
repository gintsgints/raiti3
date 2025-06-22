use serde::{Deserialize, Serialize};

mod environment;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub current_keyboard_layout: String,
}

impl Config {
    pub fn new() -> Self {
        Config {
            current_keyboard_layout: "querty".to_string(),
        }
    }

    pub fn load() -> Result<Self> {
        let config_file = environment::config_file()?;
        if config_file.exists() {
            let config: Config = serde_yaml::from_str(
                &std::fs::read_to_string(config_file)?,
            )?;
            Ok(config)
        } else {
            Ok(Self::new())
        }
    }

    // TODO
    // pub fn save(&self) -> Result<()> {
    //     let config_file = environment::config_file()?;
    //     std::fs::write(config_file, &self.current_keyboard_layout)?;
    //     Ok(())
    // }
}
