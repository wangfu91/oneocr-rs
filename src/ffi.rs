use std::ffi::c_char;

pub type CreateOcrInitOptions = unsafe extern "C" fn(*mut i64) -> i64;
pub type OcrInitOptionsSetUseModelDelayLoad = unsafe extern "C" fn(i64, c_char) -> i64;
pub type CreateOcrPipeline = unsafe extern "C" fn(
    model_path: *const c_char,
    key: *const c_char,
    ctx: i64,
    pipeline: *mut i64,
) -> i64;

pub type CreateOcrProcessOptions = unsafe extern "C" fn(*mut i64) -> i64;
pub type OcrProcessOptionsGetMaxRecognitionLineCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub type OcrProcessOptionsSetMaxRecognitionLineCount = unsafe extern "C" fn(i64, i64) -> i64;
pub type OcrProcessOptionsGetResizeResolution =
    unsafe extern "C" fn(i64, *mut i64, *mut i64) -> i64;
pub type OcrProcessOptionsSetResizeResolution = unsafe extern "C" fn(i64, i64, i64) -> i64;

/// Image resolution must be great than 50*50, otherwise it will return error code 3.
/// For images with a resolution less than 50*50, you should manually scale up the image first.
pub type RunOcrPipeline = unsafe extern "C" fn(i64, *const RawImage, i64, *mut i64) -> i64;

pub type GetImageAngle = unsafe extern "C" fn(i64, *mut f32) -> i64;

pub type GetOcrLineCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub type GetOcrLine = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;
pub type GetOcrLineContent = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub type GetOcrLineBoundingBox = unsafe extern "C" fn(i64, *mut *const RawBBox) -> i64;
pub type GetOcrLineStyle = unsafe extern "C" fn(i64, *mut i32, *mut f32) -> i64;

pub type GetOcrLineWordCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub type GetOcrWord = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;
pub type GetOcrWordContent = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub type GetOcrWordBoundingBox = unsafe extern "C" fn(i64, *mut *const RawBBox) -> i64;
pub type GetOcrWordConfidence = unsafe extern "C" fn(i64, *mut f32) -> i64;

pub type ReleaseOcrResult = unsafe extern "C" fn(i64);
pub type ReleaseOcrInitOptions = unsafe extern "C" fn(i64);
pub type ReleaseOcrPipeline = unsafe extern "C" fn(i64);
pub type ReleaseOcrProcessOptions = unsafe extern "C" fn(i64);

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
