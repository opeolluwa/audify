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
    pub fn new(export_path: &str) -> Self {
        Self {
            export_path: export_path.to_string(),
            ..Default::default()
        }
    }

    // source: source.to_string()
    pub fn synthesize(&self, source: &str) -> Result<(), AudifyError> {
        // let text = source.to_string();
        let text = extract_pdf_source("test3.pdf")?;

        let model = piper_rs::from_config_path(Path::new(&self.config_path)).unwrap();

        model.set_speaker(self.sid);

        let synth = PiperSpeechSynthesizer::new(model).unwrap();
        synth
            .synthesize_to_file(Path::new(&self.export_path), text, None)
            .unwrap();
        Ok(())
    }
}
