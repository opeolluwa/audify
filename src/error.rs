use piper_rs::PiperError;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudifyError {
    #[error("Error encoding source")]
    AudioEndoingError,
    #[error("PDF extract error: {0}")]
    TextExtractError(extractous::Error),
    #[error("Model parse error: {0}")]
    ModelLoadError(PiperError),
    #[error("Error Initializing PiperSpeechSynthesizer: {0}")]
    PiperSpeechSynthesizerInitializationError(PiperError),
    #[error("File Synthesize Error: {0}")]
    FileSynthesizeError(PiperError),
}
