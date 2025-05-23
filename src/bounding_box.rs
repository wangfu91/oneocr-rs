use serde::Serialize;

use crate::ffi::RawBBox;

/// This `Point` struct represents a point in 2D space with X and Y coordinates.
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({:.2}, {:.2})", self.x, self.y)
    }
}

/// This `BoundingBox` struct represents a bounding box in 2D space, used for OCR to tightly enclose detected text.
#[derive(Debug, Clone, Copy, Default, Serialize)]
pub struct BoundingBox {
    pub top_left: Point,
    pub top_right: Point,
    pub bottom_right: Point,
    pub bottom_left: Point,
}

impl BoundingBox {
    /// Creates a new `BoundingBox` from a FFI `RawBBox`.
    pub(crate) fn new(bbox: RawBBox) -> Self {
        BoundingBox {
            top_left: Point {
                x: bbox.x1,
                y: bbox.y1,
            },
            top_right: Point {
                x: bbox.x2,
                y: bbox.y2,
            },
            bottom_right: Point {
                x: bbox.x3,
                y: bbox.y3,
            },
            bottom_left: Point {
                x: bbox.x4,
                y: bbox.y4,
            },
        }
    }
}

impl std::fmt::Display for BoundingBox {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "[ ⌜ {}, ⌝ {}, ⌟ {}, ⌞ {}",
            self.top_left, self.top_right, self.bottom_right, self.bottom_left,
        )
    }
}
