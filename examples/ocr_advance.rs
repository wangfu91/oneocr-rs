use oneocr_rs::{OcrEngine, OneOcrError};
use std::path::Path;

fn main() -> Result<(), OneOcrError> {
    let image_path = Path::new("./target/4k_screenshot.png");

    // Create a new OCR instance
    let ocr_engine = OcrEngine::new()?;

    // Set to the max recognition line count possible.
    ocr_engine.set_max_recognition_line_count(1000)?;

    // Perform OCR on an image
    let include_word_level_detail = true;
    let ocr_result = ocr_engine.run(image_path, include_word_level_detail)?;

    // Print the OCR result
    println!("Image angle: {}", ocr_result.image_angle);

    for line in &ocr_result.lines {
        println!();
        println!("Line: {}", line.text);
        println!("{:?}", line.bounding_box);

        let (handwritten, confidence) = line.get_line_style()?;
        println!(
            "Line style: handwritten: {}, handwritten style confidence: {}",
            handwritten, confidence
        );

        if let Some(words) = &line.words {
            for word in words {
                print!("Word: [{}, {}]\t", word.text, word.confidence);
                println!("{:?}", word.bounding_box);
            }
        }
    }

    Ok(())
}
