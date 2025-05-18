use oneocr_rs::errors::OneOcrError;
use std::path::Path;

fn main() -> Result<(), OneOcrError> {
    // Create a new OCR instance
    let ocr_engine = oneocr_rs::OcrEngine::new()?;

    // Default max recognition line count is 100, range is 0-1000.
    let max_line_conunt = ocr_engine.get_max_recognition_line_count()?;
    println!("Default max recognition line count: {:#?}", max_line_conunt);

    // Default resize resolution is 1152*768.
    // The `resize resolution` defines the maximum dimensions to which an image will be automatically scaled internally before OCR processing.
    // It’s a performance and accuracy trade-off rather than a restriction on the original image’s resolution.
    let (width, height) = ocr_engine.get_resize_resolution()?;
    println!("Default resize resolution: {}*{}", width, height);

    // Set to the max recognition line count possible.
    ocr_engine.set_max_recognition_line_count(1000)?;

    let image_path = Path::new("./target/4k_screenshot.png");

    // Perform OCR on an image
    let include_word_level_detail = true;
    let ocr_result = ocr_engine.run(image_path, include_word_level_detail)?;

    println!("Image angle: {}", ocr_result.image_angle);

    for line in &ocr_result.lines {
        println!("\n");
        println!("Line: {}", line.text);
        println!("{:?}", line.bounding_box);

        let (handwriting, confidence) = line.get_line_style()?;
        println!(
            "Line style: handwrtten: {}, handwritten confidence: {}",
            handwriting, confidence
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
