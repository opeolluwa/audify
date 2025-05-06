use thiserror::Error;

#[derive(Error, Debug)]
pub enum SynthesisError {
    #[error("Error encoding source")]
    AudioEndoingError,
}
