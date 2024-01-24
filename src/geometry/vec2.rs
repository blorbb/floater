use std::ops;

use super::{side::Axis, Side};

#[derive(Debug, Clone, Copy, Default)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    #[must_use]
    pub const fn new(x: f64, y: f64) -> Self { Self { x, y } }

    /// Returns the component of the coordinate that is in the direction
    /// parallel to the provided side.
    ///
    /// left/right => y, top/bottom => x.
    #[must_use]
    pub const fn coord_cross(&self, side: Side) -> f64 {
        match side.axis() {
            Axis::Vertical => self.x,
            Axis::Horizontal => self.y,
        }
    }

    /// Returns the component of the coordinate that is in the direction
    /// parallel to the provided side.
    ///
    /// left/right => y, top/bottom => x.
    pub fn coord_cross_mut(&mut self, side: Side) -> &mut f64 {
        match side.axis() {
            Axis::Vertical => &mut self.x,
            Axis::Horizontal => &mut self.y,
        }
    }

    /// Returns the component of the coordinate that is in the direction
    /// perpendicular to the provided side.
    ///
    /// left/right => x, top/bottom => y.
    #[must_use]
    pub const fn coord_main(&self, side: Side) -> f64 {
        match side.axis() {
            Axis::Horizontal => self.x,
            Axis::Vertical => self.y,
        }
    }

    /// Returns the component of the coordinate that is in the direction
    /// perpendicular to the provided side.
    ///
    /// left/right => x, top/bottom => y.
    pub fn coord_main_mut(&mut self, side: Side) -> &mut f64 {
        match side.axis() {
            Axis::Horizontal => &mut self.x,
            Axis::Vertical => &mut self.y,
        }
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output { Self::new(-self.x, -self.y) }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output { Self::new(self.x + rhs.x, self.y + rhs.y) }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output { self + -rhs }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output { Self::new(self.x * rhs, self.y * rhs) }
}

impl ops::Div<f64> for Vec2 {
    type Output = Self;

    fn div(self, rhs: f64) -> Self::Output { self * (1.0 / rhs) }
}
