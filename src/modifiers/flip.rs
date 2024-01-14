use super::{Modifier, ModifierState, Padding};
use crate::{
    compute_position_from_placement, geometry::ElemRect, modifiers::ModifierReturn,
    space::space_around,
};

// TODO: flip to side, option to flip to most space as fallback

pub fn flip() -> Flip {
    Flip {
        flip_across: true,
        flip_to_side: false,
        padding: 0.0.into(),
    }
}

pub struct Flip {
    flip_across: bool,
    flip_to_side: bool,
    padding: Padding,
}

impl Flip {
    /// Whether to flip to the opposite side of the reference if no space is
    /// left.
    pub fn flip_across(mut self, b: bool) -> Self {
        self.flip_across = b;
        self
    }

    /// Whether to flip to an adjacent side of the reference if the initial and
    /// opposite sides do not fit.
    pub fn flip_to_side(mut self, b: bool) -> Self {
        self.flip_to_side = b;
        self
    }

    /// Allowed space from the container boundary before it attempts to flip.
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }
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
        // has enough space, no need to flip
        if space_around(floater, container).on_side(*side) > self.padding.outward {
            return ModifierReturn::new();
        }

        // try flip across
        if self.flip_across {
            let opp = side.opposite();
            let new_pos = compute_position_from_placement(*reference, floater.size(), opp);
            let new_floater = ElemRect::from_parts(new_pos, floater.size());

            if space_around(&new_floater, container).on_side(opp) > self.padding.outward {
                return ModifierReturn::new().point(new_floater.point()).side(opp);
            }
        }

        if self.flip_to_side {
            todo!()
        }
        ModifierReturn::new()
    }
}
