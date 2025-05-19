use crate::bounding_box::BoundingBox;
use crate::image::Image;
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
pub type RunOcrPipeline = unsafe extern "C" fn(i64, *const Image, i64, *mut i64) -> i64;

pub type GetImageAngle = unsafe extern "C" fn(i64, *mut f32) -> i64;

pub type GetOcrLineCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub type GetOcrLine = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;
pub type GetOcrLineContent = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub type GetOcrLineBoundingBox = unsafe extern "C" fn(i64, *mut *const BoundingBox) -> i64;
pub type GetOcrLineStyle = unsafe extern "C" fn(i64, *mut i32, *mut f32) -> i64;

pub type GetOcrLineWordCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub type GetOcrWord = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;
pub type GetOcrWordContent = unsafe extern "C" fn(i64, *mut i64) -> i64;
pub type GetOcrWordBoundingBox = unsafe extern "C" fn(i64, *mut *const BoundingBox) -> i64;
pub type GetOcrWordConfidence = unsafe extern "C" fn(i64, *mut f32) -> i64;

pub type ReleaseOcrResult = unsafe extern "C" fn(i64);
pub type ReleaseOcrInitOptions = unsafe extern "C" fn(i64);
pub type ReleaseOcrPipeline = unsafe extern "C" fn(i64);
pub type ReleaseOcrProcessOptions = unsafe extern "C" fn(i64);
