use oneocr_rs::errors::OneOcrError;
use std::path::Path;

fn main() -> Result<(), OneOcrError> {
    // Create a new OCR instance
    let ocr = oneocr_rs::OcrEngine::new()?;

    let image_path = Path::new("./target/snapshot_01.PNG");

    // Perform OCR on an image
    // Set word_level_detail to false to get the result quickly.
    let ocr_result = ocr.run(image_path, false)?;

    // Print the result
    println!("{:#?}", ocr_result.lines);

    Ok(())
}
