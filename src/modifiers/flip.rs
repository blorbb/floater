use super::{Modifier, ModifierState};
use crate::{
    compute_placement_position,
    geometry::{ElemRect, Side, Vec2},
    impl_padding_builder,
    modifiers::StateUpdate,
    padding::Padding,
    space::{space_around, Space},
};

#[must_use]
pub fn flip() -> Flip {
    Flip {
        flip_main: true,
        flip_cross: false,
        check_main_axis: true,
        check_cross_axis: false,
        padding: Padding::default(),
        fallback_method: FallbackMethod::default(),
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum FallbackMethod {
    /// Go back to the initial side if none fully fit.
    Initial,
    /// Choose the best fitting side if none fully fit.
    #[default]
    BestFit,
}

#[allow(clippy::struct_excessive_bools)]
pub struct Flip {
    flip_main: bool,
    flip_cross: bool,
    check_main_axis: bool,
    check_cross_axis: bool,
    padding: Padding,
    fallback_method: FallbackMethod,
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

    /// Which fallback method to use if no sides can fit the floater.
    #[must_use]
    pub const fn fallback_method(mut self, f: FallbackMethod) -> Self {
        self.fallback_method = f;
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
    ) -> StateUpdate {
        let fallbacks = {
            let mut fallbacks = vec![*side];
            // TODO: more configurable fallback options
            self.flip_main.then(|| fallbacks.push(side.opposite()));
            self.flip_cross.then(|| fallbacks.extend(side.adjacents()));
            fallbacks
        };

        let mut space_info: Vec<(Side, Space, Vec2)> = Vec::new();

        for side in fallbacks {
            let new_pos = compute_placement_position(*reference, floater.size(), side);
            let new_floater = ElemRect::from_parts(new_pos, floater.size());
            let space = space_around(&new_floater, container);

            if (self.check_main_axis && space.on_side(side) < self.padding.outward)
                || (self.check_cross_axis
                    && side
                        .adjacents()
                        .any(|side| space.on_side(side) < self.padding.cross))
            {
                // push in here to avoid unnecessary allocation if the first side works fine
                space_info.push((side, space, new_pos));
                continue;
            }

            // enough space: use this side
            return StateUpdate::new().point(new_pos).side(side);
        }

        match self.fallback_method {
            FallbackMethod::Initial => StateUpdate::new(),
            FallbackMethod::BestFit => {
                // score the best fit by the sides that have the least amount of overflow.
                // each score should be negative, with the magnitude indicating the total amount
                // of overflow.
                let scores = space_info.iter().map(|(_, space, _)| {
                    space
                        .on_all_sides()
                        .filter(|space| *space < 0.0)
                        .sum::<f64>()
                });

                let best_fit_index = scores
                    .enumerate()
                    .max_by(|a, b| a.1.total_cmp(&b.1))
                    .expect("should have at least one fallback side")
                    .0;

                let (best_side, _, best_point) = space_info[best_fit_index];

                StateUpdate::new().side(best_side).point(best_point)
            }
        }
    }
}
