use serde::Serialize;

/// This `BoundingBox` struct represents a quadrilateral (four-sided polygon) in 2D space, typically used for OCR (Optical Character Recognition) to tightly enclose detected text. Each pair of fields represents the X and Y coordinates of a corner of the bounding box.
///  - x1, y1: Coordinates of the top-left corner.
///  - x2, y2: Coordinates of the top-right corner.
///  - x3, y3: Coordinates of the bottom-right corner.
///  - x4, y4: Coordinates of the bottom-left corner.
#[repr(C, packed)]
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct BoundingBox {
    pub x1: f32,
    pub y1: f32,
    pub x2: f32,
    pub y2: f32,
    pub x3: f32,
    pub y3: f32,
    pub x4: f32,
    pub y4: f32,
}
