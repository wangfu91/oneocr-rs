/// A simple width×height pair.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Resolution {
    pub width: i32,
    pub height: i32,
}

/// Configuration for OCR processing behavior.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct OcrOptions {
    /// The maximum number of lines that can be recognized.
    /// Default is 100, range is 0-1000.
    pub max_recognition_line_count: i32,

    /// The maximum internal resize resolution (width, height).
    ///
    /// The `resize resolution` defines the maximum dimensions to which an image will be automatically scaled internally before OCR processing.
    /// It’s a performance and accuracy trade-off rather than a restriction on the original image’s resolution.
    ///
    /// The default and maximum resolution is (1152, 768).
    pub resize_resolution: Resolution,

    /// Whether to include word-level details in the result.
    /// If `true`, the result will contain bounding boxes and confidence scores for individual words.
    /// If `false`, only line-level information will be available.
    pub include_word_level_details: bool,
}

impl Default for OcrOptions {
    fn default() -> Self {
        OcrOptions {
            max_recognition_line_count: 100,
            resize_resolution: Resolution {
                width: 1152,
                height: 768,
            },
            include_word_level_details: false,
        }
    }
}
