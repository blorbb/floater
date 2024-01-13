use super::{Modifier, ModifierReturn, ModifierState};
use crate::geometry::{Side, Vec2};

pub fn offset(amount: f64) -> impl Modifier {
    move |state: &ModifierState| -> ModifierReturn {
        let pos = state.floater();
        let (x, y) = match state.side() {
            Side::Left => (pos.x() - amount, pos.y()),
            Side::Right => (pos.x() + amount, pos.y()),
            Side::Top => (pos.x(), pos.y() - amount),
            Side::Bottom => (pos.x(), pos.y() + amount),
        };

        ModifierReturn::new().point(Vec2::new(x, y))
    }
}
