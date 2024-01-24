use super::{Side, Vec2};

/// The dimensions of a rectangle.
///
/// Note that the width and height values may be negative, e.g. to represent
/// overlapping spaces.
#[derive(Debug, Default, Clone, Copy)]
pub struct ElemSize(Vec2);

impl ElemSize {
    #[must_use]
    pub const fn new(width: f64, height: f64) -> Self { Self(Vec2::new(width, height)) }

    #[must_use]
    pub const fn width(&self) -> f64 { self.0.x }
    #[must_use]
    pub const fn height(&self) -> f64 { self.0.y }

    #[must_use]
    pub const fn as_vec2(&self) -> &Vec2 { &self.0 }

    /// Returns the length of the rectangle parallel to the provided side.
    #[must_use]
    pub const fn dim_cross(&self, side: Side) -> f64 { self.as_vec2().coord_cross(side) }

    /// Returns the length of the rectangle perpendicular to the provided side.
    #[must_use]
    pub const fn dim_main(&self, side: Side) -> f64 { self.as_vec2().coord_main(side) }
}
