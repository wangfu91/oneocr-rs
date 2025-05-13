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
    // Set word_level_detail to false to get the result back faster.
    let include_word_level_detail = true;
    let ocr_result = ocr_engine.run(image_path, include_word_level_detail)?;

    for line in &ocr_result.lines {
        println!("\n");
        println!("Line: {}", line.content);
        println!("{:?}", line.bounding_box);

        let (handwriting, confidence) = line.get_line_style()?;
        println!(
            "Style: handwrting: {}, confidence: {}",
            handwriting, confidence
        );

        if let Some(words) = &line.words {
            for word in words {
                print!("Word: [{}, {}]\t", word.content, word.confidence);
                //println!("{:?}", word.bounding_box);
            }
        }
    }

    Ok(())
}
