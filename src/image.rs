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
pub struct Image {
    pub t: i32,
    pub col: i32,
    pub row: i32,
    pub _unk: i32,
    pub step: i64,
    pub data_ptr: i64,
}
