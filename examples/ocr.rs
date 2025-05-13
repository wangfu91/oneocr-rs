use oneocr_rs::errors::OneOcrError;
use std::path::Path;

fn main() -> Result<(), OneOcrError> {
    // Create a new OCR instance
    let ocr_engine = oneocr_rs::OcrEngine::new()?;

    // Default max recognition line count is 100, range is 0-1000.
    let max_line_conunt = ocr_engine.get_max_recognition_line_count()?;
    println!("Max line count: {:#?}", max_line_conunt);

    // Default resize resolution is 1152*768.
    let (width, height) = ocr_engine.get_resize_resolution()?;
    println!("Resize resolution: {}*{}", width, height);

    let image_path = Path::new("./target/snapshot_01.PNG");

    // Perform OCR on an image
    // Set word_level_detail to false to get the result quickly.
    let ocr_result = ocr_engine.run(image_path, false)?;

    // Print the result
    println!("{:#?}", ocr_result.lines);

    Ok(())
}
