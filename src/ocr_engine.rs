use crate::errors::OneOcrError;
use crate::ffi::{
    CreateOcrInitOptions, CreateOcrPipeline, CreateOcrProcessOptions,
    OcrInitOptionsSetUseModelDelayLoad, OcrProcessOptionsGetMaxRecognitionLineCount,
    OcrProcessOptionsGetResizeResolution, OcrProcessOptionsSetMaxRecognitionLineCount,
    OcrProcessOptionsSetResizeResolution, RawImage, ReleaseOcrInitOptions, ReleaseOcrPipeline,
    ReleaseOcrProcessOptions, RunOcrPipeline,
};
use crate::ocr_result::OcrResult;
use crate::{ImageInput, ONE_OCR_MODEL_FILE_NAME, ONE_OCR_MODEL_KEY, OcrOptions};
use image::{DynamicImage, ImageBuffer, Rgba};
use std::ffi::{CString, c_void};
use std::ptr;

// Macros
use crate::check_ocr_call;

/// The `OcrEngine` struct represents the OneOcr processing engine.
#[derive(Debug)]
pub struct OcrEngine {
    init_options: *mut c_void,
    pipeline: *mut c_void,
    process_options: *mut c_void,
    ocr_options: OcrOptions,
}

impl OcrEngine {
    /// Creates a new instance of the OCR engine with specified options.
    /// This function loads the necessary library and initializes the OCR pipeline with the provided options.
    pub fn new_with_options(ocr_options: OcrOptions) -> Result<Self, OneOcrError> {
        let mut init_options: *mut c_void = ptr::null_mut();
        check_ocr_call!(
            unsafe { CreateOcrInitOptions(&mut init_options) },
            "Failed to create init options"
        );

        // Disable model delay load
        check_ocr_call!(
            unsafe { OcrInitOptionsSetUseModelDelayLoad(init_options, 0) },
            "Failed to set model delay load"
        );

        let model_path = Self::get_model_path()?;
        let model_path_cstr = CString::new(model_path).map_err(|e| {
            OneOcrError::ModelFileLoadError(format!("Failed to convert model path to CString: {e}"))
        })?;

        let key_cstr = CString::new(ONE_OCR_MODEL_KEY).map_err(|e| {
            OneOcrError::InvalidModelKey(format!("Failed to convert model key to CString: {e}"))
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

        check_ocr_call!(
            unsafe {
                OcrProcessOptionsSetMaxRecognitionLineCount(
                    process_options,
                    ocr_options.max_recognition_line_count,
                )
            },
            "Failed to set max recognition line count"
        );

        check_ocr_call!(
            unsafe {
                OcrProcessOptionsSetResizeResolution(
                    process_options,
                    ocr_options.resize_resolution.width,
                    ocr_options.resize_resolution.height,
                )
            },
            "Failed to set resize resolution"
        );

        Ok(Self {
            init_options,
            pipeline,
            process_options,
            ocr_options,
        })
    }

    /// Creates a new instance of the OCR engine with default options.
    /// This function loads the necessary library and initializes the OCR pipeline.
    pub fn new() -> Result<Self, OneOcrError> {
        Self::new_with_options(OcrOptions::default())
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

    /// Run OCR processing on an image.
    ///
    /// This method accepts various input types through the `ImageInput` enum
    /// and allows configuration through `OcrOptions`.
    ///
    /// # Arguments
    ///
    /// * `input` - The image input source (file path, image buffer, or dynamic image)
    ///
    /// # Returns
    ///
    /// Returns an `OcrResult` containing the recognized text and associated metadata,
    /// or an error if the OCR processing fails.
    ///
    /// # Examples
    ///
    /// ```no_run
    /// use oneocr_rs::{OcrEngine, OcrOptions, ImageInput};
    /// use std::path::Path;
    /// let engine = OcrEngine::new().unwrap();
    ///
    /// // Process from file path
    /// let result = engine.run(Path::new("image.jpg").into()).unwrap();
    /// ```
    ///
    /// ```ignore
    /// // Process from in-memory image buffer
    /// let img_buffer: ImageBuffer<Rgba<u8>, Vec<u8>> = capture_screenshot(); // Your screenshot function
    /// let result = engine.run(img_buffer.into()).unwrap();
    /// ```
    pub fn run(&self, input: ImageInput) -> Result<OcrResult, OneOcrError> {
        let img_rgba = self.load_image(input)?;
        self.run_ocr_on_rgba_image(&img_rgba, self.ocr_options.include_word_level_details)
    }

    /// Loads an image from various input sources and converts it to RGBA format.
    fn load_image(&self, input: ImageInput) -> Result<ImageBuffer<Rgba<u8>, Vec<u8>>, OneOcrError> {
        match input {
            ImageInput::FilePath(path) => {
                let img = image::open(path)?;
                Ok(self.convert_to_rgba(img))
            }
            ImageInput::Buffer(buffer) => Ok(buffer),
            ImageInput::Dynamic(img) => Ok(self.convert_to_rgba(img)),
        }
    }

    /// Converts a DynamicImage to RGBA format.
    fn convert_to_rgba(&self, img: DynamicImage) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        match img {
            DynamicImage::ImageRgba8(i) => i,
            _ => img.to_rgba8(),
        }
    }

    /// Performs OCR on an RGBA image buffer.
    fn run_ocr_on_rgba_image(
        &self,
        img_rgba: &ImageBuffer<Rgba<u8>, Vec<u8>>,
        word_level_detail: bool,
    ) -> Result<OcrResult, OneOcrError> {
        let (rows, cols) = (img_rgba.height() as i32, img_rgba.width() as i32);
        let step = (img_rgba.sample_layout().height_stride) as i64;
        let data_ptr = img_rgba.as_ptr() as i64;
        let image = RawImage {
            t: 3, // RGBA format identifier expected by the C API
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
            OneOcrError::ModelFileLoadError(format!("Failed to get current executable path: {e}"))
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
