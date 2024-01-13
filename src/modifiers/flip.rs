use super::{Modifier, ModifierState};
use crate::{geometry::Side, modifiers::ModifierReturn, space::space_around};

// TODO: flip to side, option to flip to most space as fallback

pub fn flip() -> Flip {
    Flip {
        flip_across: true,
        flip_to_side: false,
        padding: 0.0,
    }
}

pub struct Flip {
    flip_across: bool,
    flip_to_side: bool,
    padding: f64,
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
    pub fn padding(mut self, padding: f64) -> Self {
        self.padding = padding;
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
        let container = container;
        let reference = reference;
        let initial_floater = floater;
        let initial_side = side;

        // has enough space, no need to flip
        if space_around(floater, &container).on_side(*initial_side) > self.padding {
            return ModifierReturn::new();
        }

        // try flip across
        if self.flip_across {
            /// Next number with the same difference betweeen middle and first.
            fn next_equal_diff(first: f64, middle: f64) -> f64 {
                let diff = middle - first;
                middle + diff
            }

            let mut new_floater = *floater;

            match side {
                Side::Left => {
                    *new_floater.x_mut() =
                        next_equal_diff(initial_floater.right(), reference.center().x)
                }
                Side::Right => {
                    *new_floater.x_mut() =
                        next_equal_diff(initial_floater.left(), reference.center().x)
                            - initial_floater.width()
                }
                Side::Top => {
                    *new_floater.y_mut() =
                        next_equal_diff(initial_floater.bottom(), reference.center().y)
                }
                Side::Bottom => {
                    *new_floater.y_mut() =
                        next_equal_diff(initial_floater.top(), reference.center().y)
                            - initial_floater.height()
                }
            };

            if space_around(&new_floater, &container).on_side(initial_side.opposite())
                > self.padding
            {
                return ModifierReturn::new()
                    .point(new_floater.point())
                    .side(initial_side.opposite());
            }
        }

        if self.flip_to_side {
            todo!()
        }
        ModifierReturn::new()
    }
}
