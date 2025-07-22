use oneocr_rs::{OcrEngine, OneOcrError};
use std::path::Path;

// cargo run --example ocr_simple -- "/path/to/input/image.png"

fn main() -> Result<(), OneOcrError> {
    // Get the input image path from command line arguments or use a default sample image
    let input_image_path = std::env::args()
        .nth(1)
        .unwrap_or("./assets/sample.jpg".to_string());

    let image_path = Path::new(&input_image_path);

    // Create a new OCR instance
    let ocr_engine = OcrEngine::new()?;

    // Perform OCR on an image
    let ocr_result = ocr_engine.run(image_path.into())?;

    // Print the OCR lines.
    for line in &ocr_result.lines {
        println!("{}", line.text);
    }

    Ok(())
}
