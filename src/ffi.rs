use std::ffi::{c_char, c_void};
use windows_link::link;

link!("oneocr.dll"  "system"  fn CreateOcrInitOptions(init_option: *mut *mut c_void) -> i32);
link!("oneocr.dll"  "system"  fn OcrInitOptionsSetUseModelDelayLoad(init_option: *mut c_void) -> i32);
link!("oneocr.dll"  "system"  fn CreateOcrPipeline(
    model_path: *const c_char,
    key: *const c_char,
    ctx: *mut c_void,
    pipeline: *mut *mut c_void
) -> i32);
link!("oneocr.dll"  "system"  fn CreateOcrProcessOptions(option: *mut *mut c_void) -> i32);
link!("oneocr.dll"  "system"  fn OcrProcessOptionsGetMaxRecognitionLineCount(option: *mut c_void, count: *mut i32) -> i32);
link!("oneocr.dll"  "system"  fn OcrProcessOptionsSetMaxRecognitionLineCount(option: *mut c_void, count: i32) -> i32);
link!("oneocr.dll"  "system"  fn OcrProcessOptionsGetResizeResolution(option: *mut c_void, width: *mut i64, height: *mut i64) -> i32);
link!("oneocr.dll"  "system"  fn OcrProcessOptionsSetResizeResolution   (option: *mut c_void, width: i32, height: i32) -> i32);
link!("oneocr.dll"  "system"  fn RunOcrPipeline(
    pipeline: *mut c_void,
    image: *const RawImage,
    process_options: *mut c_void,
    result: *mut *mut c_void
) -> i32);
link!("oneocr.dll"  "system"  fn GetImageAngle(pipeline: *mut c_void, angle: *mut f32) -> i32);
link!("oneocr.dll"  "system"  fn GetOcrLineCount(result: *mut c_void, count: *mut i64) -> i32);
link!("oneocr.dll"  "system"  fn GetOcrLine(result: *mut c_void, index: i64, line: *mut *mut c_void) -> i32);
link!("oneocr.dll"  "system"  fn GetOcrLineContent(line: *mut c_void, content: *mut *const c_char) -> i32);
link!("oneocr.dll"  "system"  fn GetOcrLineBoundingBox(line: *mut c_void, bbox: *mut *const RawBBox) -> i32);
link!("oneocr.dll"  "system"  fn GetOcrLineStyle(line: *mut c_void, style: *mut i32, confidence: *mut f32) -> i32);
link!("oneocr.dll"  "system"  fn GetOcrLineWordCount(line: *mut c_void, count: *mut i64) -> i32);
link!("oneocr.dll"  "system"  fn GetOcrWord(line: *mut c_void, index: i64, word: *mut *mut c_void) -> i32);
link!("oneocr.dll"  "system"  fn GetOcrWordContent(word: *mut c_void, content: *mut *const c_char) -> i32);
link!("oneocr.dll"  "system"  fn GetOcrWordBoundingBox(word: *mut c_void, bbox: *mut *const RawBBox) -> i32);
link!("oneocr.dll"  "system"  fn GetOcrWordConfidence(word: *mut c_void, confidence: *mut f32) -> i32);
link!("oneocr.dll"  "system"  fn ReleaseOcrResult(result: *mut c_void));
link!("oneocr.dll"  "system"  fn ReleaseOcrInitOptions(init_options: *mut c_void));
link!("oneocr.dll"  "system"  fn ReleaseOcrPipeline(pipeline: *mut c_void));
link!("oneocr.dll"  "system"  fn ReleaseOcrProcessOptions(process_options: *mut c_void));

/// This `RawImage` struct represents an image in a format suitable for OCR processing. Used for FFI.
///  - t: Type of the image (e.g., RGB, RGBA).
///  - col: Number of columns (width) in the image.
///  - row: Number of rows (height) in the image.
///  - _unk: Unknown field, possibly reserved for future use.
///  - step: Step size in bytes for each row of the image data.
///  - data_ptr: Pointer to the image data in memory.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct RawImage {
    pub t: i32,
    pub col: i32,
    pub row: i32,
    pub _unk: i32,
    pub step: i64,
    pub data_ptr: i64,
}

/// This `RawBBox` struct represents a bounding box in 2D space, used for FFI.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
pub(crate) struct RawBBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub x3: f32,
    pub y3: f32,
    pub x4: f32,
    pub y4: f32,
}
