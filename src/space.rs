use crate::{geometry::ElemRect, Side};

/// Positive = overflowing by `amount` pixels.
/// Negative = `amount` pixels left to the boundary.
/// 0 = flush with the boundary.
#[derive(Debug, Clone, Copy)]
pub struct SpaceAround {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl SpaceAround {
    #[must_use]
    pub const fn on_side(&self, side: Side) -> f64 {
        match side {
            Side::Left => self.left,
            Side::Top => self.top,
            Side::Right => self.right,
            Side::Bottom => self.bottom,
        }
    }

    #[must_use]
    pub fn on_side_mut(&mut self, side: Side) -> &mut f64 {
        match side {
            Side::Left => &mut self.left,
            Side::Top => &mut self.top,
            Side::Right => &mut self.right,
            Side::Bottom => &mut self.bottom,
        }
    }

    #[must_use]
    pub fn min(&self) -> f64 { self.left.min(self.top).min(self.right).min(self.bottom) }

    /// Returns the space on all four sides.
    ///
    /// You should not rely on the order of the values.
    #[must_use]
    pub fn on_all_sides(&self) -> std::array::IntoIter<f64, 4> {
        [self.left, self.top, self.right, self.bottom].into_iter()
    }
}

#[must_use]
pub fn space_around(rect: &ElemRect, container: &ElemRect) -> SpaceAround {
    SpaceAround {
        left: rect.left() - container.left(),
        top: rect.top() - container.top(),
        right: container.right() - rect.right(),
        bottom: container.bottom() - rect.bottom(),
    }
}
