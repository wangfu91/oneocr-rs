use image::Rgba;
use imageproc::drawing::draw_line_segment_mut;
use oneocr_rs::{OcrEngine, OneOcrError};
use std::path::Path;

// cargo run --example bbox_draw -- "/path/to/input/image.jpg" "/path/to/draw_output.jpg"

fn main() -> Result<(), OneOcrError> {
    let input_image_path = std::env::args()
        .nth(1)
        .unwrap_or("./assets/sample.jpg".to_string());
    let output_image_path = std::env::args()
        .nth(2)
        .unwrap_or("./target/bbox_draw_output.jpg".to_string());

    let input_image_path = Path::new(&input_image_path);

    let output_image_path = Path::new(&output_image_path);

    // Create a new OCR instance
    let ocr_engine = OcrEngine::new()?;

    // Set to the max recognition line count possible.
    ocr_engine.set_max_recognition_line_count(1000)?;

    // Perform OCR on an image
    let include_word_level_detail = true;
    let ocr_result = ocr_engine.run(input_image_path, include_word_level_detail)?;

    // Load the image
    let mut img = image::open(input_image_path)?;

    // Define colors for bounding boxes
    let line_color = Rgba([255u8, 0u8, 0u8, 255u8]); // Red for lines
    let word_color = Rgba([0u8, 255u8, 0u8, 255u8]); // Green for words

    for line in &ocr_result.lines {
        let line_bbox = line.bounding_box;

        // Draw the outline of the line bounding box
        let p1 = (line_bbox.top_left.x, line_bbox.top_left.y);
        let p2 = (line_bbox.top_right.x, line_bbox.top_right.y);
        let p3 = (line_bbox.bottom_right.x, line_bbox.bottom_right.y);
        let p4 = (line_bbox.bottom_left.x, line_bbox.bottom_left.y);

        draw_line_segment_mut(&mut img, p1, p2, line_color);
        draw_line_segment_mut(&mut img, p2, p3, line_color);
        draw_line_segment_mut(&mut img, p3, p4, line_color);
        draw_line_segment_mut(&mut img, p4, p1, line_color); // Close the polygon

        if let Some(words) = &line.words {
            for word in words {
                let word_bbox = word.bounding_box;

                // Draw the outline of the word bounding box
                let wp1 = (word_bbox.top_left.x, word_bbox.top_left.y);
                let wp2 = (word_bbox.top_right.x, word_bbox.top_right.y);
                let wp3 = (word_bbox.bottom_right.x, word_bbox.bottom_right.y);
                let wp4 = (word_bbox.bottom_left.x, word_bbox.bottom_left.y);

                draw_line_segment_mut(&mut img, wp1, wp2, word_color);
                draw_line_segment_mut(&mut img, wp2, wp3, word_color);
                draw_line_segment_mut(&mut img, wp3, wp4, word_color);
                draw_line_segment_mut(&mut img, wp4, wp1, word_color); // Close the polygon
            }
        }
    }

    // Save the new image
    img.save(output_image_path)?;

    println!(
        "Output image saved to: {}",
        output_image_path.to_string_lossy()
    );

    Ok(())
}
