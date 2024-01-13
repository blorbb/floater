use super::{Modifier, ModifierReturn, ModifierState};
use crate::geometry::Side;

pub fn offset(amount: f64) -> impl Modifier {
    move |ModifierState { floater, side, .. }: &_| -> ModifierReturn {
        let pos = floater;
        let (x, y) = match side {
            Side::Left => (pos.x() - amount, pos.y()),
            Side::Right => (pos.x() + amount, pos.y()),
            Side::Top => (pos.x(), pos.y() - amount),
            Side::Bottom => (pos.x(), pos.y() + amount),
        };

        ModifierReturn::new().point_xy(x, y)
    }
}
