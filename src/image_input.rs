use std::path::Path;

use image::{DynamicImage, ImageBuffer, Rgba};

/// Input source for OCR processing.
#[derive(Debug)]
pub enum ImageInput<'a> {
    /// Process an image from a file path.
    FilePath(&'a Path),
    /// Process an image from an in-memory buffer.
    /// The buffer should contain RGBA pixel data.
    Buffer(&'a ImageBuffer<Rgba<u8>, Vec<u8>>),
    /// Process a dynamic image.
    Dynamic(&'a DynamicImage),
}

impl<'a> From<&'a Path> for ImageInput<'a> {
    fn from(path: &'a Path) -> Self {
        ImageInput::FilePath(path)
    }
}

impl<'a> From<&'a ImageBuffer<Rgba<u8>, Vec<u8>>> for ImageInput<'a> {
    fn from(buffer: &'a ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self {
        ImageInput::Buffer(buffer)
    }
}

impl<'a> From<&'a DynamicImage> for ImageInput<'a> {
    fn from(image: &'a DynamicImage) -> Self {
        ImageInput::Dynamic(image)
    }
}
