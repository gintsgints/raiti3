
use serde::Deserialize;


#[derive(Debug, Clone, Default, Deserialize)]
pub enum Align {
    Left,
    #[default]
    Center,
    Right,
}
