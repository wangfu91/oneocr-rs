use crate::bounding_box::BoundingBox;
use crate::errors::OneOcrError;
use libloading::Library;
use serde::Serialize;
use std::ffi::{CStr, c_char};

// FFI types used by this module
use crate::{GetOcrWordBoundingBox, GetOcrWordConfidence, GetOcrWordContent};
// Macros
use crate::{check_ocr_call, load_symbol};

/// The `OcrWord` struct represents a word recognized by the OCR engine.
/// It contains the recognized word, its confidence score, and its bounding box.
#[derive(Debug, Serialize)]
pub struct OcrWord {
    pub text: String,
    pub confidence: f32,
    pub bounding_box: BoundingBox,
}

impl OcrWord {
    pub(crate) fn new(lib: &Library, word_handle: i64) -> Result<Self, OneOcrError> {
        load_symbol!(lib, get_ocr_word_content, GetOcrWordContent);
        load_symbol!(lib, get_ocr_word_bounding_box, GetOcrWordBoundingBox);
        load_symbol!(lib, get_ocr_word_confidence, GetOcrWordConfidence);

        let mut word_content: i64 = 0;
        check_ocr_call!(
            unsafe { get_ocr_word_content(word_handle, &mut word_content) },
            "Failed to get word content"
        );
        let word_content_cstr = unsafe { CStr::from_ptr(word_content as *const c_char) };
        let word_content_str = word_content_cstr.to_string_lossy().to_string();

        let mut bounding_box_ptr: *const BoundingBox = std::ptr::null();
        check_ocr_call!(
            unsafe { get_ocr_word_bounding_box(word_handle, &mut bounding_box_ptr) },
            "Failed to get word bounding box"
        );

        if bounding_box_ptr.is_null() {
            return Err(OneOcrError::OcrApiError {
                result: -1,
                message: "GetOcrWordBoundingBox returned a null pointer.".to_string(),
            });
        }
        let bounding_box = unsafe { std::ptr::read(bounding_box_ptr) };

        let mut confidence: f32 = 0.0;
        check_ocr_call!(
            unsafe { get_ocr_word_confidence(word_handle, &mut confidence) },
            "Failed to get word confidence"
        );

        Ok(Self {
            text: word_content_str,
            confidence,
            bounding_box,
        })
    }
}
