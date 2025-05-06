use serde::{Deserialize, Serialize};

use crate::{error::SynthesisError, languages::Languages};

#[derive(Debug, Serialize, Deserialize)]
pub struct Audify {
    language: Languages,
    model_path: String,
    export_path: String,
    source: String,
}

impl Default for Audify {
    fn default() -> Self {
        Self {
            language: Default::default(),
            model_path: Default::default(),
            export_path: Default::default(),
            source: Default::default(),
        }
    }
}

impl Audify {
    pub fn new(export_path: &str) -> Self {
        Self {
            export_path: export_path.to_string(),
            ..Default::default()
        }
    }

    // source: source.to_string()
    pub fn synthesize(source: &str) -> Result<(), SynthesisError> {
        Ok(())
    }
}
