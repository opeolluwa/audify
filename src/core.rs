use crate::{error::AudifyError, extractor::extract_pdf_source, languages::Languages};
use piper_rs::synth::PiperSpeechSynthesizer;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize)]
pub struct Audify {
    language: Languages,
    model_path: String,
    export_path: String,
    source: String,
    config_path: String,
    sid: i64,
}

impl Default for Audify {
    fn default() -> Self {
        Self {
            language: Default::default(),
            model_path: "en_US-libritts_r-medium.onnx.json".to_string(),
            config_path: "en_US-libritts_r-medium.onnx.json".to_string(),
            export_path: ".".to_string(),
            source: Default::default(),
            sid: 50i64,
        }
    }
}

impl Audify {
    pub fn new() -> Self {
        Self {
            // export_path: export_path.to_string(),
            ..Default::default()
        }
    }

    // source: source.to_string()
    pub fn synthesize(&self, source_path: &str, export_path: &str) -> Result<(), AudifyError> {
        let text = extract_pdf_source(source_path)?;

        let model = piper_rs::from_config_path(Path::new(&self.config_path))
            .map_err(AudifyError::ModelLoadError)?;

        model.set_speaker(self.sid);

        let synth = PiperSpeechSynthesizer::new(model)
            .map_err(AudifyError::PiperSpeechSynthesizerInitializationError)?;
        synth
            .synthesize_to_file(Path::new(export_path), text, None)
            .map_err(AudifyError::FileSynthesizeError)?;
        Ok(())
    }
}
