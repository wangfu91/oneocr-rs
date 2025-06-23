mod bounding_box;
mod errors;
mod ffi;
mod ocr_engine;
mod ocr_line;
mod ocr_result;
mod ocr_word;

// Re-export the public structs for easier access
pub use bounding_box::BoundingBox;
pub use bounding_box::Point;
pub use errors::OneOcrError;
pub use ocr_engine::OcrEngine;
pub use ocr_line::OcrLine;
pub use ocr_result::OcrResult;
pub use ocr_word::OcrWord;

pub(crate) const ONE_OCR_MODEL_FILE_NAME: &str = "oneocr.onemodel";
pub(crate) const ONE_OCR_MODEL_KEY: &str = r#"kj)TGtrK>f]b[Piow.gU+nC@s""""""4"#;

/// A macro to check the result of an OCR call and return an error if it fails.
/// This macro takes an expression `$call` and an error message `$err_msg`.
/// If the result of `$call` is not 0, it returns an `OneOcrError::OcrApiError` error with the provided message.
/// This macro is used to simplify error handling in the OCR engine methods.
/// It helps to avoid repetitive error checking code and makes the code cleaner and more readable.
macro_rules! check_ocr_call {
    ($call:expr, $err_msg:literal) => {
        let res = $call;
        if res != 0 {
            return Err($crate::errors::OneOcrError::OcrApiError {
                // Use $crate for items from the macro's own crate
                result: res,
                message: $err_msg.to_string(),
            });
        }
    };
}

pub(crate) use check_ocr_call;
