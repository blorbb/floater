use super::{Modifier, ModifierState};
use crate::{geometry::Side, space::space_around};

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
    pub fn with_padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }
}

impl Modifier for Flip {
    fn run(&mut self, state: &mut ModifierState) {
        let container = *state.container();
        let reference = *state.reference();
        let initial_floater = *state.floater();
        let initial_side = state.side();

        let reset_state = |state: &mut ModifierState| {
            *state.floater_mut() = initial_floater;
            *state.side_mut() = initial_side;
        };

        let space = space_around(state.floater(), &container);
        // has enough space, no need to flip
        if space.min() > self.padding {
            return;
        }

        // try flip across
        // also check that the flip requirement wasn't because of no space *parallel*
        // to the side: would require a flip to an adjacent side
        if self.flip_across
            && initial_side
                .adjacents()
                .into_iter()
                .all(|side| space.on_side(side) > self.padding)
        {
            /// Next number with the same difference betweeen middle and first.
            fn next_equal_diff(first: f64, middle: f64) -> f64 {
                let diff = middle - first;
                middle + diff
            }

            match state.side() {
                Side::Left => {
                    *state.floater_mut().x_mut() =
                        next_equal_diff(initial_floater.right(), reference.center().x)
                }
                Side::Right => {
                    *state.floater_mut().x_mut() =
                        next_equal_diff(initial_floater.left(), reference.center().x)
                            - initial_floater.width()
                }
                Side::Top => {
                    *state.floater_mut().y_mut() =
                        next_equal_diff(initial_floater.bottom(), reference.center().y)
                }
                Side::Bottom => {
                    *state.floater_mut().y_mut() =
                        next_equal_diff(initial_floater.top(), reference.center().y)
                            - initial_floater.height()
                }
            };
            *state.side_mut() = initial_side.opposite();

            if space_around(state.floater(), &container).on_side(state.side()) > self.padding {
                return;
            } else {
                reset_state(state)
            }
        }

        if self.flip_to_side {
            todo!()
        }
    }
}
