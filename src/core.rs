use crate::{error::AudifyError, extractor::extract_pdf_source, languages::Languages};
use piper_rs::synth::PiperSpeechSynthesizer;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Debug, Serialize, Deserialize, uniffi::Object)]
pub struct Audify {
    /// language or model to use see https://github.com/rhasspy/piper/blob/master/VOICES.md
    language: Languages,
    /// where to put the generated audio file
    export_path: String,
    /// the text passed to the library
    source: String,
    /// path to <model_name>.onnx.json
    config_path: String,
    sid: i64,
}

impl Default for Audify {
    fn default() -> Self {
        Self {
            language: Default::default(),
            config_path: "".to_string(),
            export_path: ".".to_string(),
            source: Default::default(),
            sid: 50i64,
        }
    }
}

#[uniffi::export]
impl Audify {
    /// create a new Audify instance
    ///
    /// ### Example
    /// ```
    /// use audify_rs::error::AudifyError;
    /// use audify_rs::core::Audify;
    /// fn main() -> Result<(), AudifyError> {
    /// let config_path = "en_US-libritts_r-medium.onnx.json";
    ///  let source_text = "hey man";
    ///  let pdf_path = "test.pdf";
    ///  let raw_export_path = "out_raw.wav";
    ///  let pdf_export_path = "out_pdf.wav";
    ///
    ///   let audify_rs = Audify::new(config_path);
    ///  audify_rs.synthesize_text(source_text, raw_export_path)?;
    ///  audify_rs.synthesize_pdf(pdf_path, pdf_export_path)?;
    ///
    ///   println!("done");
    ///   Ok(())
    /// }
    /// ```
    ///
    #[uniffi::constructor]
    pub fn new(config_path: &str) -> Self {
        Self {
            config_path: config_path.to_string(),
            ..Default::default()
        }
    }

    /// generate audio from raw text
    pub fn synthesize_text(&self, source_text: &str, export_path: &str) -> Result<(), AudifyError> {
        let text = source_text.to_string();

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

    /// generate audio from PDF
    pub fn synthesize_pdf(&self, pdf_path: &str, export_path: &str) -> Result<(), AudifyError> {
        let text = extract_pdf_source(pdf_path)?;

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
