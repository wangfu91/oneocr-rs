pub mod errors;

use errors::OneOcrError;
use image::DynamicImage;
use libloading::Library;
use std::{
    ffi::{CStr, c_char},
    path::Path,
};

const ONE_OCR_MODEL_FILE_NAME: &str = "oneocr.onemodel";
const ONE_OCR_MODEL_KEY: &str = r#"kj)TGtrK>f]b[Piow.gU+nC@s""""""4"#;

type CreateOcrInitOptions = unsafe extern "C" fn(*mut i64) -> i64;
type OcrInitOptionsSetUseModelDelayLoad = unsafe extern "C" fn(i64, c_char) -> i64;
type CreateOcrPipeline = unsafe extern "C" fn(
    model_path: *const c_char,
    key: *const c_char,
    ctx: i64,
    pipeline: *mut i64,
) -> i64;

type CreateOcrProcessOptions = unsafe extern "C" fn(*mut i64) -> i64;
type OcrProcessOptionsGetMaxRecognitionLineCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
type OcrProcessOptionsSetMaxRecognitionLineCount = unsafe extern "C" fn(i64, i64) -> i64;
type OcrProcessOptionsGetResizeResolution = unsafe extern "C" fn(i64, *mut i64, *mut i64) -> i64;
type OcrProcessOptionsSetResizeResolution = unsafe extern "C" fn(i64, i64, i64) -> i64;

/// Image resolution must be great than 50*50, otherwise it will return error code 3.
type RunOcrPipeline = unsafe extern "C" fn(i64, *const Image, i64, *mut i64) -> i64;

type GetImageAngle = unsafe extern "C" fn(i64, *mut f32) -> i64;

type GetOcrLineCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
type GetOcrLine = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;
type GetOcrLineContent = unsafe extern "C" fn(i64, *mut i64) -> i64;
type GetOcrLineBoundingBox = unsafe extern "C" fn(i64, *mut *const BoundingBox) -> i64;

type GetOcrLineWordCount = unsafe extern "C" fn(i64, *mut i64) -> i64;
type GetOcrWord = unsafe extern "C" fn(i64, i64, *mut i64) -> i64;
type GetOcrWordContent = unsafe extern "C" fn(i64, *mut i64) -> i64;
type GetOcrWordBoundingBox = unsafe extern "C" fn(i64, *mut *const BoundingBox) -> i64;
type GetOcrWordConfidence = unsafe extern "C" fn(i64, *mut f32) -> i64;

type ReleaseOcrResult = unsafe extern "C" fn(i64);
type ReleaseOcrInitOptions = unsafe extern "C" fn(i64);
type ReleaseOcrPipeline = unsafe extern "C" fn(i64);
type ReleaseOcrProcessOptions = unsafe extern "C" fn(i64);

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

/// A macro to attempt to load a symbol and call it, for use in contexts like `drop`.
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

/// A macro to check the result of an OCR call and return an error if it fails.
/// This macro takes an expression `$call` and an error message `$err_msg`.
/// If the result of `$call` is not 0, it returns an `OneOcrError::OcrApiError` error with the provided message.
/// This macro is used to simplify error handling in the OCR engine methods.
/// It helps to avoid repetitive error checking code and makes the code cleaner and more readable.
macro_rules! check_ocr_call {
    ($call:expr, $err_msg:literal) => {
        let res = $call;
        if res != 0 {
            return Err(OneOcrError::OcrApiError {
                result: res,
                message: $err_msg.to_string(),
            });
        }
    };
}

/// This struct represents an image in a format suitable for OCR processing.
///  - t: Type of the image (e.g., RGB, RGBA).
///  - col: Number of columns (width) in the image.
///  - row: Number of rows (height) in the image.
///  - _unk: Unknown field, possibly reserved for future use.
///  - step: Step size in bytes for each row of the image data.
///  - data_ptr: Pointer to the image data in memory.
///
/// The `#[repr(C, packed)]` attribute ensures that the struct has a C-compatible layout and is packed tightly in memory.
/// This is important for interoperability with C libraries and for ensuring that the data layout matches the expected format.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy)]
struct Image {
    t: i32,
    col: i32,
    row: i32,
    _unk: i32,
    step: i64,
    data_ptr: i64,
}

/// This struct represents a quadrilateral (four-sided polygon) in 2D space, typically used for OCR (Optical Character Recognition) to tightly enclose detected text. Each pair of fields represents the X and Y coordinates of a corner of the bounding box.
///  - x1, y1: Coordinates of the first corner (often the top-left).
///  - x2, y2: Coordinates of the second corner (often the top-right).
///  - x3, y3: Coordinates of the third corner (often the bottom-right).
///  - x4, y4: Coordinates of the fourth corner (often the bottom-left).
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default)]
pub struct BoundingBox {
    x1: f32,
    y1: f32,
    x2: f32,
    y2: f32,
    x3: f32,
    y3: f32,
    x4: f32,
    y4: f32,
}

#[derive(Debug)]
pub struct OcrEngine {
    lib: Library,
    init_options: i64,
    pipeline: i64,
    process_options: i64,
}

impl OcrEngine {
    pub fn new() -> Result<Self, OneOcrError> {
        let lib = unsafe { Library::new("oneocr.dll")? };

        load_symbol!(lib, create_ocr_init_options, CreateOcrInitOptions);
        load_symbol!(
            lib,
            ocr_init_options_set_use_model_delay_load,
            OcrInitOptionsSetUseModelDelayLoad
        );
        load_symbol!(lib, create_ocr_pipeline, CreateOcrPipeline);
        load_symbol!(lib, create_ocr_process_options, CreateOcrProcessOptions);

        let mut init_options: i64 = 0;
        check_ocr_call!(
            unsafe { create_ocr_init_options(&mut init_options) },
            "Failed to create init options"
        );

        check_ocr_call!(
            unsafe { ocr_init_options_set_use_model_delay_load(init_options, 0) },
            "Failed to set model delay load"
        );

        let model_path = Self::get_model_path()?;
        let model_path_cstr = std::ffi::CString::new(model_path).map_err(|e| {
            OneOcrError::ModelFileLoadError(format!(
                "Failed to convert model path to CString: {}",
                e
            ))
        })?;

        let key_cstr = std::ffi::CString::new(ONE_OCR_MODEL_KEY).map_err(|e| {
            OneOcrError::InvalidModelKey(format!("Failed to convert model key to CString: {}", e))
        })?;

        let mut pipeline: i64 = 0;
        check_ocr_call!(
            unsafe {
                create_ocr_pipeline(
                    model_path_cstr.as_ptr(),
                    key_cstr.as_ptr(),
                    init_options,
                    &mut pipeline,
                )
            },
            "Failed to create OCR pipeline"
        );

        let mut process_options: i64 = 0;
        check_ocr_call!(
            unsafe { create_ocr_process_options(&mut process_options) },
            "Failed to create OCR process options"
        );

        Ok(Self {
            lib,
            init_options,
            pipeline,
            process_options,
        })
    }

    pub fn get_max_recognition_line_count(&self) -> Result<i64, OneOcrError> {
        load_symbol!(
            self.lib,
            ocr_process_options_get_max_recognition_line_count,
            OcrProcessOptionsGetMaxRecognitionLineCount
        );
        let mut count: i64 = 0;
        check_ocr_call!(
            unsafe {
                ocr_process_options_get_max_recognition_line_count(self.process_options, &mut count)
            },
            "Failed to get max recognition line count"
        );
        Ok(count)
    }

    pub fn set_max_recognition_line_count(&mut self, count: i64) -> Result<(), OneOcrError> {
        load_symbol!(
            self.lib,
            ocr_process_options_set_max_recognition_line_count,
            OcrProcessOptionsSetMaxRecognitionLineCount
        );
        check_ocr_call!(
            unsafe {
                ocr_process_options_set_max_recognition_line_count(self.process_options, count)
            },
            "Failed to set max recognition line count"
        );
        Ok(())
    }

    pub fn get_resize_resolution(&self) -> Result<(i64, i64), OneOcrError> {
        load_symbol!(
            self.lib,
            ocr_process_options_get_resize_resolution,
            OcrProcessOptionsGetResizeResolution
        );
        let mut width: i64 = 0;
        let mut height: i64 = 0;
        check_ocr_call!(
            unsafe {
                ocr_process_options_get_resize_resolution(
                    self.process_options,
                    &mut width,
                    &mut height,
                )
            },
            "Failed to get resize resolution"
        );
        Ok((width, height))
    }

    pub fn set_resize_resolution(&mut self, width: i64, height: i64) -> Result<(), OneOcrError> {
        load_symbol!(
            self.lib,
            ocr_process_options_set_resize_resolution,
            OcrProcessOptionsSetResizeResolution
        );
        check_ocr_call!(
            unsafe {
                ocr_process_options_set_resize_resolution(self.process_options, width, height)
            },
            "Failed to set resize resolution"
        );
        Ok(())
    }

    /// Run the OCR pipeline on the given image path.
    ///  - `image_path`: The path to the image file.
    ///  - `word_level_detail`: If true, returns word-level recognition details.
    ///  - Returns an `OcrResult` containing the recognized text and bounding boxes.
    pub fn run(
        &self,
        image_path: &Path,
        word_level_detail: bool,
    ) -> Result<OcrResult, OneOcrError> {
        let img = image::open(Path::new(image_path))?;
        let img_rgba = match img {
            DynamicImage::ImageRgba8(i) => i,
            DynamicImage::ImageRgb8(i) => DynamicImage::ImageRgb8(i).to_rgba8(),
            _ => {
                return Err(OneOcrError::ImageFormatError(format!(
                    "Unsupported image format: {:?}",
                    img
                )));
            }
        };
        let (rows, cols) = (img_rgba.height() as i32, img_rgba.width() as i32);
        let step = (img_rgba.sample_layout().height_stride) as i64;
        let data_ptr = img_rgba.as_ptr() as i64;
        let image = Image {
            t: 3,
            col: cols,
            row: rows,
            _unk: 0,
            step,
            data_ptr,
        };

        load_symbol!(self.lib, run_ocr_pipeline, RunOcrPipeline);

        let mut ocr_result: i64 = 0;
        check_ocr_call!(
            unsafe {
                run_ocr_pipeline(self.pipeline, &image, self.process_options, &mut ocr_result)
            },
            "Failed to run OCR pipeline"
        );

        OcrResult::new(&self.lib, ocr_result, word_level_detail)
    }

    fn get_model_path() -> Result<String, OneOcrError> {
        let exe_path = std::env::current_exe().map_err(|e| {
            OneOcrError::ModelFileLoadError(format!("Failed to get current executable path: {}", e))
        })?;
        let model_path_buf = exe_path
            .parent()
            .ok_or_else(|| {
                OneOcrError::ModelFileLoadError(
                    "Failed to get parent directory of current executable".to_string(),
                )
            })?
            .join(ONE_OCR_MODEL_FILE_NAME);
        let model_path_sting = model_path_buf.to_string_lossy().to_string();

        Ok(model_path_sting)
    }
}

impl Drop for OcrEngine {
    fn drop(&mut self) {
        release_ocr_resource!(self.lib, ReleaseOcrPipeline, self.pipeline);
        release_ocr_resource!(self.lib, ReleaseOcrInitOptions, self.init_options);
        release_ocr_resource!(self.lib, ReleaseOcrProcessOptions, self.process_options);
    }
}

#[derive(Debug)]
pub struct OcrResult<'a> {
    lib: &'a Library,
    result_handle: i64,
    pub lines: Vec<OcrLine>,
    pub image_angle: f32,
}

impl<'a> OcrResult<'a> {
    fn new(
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

#[derive(Debug)]
pub struct OcrLine {
    pub content: String,
    pub bounding_box: BoundingBox,
    pub words: Option<Vec<OcrWord>>,
}

impl OcrLine {
    pub fn new(
        lib: &Library,
        line_handle: i64,
        word_level_detail: bool,
    ) -> Result<Self, OneOcrError> {
        load_symbol!(lib, get_ocr_line_content, GetOcrLineContent);
        load_symbol!(lib, get_ocr_line_bounding_box, GetOcrLineBoundingBox);
        load_symbol!(lib, get_ocr_line_word_count, GetOcrLineWordCount);
        load_symbol!(lib, get_ocr_word, GetOcrWord);

        let mut line_content: i64 = 0;
        check_ocr_call!(
            unsafe { get_ocr_line_content(line_handle, &mut line_content) },
            "Failed to get line content"
        );
        let line_content_cstr = unsafe { CStr::from_ptr(line_content as *const c_char) };
        let line_content_str = line_content_cstr.to_string_lossy().to_string();

        let mut bounding_box_ptr: *const BoundingBox = std::ptr::null();
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
        let bounding_box = unsafe { std::ptr::read(bounding_box_ptr) };

        if !word_level_detail {
            return Ok(Self {
                content: line_content_str,
                bounding_box,
                words: None,
            });
        }

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

            let word = OcrWord::new(lib, word)?;

            words.push(word);
        }

        Ok(Self {
            content: line_content_str,
            bounding_box,
            words: Some(words),
        })
    }
}

#[derive(Debug)]
pub struct OcrWord {
    pub content: String,
    pub confidence: f32,
    pub bounding_box: BoundingBox,
}

impl OcrWord {
    pub fn new(lib: &Library, word_handle: i64) -> Result<Self, OneOcrError> {
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
            content: word_content_str,
            confidence,
            bounding_box,
        })
    }
}
