use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Default, uniffi::Enum)]
pub enum Languages {
    #[default]
    English,
}
