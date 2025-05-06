use crate::error::AudifyError;

pub(super) fn extract_pdf_source(pdf_path: &str) -> Result<String, AudifyError> {

    let out = pdf_extract::extract_text(&pdf_path).unwrap();

    Ok(out)
}
