use crate::errors::OneOcrError;
use crate::ocr_result::OcrResult;
use crate::{ONE_OCR_MODEL_FILE_NAME, ONE_OCR_MODEL_KEY};
use image::DynamicImage;
use libloading::Library;
use std::ffi::{CString, c_char};
use std::path::Path;

// FFI types
use crate::ffi::{
    CreateOcrInitOptions, CreateOcrPipeline, CreateOcrProcessOptions,
    OcrInitOptionsSetUseModelDelayLoad, OcrProcessOptionsGetMaxRecognitionLineCount,
    OcrProcessOptionsGetResizeResolution, OcrProcessOptionsSetMaxRecognitionLineCount,
    OcrProcessOptionsSetResizeResolution, RawImage, ReleaseOcrInitOptions, ReleaseOcrPipeline,
    ReleaseOcrProcessOptions, RunOcrPipeline,
};
// Macros
use crate::{check_ocr_call, load_symbol, release_ocr_resource};

/// The `OcrEngine` struct represents the OneOcr processing engine.
#[derive(Debug)]
pub struct OcrEngine {
    lib: Library,
    init_options: i64,
    pipeline: i64,
    process_options: i64,
}

impl OcrEngine {
    /// Creates a new instance of the OCR engine.
    /// This function loads the necessary library and initializes the OCR pipeline.
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

        // The FFI function OcrInitOptionsSetUseModelDelayLoad expects a c_char (i8).
        // In C, char can be signed or unsigned by default depending on the compiler/platform.
        // Rust's c_char is i8. Assuming 0 is a valid value for false.
        check_ocr_call!(
            unsafe { ocr_init_options_set_use_model_delay_load(init_options, 0 as c_char) },
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

    /// Retrieves the maximum number of lines that can be recognized.
    /// Default is 100.
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

    /// Sets the maximum number of lines that can be recognized.
    /// Default is 100, range is 0-1000.
    pub fn set_max_recognition_line_count(&self, count: i64) -> Result<(), OneOcrError> {
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

    /// Retrieves the maximum internal resize resolution.
    ///
    /// The `resize resolution` defines the maximum dimensions to which an image will be automatically scaled internally before OCR processing.
    /// It’s a performance and accuracy trade-off rather than a restriction on the original image’s resolution.
    ///
    /// Default is 1152*768.
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

    /// Sets the maximum internal resize resolution.
    ///
    /// The `resize resolution` defines the maximum dimensions to which an image will be automatically scaled internally before OCR processing.
    /// It’s a performance and accuracy trade-off rather than a restriction on the original image’s resolution.
    ///
    /// The maximum resolution is 1152*768.
    pub fn set_resize_resolution(&self, width: i64, height: i64) -> Result<(), OneOcrError> {
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

        load_symbol!(self.lib, run_ocr_pipeline, RunOcrPipeline);

        let mut ocr_result_handle: i64 = 0;
        check_ocr_call!(
            unsafe {
                run_ocr_pipeline(
                    self.pipeline,
                    &image,
                    self.process_options,
                    &mut ocr_result_handle,
                )
            },
            "Failed to run OCR pipeline"
        );

        OcrResult::new(&self.lib, ocr_result_handle, word_level_detail)
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
        release_ocr_resource!(self.lib, ReleaseOcrPipeline, self.pipeline);
        release_ocr_resource!(self.lib, ReleaseOcrInitOptions, self.init_options);
        release_ocr_resource!(self.lib, ReleaseOcrProcessOptions, self.process_options);
    }
}
