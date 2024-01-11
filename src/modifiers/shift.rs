use super::{Modifier, ModifierState};
use crate::{geometry::Side, space::space_around};

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
    fn run(&mut self, state: &mut ModifierState) {
        let space = space_around(state.floater(), state.container());
        let adjacent_sides = state.side().adjacents();

        for side in adjacent_sides {
            let space_on_side = space.on_side(side);
            if space_on_side < self.padding {
                *state.floater_mut().point_mut().coord_across(side) += match side {
                    Side::Top | Side::Left => -space_on_side - self.padding,
                    Side::Bottom | Side::Right => space_on_side + self.padding,
                };
                return;
            }
        }
    }
}
