use std::fs;

use serde::Deserialize;

use crate::config::data_dir;

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Default, Deserialize)]
pub enum Key {
    /// A key with an established name.
    Named(String),

    /// A key string that corresponds to the character typed by the user, taking into account the
    /// user’s current locale setting, and any system-level keyboard mapping overrides that are in
    /// effect.
    Character(String),

    /// An unidentified key.
    #[default]
    Unidentified,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub enum Location {
    /// The standard group of keys on the keyboard.
    #[default]
    Standard,
    /// The left side of the keyboard.
    Left,
    /// The right side of the keyboard.
    Right,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct KeySpec {
    pub key: Key,
    #[serde(default = "default_location")]
    pub location: Location,
    // Key width ratio against calculated key width
    // Should be specified if key is larger than usual keys
    #[serde(default = "default_width_ratio")]
    pub width_ratio: f32,
    pub label1: String,
    #[serde(default)]
    pub label2: String,
}

fn default_width_ratio() -> f32 {
    1.0
}

fn default_location() -> Location {
    Location::Standard
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Row {
    pub keys: Vec<KeySpec>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct KeyboardConfig {
    cols_for_keys: f32,
    space_between_keys: f32,
    keyboard_side_padding: f32,
    key_text_top_pad: f32,
    key_text_left_pad: f32,
    rows: Vec<Row>,
}

impl KeyboardConfig {
    pub fn load(layout: &str) -> Result<Self> {
        let content = fs::read_to_string(
            data_dir()?
                .join("keyboards")
                .join(format!("{}.yaml", layout)),
        )?;
        Ok(serde_yaml::from_str(&content)?)
    }
}
