// Define a custom error type named OneOcrError using thiserror crate for better error handling
#[derive(Debug, thiserror::Error)]
pub enum OneOcrError {
    #[error("Failed to open image: {0}")]
    ImageOpenError(#[from] image::ImageError),

    #[error("Image format not supported: {0}")]
    ImageFormatError(String),

    #[error("Failed to load library: {0}")]
    LibraryLoadError(#[from] libloading::Error),

    #[error("Failed to load model file: {0}")]
    ModelFileLoadError(String),

    #[error("Invalid model decryption key: {0}")]
    InvalidModelKey(String),

    #[error("Failed to run ocr API {result}, result: {message}")]
    OcrApiError { result: i64, message: String },

    #[error("Other error: {0}")]
    Other(String),
}
