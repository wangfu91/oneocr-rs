use crate::errors::OneOcrError;
use crate::ocr_line::OcrLine;
use serde::Serialize;
use std::ffi::c_void;
use std::ptr;

// FFI types
use crate::ffi::{GetImageAngle, GetOcrLine, GetOcrLineCount, ReleaseOcrResult};
// Macros
use crate::check_ocr_call;

/// The `OcrResult` struct represents the result of an OCR operation.
/// It contains the recognized text lines, their bounding boxes, and the image angle.
#[derive(Debug, Serialize)]
pub struct OcrResult {
    #[serde(skip_serializing)]
    result_handle: *mut c_void,
    pub lines: Vec<OcrLine>,
    pub image_angle: f32,
}

impl OcrResult {
    pub(crate) fn new(
        result_handle: *mut c_void,
        word_level_detail: bool,
    ) -> Result<Self, OneOcrError> {
        let mut line_count: i64 = 0;
        check_ocr_call!(
            unsafe { GetOcrLineCount(result_handle, &mut line_count) },
            "Failed to get line count"
        );
        let mut lines = Vec::with_capacity(line_count as usize);
        for i in 0..line_count {
            let mut line: *mut c_void = ptr::null_mut();
            check_ocr_call!(
                unsafe { GetOcrLine(result_handle, i, &mut line) },
                "Failed to get line"
            );
            let ocr_line = OcrLine::new(line, word_level_detail)?;
            lines.push(ocr_line);
        }
        let mut angle: f32 = 0.0;
        check_ocr_call!(
            unsafe { GetImageAngle(result_handle, &mut angle) },
            "Failed to get image angle"
        );

        Ok(Self {
            result_handle,
            lines,
            image_angle: angle,
        })
    }
}

impl Drop for OcrResult {
    fn drop(&mut self) {
        unsafe { ReleaseOcrResult(self.result_handle) };
    }
}
