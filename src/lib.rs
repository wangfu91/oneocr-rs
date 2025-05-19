mod bounding_box;
mod errors;
mod image;
mod ocr_engine;
mod ocr_line;
mod ocr_result;
mod ocr_word;

use std::ffi::c_char;

// Re-export the public structs for easier access by users of your library
pub use bounding_box::BoundingBox;
pub use errors::OneOcrError;
pub(crate) use image::Image;
pub use ocr_engine::OcrEngine;
pub use ocr_line::OcrLine;
pub use ocr_result::OcrResult;
pub use ocr_word::OcrWord;

pub(crate) const ONE_OCR_MODEL_FILE_NAME: &str = "oneocr.onemodel";
pub(crate) const ONE_OCR_MODEL_KEY: &str = r#"kj)TGtrK>f]b[Piow.gU+nC@s""""""4"#;

pub(crate) type CreateOcrInitOptions = unsafe extern "C" fn(*mut i64) -> i64;
pub(crate) type OcrInitOptionsSetUseModelDelayLoad = unsafe extern "C" fn(i64, c_char) -> i64;
pub(crate) type CreateOcrPipeline = unsafe extern "C" fn(
    model_path: *const c_char,
    key: *const c_char,
    ctx: i64,
    pipeline: *mut i64,
) -> i64;

pub(crate) type CreateOcrProcessOptions = unsafe extern "C" fn(*mut i64) -> i64;
pub(crate) type OcrProcessOptionsGetMaxRecognitionLineCount =
    unsafe extern "C" fn(i64, *mut i64) -> i64;
pub(crate) type OcrProcessOptionsSetMaxRecognitionLineCount = unsafe extern "C" fn(i64, i64) -> i64;
pub(crate) type OcrProcessOptionsGetResizeResolution =
    unsafe extern "C" fn(i64, *mut i64, *mut i64) -> i64;
pub(crate) type OcrProcessOptionsSetResizeResolution = unsafe extern "C" fn(i64, i64, i64) -> i64;

/// Image resolution must be great than 50*50, otherwise it will return error code 3.
/// For images with a resolution less than 50*50, you should manually scale up the image first.
pub(crate) type RunOcrPipeline = unsafe extern "C" fn(i64, *const Image, i64, *mut i64) -> i64;

pub(crate) type GetImageAngle = unsafe extern "C" fn(i64, *mut f32) -> i64;

pub(crate) type GetOcrLineCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub(crate) type GetOcrLine = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;
pub(crate) type GetOcrLineContent = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub(crate) type GetOcrLineBoundingBox = unsafe extern "C" fn(i64, *mut *const BoundingBox) -> i64;
pub(crate) type GetOcrLineStyle = unsafe extern "C" fn(i64, *mut i32, *mut f32) -> i64;

pub(crate) type GetOcrLineWordCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub(crate) type GetOcrWord = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;
pub(crate) type GetOcrWordContent = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub(crate) type GetOcrWordBoundingBox = unsafe extern "C" fn(i64, *mut *const BoundingBox) -> i64;
pub(crate) type GetOcrWordConfidence = unsafe extern "C" fn(i64, *mut f32) -> i64;

pub(crate) type ReleaseOcrResult = unsafe extern "C" fn(i64);
pub(crate) type ReleaseOcrInitOptions = unsafe extern "C" fn(i64);
pub(crate) type ReleaseOcrPipeline = unsafe extern "C" fn(i64);
pub(crate) type ReleaseOcrProcessOptions = unsafe extern "C" fn(i64);

/// A macro to load a symbol from the library.
/// This macro takes three arguments:
/// - `$library`: The library from which to load the symbol.
/// - `$var_name`: The name of the variable to store the loaded symbol.
/// - `$symbol_name_type`: The type of the symbol to load.
///
/// This macro is used to simplify the process of loading symbols from the library.
/// It helps to avoid repetitive code and makes the code cleaner and more readable.
macro_rules! load_symbol {
    ($library:expr, $var_name:ident, $symbol_name_type:ident) => {
        let $var_name: libloading::Symbol<$symbol_name_type> =
            unsafe { $library.get(stringify!($symbol_name_type).as_bytes())? };
    };
}

pub(crate) use load_symbol;

/// A macro to attempt to load a symbol and call it, for use in contexts like `drop`
/// Errors during symbol loading are logged to stderr, and the call is skipped.
/// - `$library`: The library instance.
/// - `$symbol_name_type`: The type of the FFI function (also used as the symbol name).
/// - $($arg:expr),*`: The arguments to pass to the function if loaded successfully.
macro_rules! release_ocr_resource {
    ($library:expr, $symbol_name_type:ident, $($arg:expr),* ) => {
        match unsafe { $library.get::<$symbol_name_type>(stringify!($symbol_name_type).as_bytes()) } {
            Ok(func_symbol) => {
                unsafe { func_symbol($($arg),*) };
            }
            Err(_) => {
                // Ignore the error, as this is best effort
                // and we are in the drop context.
            }
        }
    };
}

pub(crate) use release_ocr_resource;

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
