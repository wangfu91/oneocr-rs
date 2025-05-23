use oneocr_rs::{OcrEngine, OneOcrError};
use std::path::Path;

fn main() -> Result<(), OneOcrError> {
    let image_path = Path::new("./target/screenshot.png");

    // Create a new OCR instance
    let ocr_engine = OcrEngine::new()?;

    // Perform OCR on an image
    let ocr_result = ocr_engine.run(image_path, false)?;

    // Print the OCR lines and their bounding boxes
    for line in &ocr_result.lines {
        println!("{}, {}", line.text, line.bounding_box);
    }

    Ok(())
}
