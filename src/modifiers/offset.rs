use crate::geometry::Vec2;
use crate::Side;
use super::ModifierState;
use super::Modifier;

pub fn offset(amount: f64) -> impl Modifier {
    move |state: &mut ModifierState| {
        let pos = state.floater();
        let (x, y) = match state.side() {
            Side::Left => (pos.x() - amount, pos.y()),
            Side::Right => (pos.x() + amount, pos.y()),
            Side::Top => (pos.x(), pos.y() - amount),
            Side::Bottom => (pos.x(), pos.y() + amount),
        };

        state.floater_mut().set_point(Vec2::new(x, y))
    }
}
