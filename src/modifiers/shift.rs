use super::{Modifier, ModifierReturn, ModifierState};
use crate::{geometry::Side, space::space_around};

// TODO: option for shifting perpendicular to the side, use with the limiter
// so that it only shifts away from

pub fn shift() -> Shift { Shift { padding: 0.0 } }

pub struct Shift {
    padding: f64,
}

impl Shift {
    pub fn with_padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }
}

impl Modifier for Shift {
    fn run(&mut self, state: &ModifierState) -> ModifierReturn {
        let space = space_around(state.floater(), state.container());

        for side in state.side().adjacents() {
            let space_on_side = space.on_side(side);
            if space_on_side < self.padding {
                let mut new_point = state.floater().point();
                *new_point.coord_across(side) += match side {
                    Side::Top | Side::Left => -(space_on_side - self.padding),
                    Side::Bottom | Side::Right => space_on_side - self.padding,
                };
                return ModifierReturn::new().point(new_point);
            }
        }

        ModifierReturn::new()
    }
}
