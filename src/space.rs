use crate::{geometry::ElemRect, Side};

/// Positive = overflowing by `amount` pixels.
/// Negative = `amount` pixels left to the boundary.
/// 0 = flush with the boundary.
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
    pub fn min(&self) -> f64 { self.left.min(self.top).min(self.right).min(self.bottom) }
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
