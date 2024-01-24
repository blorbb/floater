use super::{Modifier, ModifierState, StateUpdate};
use crate::geometry::Side;

#[must_use]
pub fn offset(amount: f64) -> impl Modifier {
    move |ModifierState { floater, side, .. }: &_| -> StateUpdate {
        let pos = floater;
        let (x, y) = match side {
            Side::Left => (pos.x() - amount, pos.y()),
            Side::Right => (pos.x() + amount, pos.y()),
            Side::Top => (pos.x(), pos.y() - amount),
            Side::Bottom => (pos.x(), pos.y() + amount),
        };

        StateUpdate::new().point_xy(x, y)
    }
}
