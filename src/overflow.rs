use crate::{geometry::ElemRect, Side};

/// Positive = overflowing by `amount` pixels.
/// Negative = `amount` pixels left to the boundary.
/// 0 = flush with the boundary.
pub struct OverflowAmounts {
    pub left: f64,
    pub top: f64,
    pub right: f64,
    pub bottom: f64,
}

impl OverflowAmounts {
    pub fn of_side(&self, side: Side) -> f64 {
        match side {
            Side::Left => self.left,
            Side::Top => self.top,
            Side::Right => self.right,
            Side::Bottom => self.bottom,
        }
    }
}

pub fn overflow_of(rect: ElemRect, container: ElemRect) -> OverflowAmounts {
    OverflowAmounts {
        left: rect.left() - container.left(),
        top: rect.top() - container.top(),
        right: container.right() - rect.right(),
        bottom: container.bottom() - rect.bottom(),
    }
}
