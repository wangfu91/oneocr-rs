use oneocr_rs::{OcrEngine, OneOcrError};
use std::path::Path;

fn main() -> Result<(), OneOcrError> {
    // Create a new OCR instance
    let ocr_engine = OcrEngine::new()?;

    let image_path = Path::new("./target/screenshot.png");

    // Perform OCR on an image
    let ocr_result = ocr_engine.run(image_path, false)?;

    // Print the OCR result
    for line in &ocr_result.lines {
        println!("{}", line.text);
    }

    Ok(())
}
