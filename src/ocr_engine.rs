use crate::errors::OneOcrError;
use crate::ffi::{
    CreateOcrInitOptions, CreateOcrPipeline, CreateOcrProcessOptions,
    OcrInitOptionsSetUseModelDelayLoad, OcrProcessOptionsGetMaxRecognitionLineCount,
    OcrProcessOptionsGetResizeResolution, OcrProcessOptionsSetMaxRecognitionLineCount,
    OcrProcessOptionsSetResizeResolution, RawImage, ReleaseOcrInitOptions, ReleaseOcrPipeline,
    ReleaseOcrProcessOptions, RunOcrPipeline,
};
use crate::ocr_result::OcrResult;
use crate::{ONE_OCR_MODEL_FILE_NAME, ONE_OCR_MODEL_KEY};
use image::DynamicImage;
use std::ffi::{CString, c_void};
use std::path::Path;
use std::ptr;

// Macros
use crate::check_ocr_call;

/// The `OcrEngine` struct represents the OneOcr processing engine.
#[derive(Debug)]
pub struct OcrEngine {
    init_options: *mut c_void,
    pipeline: *mut c_void,
    process_options: *mut c_void,
}

impl OcrEngine {
    /// Creates a new instance of the OCR engine.
    /// This function loads the necessary library and initializes the OCR pipeline.
    pub fn new() -> Result<Self, OneOcrError> {
        let mut init_options: *mut c_void = ptr::null_mut();
        check_ocr_call!(
            unsafe { CreateOcrInitOptions(&mut init_options) },
            "Failed to create init options"
        );

        // The FFI function OcrInitOptionsSetUseModelDelayLoad expects a c_char (i8).
        // In C, char can be signed or unsigned by default depending on the compiler/platform.
        // Rust's c_char is i8. Assuming 0 is a valid value for false.
        check_ocr_call!(
            unsafe { OcrInitOptionsSetUseModelDelayLoad(init_options) },
            "Failed to set model delay load"
        );

        let model_path = Self::get_model_path()?;
        let model_path_cstr = CString::new(model_path).map_err(|e| {
            OneOcrError::ModelFileLoadError(format!(
                "Failed to convert model path to CString: {}",
                e
            ))
        })?;

        let key_cstr = CString::new(ONE_OCR_MODEL_KEY).map_err(|e| {
            OneOcrError::InvalidModelKey(format!("Failed to convert model key to CString: {}", e))
        })?;

        let mut pipeline: *mut c_void = ptr::null_mut();
        check_ocr_call!(
            unsafe {
                CreateOcrPipeline(
                    model_path_cstr.as_ptr(),
                    key_cstr.as_ptr(),
                    init_options,
                    &mut pipeline,
                )
            },
            "Failed to create OCR pipeline"
        );

        let mut process_options: *mut c_void = ptr::null_mut();
        check_ocr_call!(
            unsafe { CreateOcrProcessOptions(&mut process_options) },
            "Failed to create OCR process options"
        );

        Ok(Self {
            init_options,
            pipeline,
            process_options,
        })
    }

    /// Retrieves the maximum number of lines that can be recognized.
    /// Default is 100.
    pub fn get_max_recognition_line_count(&self) -> Result<i32, OneOcrError> {
        let mut count: i32 = 0;
        check_ocr_call!(
            unsafe {
                OcrProcessOptionsGetMaxRecognitionLineCount(self.process_options, &mut count)
            },
            "Failed to get max recognition line count"
        );
        Ok(count)
    }

    /// Sets the maximum number of lines that can be recognized.
    /// Default is 100, range is 0-1000.
    pub fn set_max_recognition_line_count(&self, count: i32) -> Result<(), OneOcrError> {
        check_ocr_call!(
            unsafe { OcrProcessOptionsSetMaxRecognitionLineCount(self.process_options, count) },
            "Failed to set max recognition line count"
        );
        Ok(())
    }

    /// Retrieves the maximum internal resize resolution.
    ///
    /// The `resize resolution` defines the maximum dimensions to which an image will be automatically scaled internally before OCR processing.
    /// It’s a performance and accuracy trade-off rather than a restriction on the original image’s resolution.
    ///
    /// Default is 1152*768.
    pub fn get_resize_resolution(&self) -> Result<(i64, i64), OneOcrError> {
        let mut width: i64 = 0;
        let mut height: i64 = 0;
        check_ocr_call!(
            unsafe {
                OcrProcessOptionsGetResizeResolution(self.process_options, &mut width, &mut height)
            },
            "Failed to get resize resolution"
        );
        Ok((width, height))
    }

    /// Sets the maximum internal resize resolution.
    ///
    /// The `resize resolution` defines the maximum dimensions to which an image will be automatically scaled internally before OCR processing.
    /// It’s a performance and accuracy trade-off rather than a restriction on the original image’s resolution.
    ///
    /// The maximum resolution is 1152*768.
    pub fn set_resize_resolution(&self, width: i32, height: i32) -> Result<(), OneOcrError> {
        check_ocr_call!(
            unsafe { OcrProcessOptionsSetResizeResolution(self.process_options, width, height) },
            "Failed to set resize resolution"
        );
        Ok(())
    }

    /// Run the OCR pipeline on the given image path.
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
        let image = RawImage {
            t: 3, // Assuming 3 means RGBA or a type the C API expects
            col: cols,
            row: rows,
            _unk: 0,
            step,
            data_ptr,
        };

        let mut ocr_result: *mut c_void = ptr::null_mut();
        check_ocr_call!(
            unsafe { RunOcrPipeline(self.pipeline, &image, self.process_options, &mut ocr_result) },
            "Failed to run OCR pipeline"
        );

        OcrResult::new(ocr_result, word_level_detail)
    }

    /// Retrieves the path to the model file.
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
        let model_path_string = model_path_buf.to_string_lossy().to_string();

        Ok(model_path_string)
    }
}

impl Drop for OcrEngine {
    fn drop(&mut self) {
        unsafe {
            ReleaseOcrPipeline(self.pipeline);
            ReleaseOcrInitOptions(self.init_options);
            ReleaseOcrProcessOptions(self.process_options);
        };
    }
}
