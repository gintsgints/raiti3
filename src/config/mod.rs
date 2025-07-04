use serde::{Deserialize, Serialize};

mod environment;
pub use environment::{assets_dir, config_file, data_dir};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Deserialize, Serialize)]
pub struct Config {
    pub current_keyboard_layout: String,
    pub current_lesson: String,
    pub current_page: usize,
    pub current_exercise: usize,
}

impl Config {
    pub fn new() -> Self {
        Config {
            ..Config::default()
        }
    }

    pub fn load() -> Result<Self> {
        let config_file = config_file()?;
        if config_file.exists() {
            let config: Config = serde_yaml::from_str(&std::fs::read_to_string(config_file)?)?;
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

impl Default for Config {
    fn default() -> Self {
        Config {
            current_keyboard_layout: "querty".to_string(),
            current_lesson: String::new(),
            current_page: 0,
            current_exercise: 0,
        }
    }
}
