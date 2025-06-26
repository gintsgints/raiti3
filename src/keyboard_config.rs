use std::fs;

use serde::Deserialize;

use crate::{config::data_dir, Align};

pub type Error = Box<dyn std::error::Error>;
pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, Default, Deserialize)]
pub struct KeySpec {
    pub key_code: i32,
    // Key width ratio against calculated key width
    // Should be specified if key is larger than usual keys
    #[serde(default = "default_width_ratio")]
    pub width_ratio: f32,
    #[serde(default)]
    pub label1: String,
    #[serde(default)]
    pub label1_align: Align,
    #[serde(default)]
    pub label2: String,
    #[serde(default)]
    pub label2_align: Align,
}

fn default_width_ratio() -> f32 {
    1.0
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct Row {
    pub keys: Vec<KeySpec>,
}

#[derive(Debug, Clone, Default, Deserialize)]
pub struct KeyboardConfig {
    pub cols_for_keys: i16,
    pub space_between_keys: i16,
    pub keyboard_corner_curve: i16,
    pub keyboard_side_padding: i16,
    pub key_text_top_pad: i16,
    pub key_text_left_pad: i16,
    pub rows: Vec<Row>,
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
