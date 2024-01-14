use super::{Side, Vec2};

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

    #[must_use]
    pub const fn dim_along(&self, side: Side) -> f64 { self.as_vec2().coord_along(side) }

    #[must_use]
    pub const fn dim_across(&self, side: Side) -> f64 { self.as_vec2().coord_across(side) }
}
