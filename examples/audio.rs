use audify_rs::{core::Audify, error::AudifyError};

/**
 *
 * just download-models
 * cargo run --example audio
 */
fn main() -> Result<(), AudifyError> {
    let config_path = "en_US-libritts_r-medium.onnx.json";
    let source_text = "hey man";
    let pdf_path = "test.pdf";
    let raw_export_path = "out_raw.wav";
    let pdf_export_path = "out_pdf.wav";

    let audify_rs = Audify::new(config_path);

    audify_rs.synthesize_text(source_text, raw_export_path)?;
    audify_rs.synthesize_pdf(pdf_path, pdf_export_path)?;

    println!("done");

    Ok(())
}
