use std::path::{Path, PathBuf};

use image::{DynamicImage, ImageBuffer, Rgba};

/// Input source for OCR processing.
#[derive(Debug)]
pub enum ImageInput {
    /// Process an image from a file path.
    FilePath(PathBuf),
    /// Process an image from an in-memory buffer.
    /// The buffer should contain RGBA pixel data.
    Buffer(ImageBuffer<Rgba<u8>, Vec<u8>>),
    /// Process a dynamic image.
    Dynamic(DynamicImage),
}

impl From<&Path> for ImageInput {
    fn from(path: &Path) -> Self {
        ImageInput::FilePath(path.to_path_buf())
    }
}

impl From<PathBuf> for ImageInput {
    fn from(path: PathBuf) -> Self {
        ImageInput::FilePath(path)
    }
}

impl From<ImageBuffer<Rgba<u8>, Vec<u8>>> for ImageInput {
    fn from(buffer: ImageBuffer<Rgba<u8>, Vec<u8>>) -> Self {
        ImageInput::Buffer(buffer)
    }
}

impl From<DynamicImage> for ImageInput {
    fn from(image: DynamicImage) -> Self {
        ImageInput::Dynamic(image)
    }
}
