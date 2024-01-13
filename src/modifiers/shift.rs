use super::{Modifier, ModifierReturn, ModifierState};
use crate::{geometry::Side, space::space_around};

// TODO: option for shifting perpendicular to the side, use with the limiter
// so that it only shifts away from

pub fn shift() -> Shift { Shift { padding: 0.0 } }

pub struct Shift {
    padding: f64,
}

impl Shift {
    pub fn padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }
}

impl Modifier for Shift {
    fn run(
        &mut self,
        ModifierState {
            floater,
            container,
            side,
            ..
        }: &ModifierState,
    ) -> ModifierReturn {
        let space = space_around(floater, container);

        for side in side.adjacents() {
            let space_on_side = space.on_side(side);
            if space_on_side < self.padding {
                let mut new_point = floater.point();
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
