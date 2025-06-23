// Define a custom error type named OneOcrError using thiserror crate for better error handling
#[derive(Debug, thiserror::Error)]
pub enum OneOcrError {
    #[error("Failed to open image: {0}")]
    ImageOpenError(#[from] image::ImageError),

    #[error("Image format not supported: {0}")]
    ImageFormatError(String),

    #[error("Failed to load model file: {0}")]
    ModelFileLoadError(String),

    #[error("Invalid model decryption key: {0}")]
    InvalidModelKey(String),

    #[error("Failed to run OCR API (code: {result}): {message}")]
    OcrApiError { result: i32, message: String },

    #[error("Other error: {0}")]
    Other(String),
}
