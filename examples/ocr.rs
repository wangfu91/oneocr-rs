use oneocr_rs::OneOcrError;
use std::path::Path;

fn main() -> Result<(), OneOcrError> {
    // Create a new OCR instance
    let ocr = oneocr_rs::OcrEngine::new()?;

    let image_path = Path::new("snapshot_01.PNG");
    // Perform OCR on an image
    // Set word_level_detail to false to get the result quickly.
    let ocr_result = ocr.run(image_path, false)?;

    // Print the result
    let lines = ocr_result
        .lines
        .iter()
        .map(|line| line.content.clone())
        .collect::<Vec<_>>();
    println!("{:#?}", lines);

    Ok(())
}
