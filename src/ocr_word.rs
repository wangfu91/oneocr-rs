use crate::bounding_box::BoundingBox;
use crate::errors::OneOcrError;
use serde::Serialize;
use std::ffi::{CStr, c_char, c_void};
use std::ptr;

// FFI types used by this module
use crate::ffi::{GetOcrWordBoundingBox, GetOcrWordConfidence, GetOcrWordContent, RawBBox};
// Macros
use crate::check_ocr_call;

/// The `OcrWord` struct represents a word recognized by the OCR engine.
/// It contains the recognized word, its confidence score, and its bounding box.
#[derive(Debug, Serialize)]
pub struct OcrWord {
    pub text: String,
    pub confidence: f32,
    pub bounding_box: BoundingBox,
}

impl OcrWord {
    pub(crate) fn new(word_handle: *mut c_void) -> Result<Self, OneOcrError> {
        let mut word_content: *const c_char = ptr::null();
        check_ocr_call!(
            unsafe { GetOcrWordContent(word_handle, &mut word_content) },
            "Failed to get word content"
        );
        let word_content_cstr = unsafe { CStr::from_ptr(word_content) };
        let word_content_str = word_content_cstr.to_string_lossy().to_string();

        let mut bounding_box_ptr: *const RawBBox = ptr::null();
        check_ocr_call!(
            unsafe { GetOcrWordBoundingBox(word_handle, &mut bounding_box_ptr) },
            "Failed to get word bounding box"
        );

        if bounding_box_ptr.is_null() {
            return Err(OneOcrError::OcrApiError {
                result: -1,
                message: "GetOcrWordBoundingBox returned a null pointer.".to_string(),
            });
        }

        let raw_bbox = unsafe { ptr::read(bounding_box_ptr) };
        let bounding_box = BoundingBox::new(raw_bbox);

        let mut confidence: f32 = 0.0;
        check_ocr_call!(
            unsafe { GetOcrWordConfidence(word_handle, &mut confidence) },
            "Failed to get word confidence"
        );

        Ok(Self {
            text: word_content_str,
            confidence,
            bounding_box,
        })
    }
}
