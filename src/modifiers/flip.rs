use super::{Modifier, ModifierState};
use crate::{
    compute_position_from_placement, geometry::ElemRect, impl_padding_builder,
    modifiers::ModifierReturn, padding::Padding, space::space_around,
};

// TODO: flip to side, option to flip to most space as fallback

#[must_use]
pub fn flip() -> Flip {
    Flip {
        flip_main: true,
        flip_cross: false,
        check_main_axis: true,
        check_cross_axis: false,
        padding: Padding::default(),
    }
}

pub struct Flip {
    flip_main: bool,
    flip_cross: bool,
    check_main_axis: bool,
    check_cross_axis: bool,
    padding: Padding,
}

impl Flip {
    /// Whether to flip to the opposite side of the reference if no space is
    /// left.
    #[must_use]
    pub const fn flip_main(mut self, b: bool) -> Self {
        self.flip_main = b;
        self
    }

    /// Whether to flip to an adjacent side of the reference if the initial and
    /// opposite sides do not fit.
    #[must_use]
    pub const fn flip_cross(mut self, b: bool) -> Self {
        self.flip_cross = b;
        self
    }

    #[must_use]
    pub const fn check_main_axis(mut self, b: bool) -> Self {
        self.check_main_axis = b;
        self
    }

    #[must_use]
    pub const fn check_cross_axis(mut self, b: bool) -> Self {
        self.check_cross_axis = b;
        self
    }

    impl_padding_builder!(padding);
}

impl Modifier for Flip {
    fn run(
        &mut self,
        ModifierState {
            reference,
            floater,
            container,
            side,
            ..
        }: &ModifierState,
    ) -> ModifierReturn {
        let fallbacks = {
            let mut fallbacks = vec![*side];
            // TODO: more configurable fallback options
            self.flip_main.then(|| fallbacks.push(side.opposite()));
            self.flip_cross.then(|| fallbacks.extend(side.adjacents()));
            fallbacks
        };

        for side in fallbacks {
            let new_pos = compute_position_from_placement(*reference, floater.size(), side);
            let new_floater = ElemRect::from_parts(new_pos, floater.size());
            let space = space_around(&new_floater, container);

            // check if they are satisfactory
            if self.check_main_axis && space.on_side(side) < self.padding.outward {
                continue;
            }
            if self.check_cross_axis
                && side
                    .adjacents()
                    .into_iter()
                    .any(|side| space.on_side(side) < self.padding.cross)
            {
                continue;
            }
            // is satisfactory: use this side
            return ModifierReturn::new().point(new_pos).side(side);
        }

        // falback to the initial placement

        ModifierReturn::new()
    }
}
