use crate::bounding_box::BoundingBox;
use crate::errors::OneOcrError;
use crate::ocr_word::OcrWord;
use libloading::Library;
use serde::Serialize;
use std::ffi::{CStr, c_char};

// FFI types
use crate::ffi::{
    GetOcrLineBoundingBox, GetOcrLineContent, GetOcrLineStyle, GetOcrLineWordCount, GetOcrWord,
    RawBBox,
};
// Macros
use crate::{check_ocr_call, load_symbol};

/// The `OcrLine` struct represents a line of text recognized by the OCR engine.
/// It contains the recognized text, its bounding box, and optionally the words within the line.
#[derive(Debug, Serialize)]
pub struct OcrLine<'a> {
    #[serde(skip_serializing)]
    lib: &'a Library,
    #[serde(skip_serializing)]
    line_handle: i64,
    pub text: String,
    pub bounding_box: BoundingBox,
    pub words: Option<Vec<OcrWord>>,
}

impl<'a> OcrLine<'a> {
    pub(crate) fn new(
        lib: &'a Library,
        line_handle: i64,
        word_level_detail: bool,
    ) -> Result<Self, OneOcrError> {
        load_symbol!(lib, get_ocr_line_content, GetOcrLineContent);
        load_symbol!(lib, get_ocr_line_bounding_box, GetOcrLineBoundingBox);

        let mut line_content: i64 = 0;
        check_ocr_call!(
            unsafe { get_ocr_line_content(line_handle, &mut line_content) },
            "Failed to get line content"
        );
        let line_content_cstr = unsafe { CStr::from_ptr(line_content as *const c_char) };
        let line_content_str = line_content_cstr.to_string_lossy().to_string();

        let mut bounding_box_ptr: *const RawBBox = std::ptr::null();
        check_ocr_call!(
            unsafe { get_ocr_line_bounding_box(line_handle, &mut bounding_box_ptr) },
            "Failed to get line bounding box"
        );

        if bounding_box_ptr.is_null() {
            return Err(OneOcrError::OcrApiError {
                result: -1,
                message: "GetOcrLineBoundingBox returned a null pointer.".to_string(),
            });
        }

        let raw_bbox = unsafe { std::ptr::read(bounding_box_ptr) };
        let bounding_box = BoundingBox::new(raw_bbox);

        if !word_level_detail {
            return Ok(Self {
                lib,
                line_handle,
                text: line_content_str,
                bounding_box,
                words: None,
            });
        }

        load_symbol!(lib, get_ocr_line_word_count, GetOcrLineWordCount);
        load_symbol!(lib, get_ocr_word, GetOcrWord);

        let mut word_count: i64 = 0;
        check_ocr_call!(
            unsafe { get_ocr_line_word_count(line_handle, &mut word_count) },
            "Failed to get word count"
        );
        let mut words = Vec::with_capacity(word_count as usize);
        for i in 0..word_count {
            let mut word: i64 = 0;
            check_ocr_call!(
                unsafe { get_ocr_word(line_handle, i, &mut word) },
                "Failed to get word"
            );

            let ocr_word = OcrWord::new(lib, word)?; // Corrected: OcrWord::new instead of Word::new

            words.push(ocr_word);
        }

        Ok(Self {
            lib,
            line_handle,
            text: line_content_str,
            bounding_box,
            words: Some(words),
        })
    }

    /// Get the line style and confidence score.
    ///  - Returns a tuple containing:
    ///    - A boolean indicating if the line is handwritten (true) or printed (false).
    ///    - A confidence score (0.0-1.0) indicating the certainty of the classification.
    ///      - 0.0: Handwritten
    ///      - 1.0: Printed
    ///  - Returns an error if the OCR API call fails.
    pub fn get_line_style(&self) -> Result<(bool, f32), OneOcrError> {
        load_symbol!(self.lib, get_ocr_line_style_fn, GetOcrLineStyle);

        // style: 0 = Handwritten, 1 = Printed
        let mut style: i32 = 0;
        // handwritten_confidence: 0.0 = Handwritten, 1.0 = Printed
        let mut handwritten_confidence: f32 = 0.0;

        check_ocr_call!(
            unsafe {
                get_ocr_line_style_fn(self.line_handle, &mut style, &mut handwritten_confidence)
            },
            "Failed to get OCR line style"
        );

        Ok((style == 0, handwritten_confidence))
    }
}
