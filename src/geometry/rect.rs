use super::{size::ElemSize, Vec2};

/// A rectangle placed on a viewport (scrolling context).
///
/// Positive `x` goes right, positive `y` goes down. Width and height must be
/// non-negative.
#[derive(Debug, Default)]
pub struct ElemRect {
    point: Vec2,
    size: ElemSize,
}

impl ElemRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            point: Vec2::new(x, y),
            size: ElemSize::new(width, height),
        }
    }

    pub fn x(&self) -> f64 { self.point.x }
    pub fn y(&self) -> f64 { self.point.y }
    pub fn width(&self) -> f64 { self.size.width() }
    pub fn height(&self) -> f64 { self.size.height() }

    pub fn xy(&self) -> (f64, f64) { (self.x(), self.y()) }
    pub fn point(&self) -> Vec2 { self.point }

    pub fn set_point(&mut self, point: Vec2) { self.point = point; }
    pub fn set_size(&mut self, size: ElemSize) { self.size = size; }

    pub fn left(&self) -> f64 { self.x() }
    pub fn right(&self) -> f64 { self.x() + self.width() }
    pub fn top(&self) -> f64 { self.y() }
    pub fn bottom(&self) -> f64 { self.y() + self.height() }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x() + self.width() / 2.0,
            self.y() + self.height() / 2.0,
        )
    }
}
