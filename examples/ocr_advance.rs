use oneocr_rs::{OcrEngine, OneOcrError};
use std::path::Path;

// cargo run --example ocr_advance -- "/path/to/input/image.png"

fn main() -> Result<(), OneOcrError> {
    let input_image_path = std::env::args()
        .nth(1)
        .unwrap_or("./assets/sample.jpg".to_string());

    let image_path = Path::new(&input_image_path);

    // Create a new OCR instance
    let ocr_engine = OcrEngine::new()?;

    // Set to the max recognition line count possible.
    ocr_engine.set_max_recognition_line_count(1000)?;

    // Perform OCR on an image
    let include_word_level_detail = true;
    let ocr_result = ocr_engine.run(image_path, include_word_level_detail)?;

    // Print the OCR result
    println!("Image angle: {:.2}", ocr_result.image_angle);

    for line in &ocr_result.lines {
        println!();
        println!("Line: {}", line.text);
        println!("{}", line.bounding_box);

        let (handwritten, confidence) = line.get_line_style()?;
        println!(
            "Line style: handwritten: {handwritten}, handwritten style confidence: {confidence}"
        );

        if let Some(words) = &line.words {
            for word in words {
                print!("Word: [{}, {:.2}]\t", word.text, word.confidence);
                println!("{}", word.bounding_box);
            }
        }
    }

    Ok(())
}
