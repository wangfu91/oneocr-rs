use crate::errors::OneOcrError;
use crate::ocr_line::OcrLine;
use libloading::Library;
use serde::Serialize;

// FFI types
use crate::ffi::{GetImageAngle, GetOcrLine, GetOcrLineCount, ReleaseOcrResult};
// Macros
use crate::{check_ocr_call, load_symbol, release_ocr_resource};

/// The `OcrResult` struct represents the result of an OCR operation.
/// It contains the recognized text lines, their bounding boxes, and the image angle.
#[derive(Debug, Serialize)]
pub struct OcrResult<'a> {
    #[serde(skip_serializing)]
    lib: &'a Library,
    #[serde(skip_serializing)]
    result_handle: i64,
    pub lines: Vec<OcrLine<'a>>,
    pub image_angle: f32,
}

impl<'a> OcrResult<'a> {
    pub(crate) fn new(
        lib: &'a Library,
        result_handle: i64,
        word_level_detail: bool,
    ) -> Result<Self, OneOcrError> {
        load_symbol!(lib, get_ocr_line_count, GetOcrLineCount);
        load_symbol!(lib, get_ocr_line, GetOcrLine);
        load_symbol!(lib, get_image_angle, GetImageAngle);

        let mut line_count: i64 = 0;
        check_ocr_call!(
            unsafe { get_ocr_line_count(result_handle, &mut line_count) },
            "Failed to get line count"
        );
        let mut lines = Vec::with_capacity(line_count as usize);
        for i in 0..line_count {
            let mut line: i64 = 0;
            check_ocr_call!(
                unsafe { get_ocr_line(result_handle, i, &mut line) },
                "Failed to get line"
            );
            let ocr_line = OcrLine::new(lib, line, word_level_detail)?;
            lines.push(ocr_line);
        }
        let mut angle: f32 = 0.0;
        check_ocr_call!(
            unsafe { get_image_angle(result_handle, &mut angle) },
            "Failed to get image angle"
        );

        Ok(Self {
            lib,
            result_handle,
            lines,
            image_angle: angle,
        })
    }
}

impl Drop for OcrResult<'_> {
    fn drop(&mut self) {
        release_ocr_resource!(self.lib, ReleaseOcrResult, self.result_handle);
    }
}
