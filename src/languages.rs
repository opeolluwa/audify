// #[derive(uniffi::Enum)]
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default)]
pub enum Languages {
    #[default]
    English,
}
