use super::{Modifier, ModifierReturn, ModifierState, Padding};
use crate::{geometry::Side, space::space_around};

// TODO: option for shifting perpendicular to the side, use with the limiter
// so that it only shifts away from

pub fn shift() -> Shift {
    Shift {
        padding: 0.0.into(),
    }
}

pub struct Shift {
    padding: Padding,
}

impl Shift {
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
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
            if space_on_side < self.padding.sideways {
                let mut new_point = floater.point();
                let shift_amount = space_on_side - self.padding.sideways;
                *new_point.coord_across(side) += match side {
                    Side::Top | Side::Left => -shift_amount,
                    Side::Bottom | Side::Right => shift_amount,
                };
                return ModifierReturn::new().point(new_point);
            }
        }

        ModifierReturn::new()
    }
}
