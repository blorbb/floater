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
    pub fn on_side(&self, side: Side) -> f64 {
        match side {
            Side::Left => self.left,
            Side::Top => self.top,
            Side::Right => self.right,
            Side::Bottom => self.bottom,
        }
    }
}

pub fn space_around(rect: &ElemRect, container: &ElemRect) -> SpaceAround {
    SpaceAround {
        left: container.left() - rect.left(),
        top: container.top() - rect.top(),
        right: rect.right() - container.right(),
        bottom: rect.bottom() - container.bottom(),
    }
}
