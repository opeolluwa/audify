use thiserror::Error;

#[derive(Error, Debug)]
pub enum AudifyError {
    #[error("Error encoding source")]
    AudioEndoingError,
    #[error("PDF extract error")]
    TextExtractError,
}
