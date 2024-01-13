use super::{size::ElemSize, Vec2};

/// A rectangle placed on a viewport (scrolling context).
///
/// Positive `x` goes right, positive `y` goes down. Width and height must be
/// non-negative.
#[derive(Debug, Default, Clone, Copy)]
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

    pub fn from_parts(point: Vec2, size: ElemSize) -> Self { Self { point, size } }

    pub fn x(&self) -> f64 { self.point.x }
    pub fn y(&self) -> f64 { self.point.y }
    pub fn width(&self) -> f64 { self.size.width() }
    pub fn height(&self) -> f64 { self.size.height() }

    pub fn left(&self) -> f64 { self.x() }
    pub fn right(&self) -> f64 { self.x() + self.width() }
    pub fn top(&self) -> f64 { self.y() }
    pub fn bottom(&self) -> f64 { self.y() + self.height() }

    pub fn xy(&self) -> (f64, f64) { (self.x(), self.y()) }
    pub fn point(&self) -> Vec2 { self.point }
    pub fn size(&self) -> ElemSize { self.size }

    pub fn point_mut(&mut self) -> &mut Vec2 { &mut self.point }
    pub fn size_mut(&mut self) -> &mut ElemSize { &mut self.size }

    pub fn x_mut(&mut self) -> &mut f64 { &mut self.point.x }
    pub fn y_mut(&mut self) -> &mut f64 { &mut self.point.y }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x() + self.width() / 2.0,
            self.y() + self.height() / 2.0,
        )
    }

    // pub fn intersect(&self, other: &Self) -> Self {
    //     let left = self.x().max(other.x());
    //     let top = self.y().max(other.y());
    //     let right = self.right().min(other.right());
    //     let bottom = self.bottom().min(other.bottom());

    //     Self::new(left, top, right - left, bottom - top)
    // }
}
