use crate::error::AudifyError;
use extractous::Extractor;
// use extractous::PdfParserConfig;

pub(super) fn extract_pdf_source(pdf_path: &str) -> Result<String, AudifyError> {
    let extractor = Extractor::new();
    let (content, metadata) = extractor
        .extract_file_to_string(pdf_path)
        .map_err(AudifyError::TextExtractError)?;
    println!("{}", content);
    println!("{:?}", metadata);

    Ok(content)
}
