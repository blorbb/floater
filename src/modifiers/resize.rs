use super::{Modifier, ModifierState, StateUpdate};
use crate::{
    compute_placement_position,
    geometry::{side::Orientation, ElemRect, ElemSize, Side},
    impl_padding_builder,
    padding::Padding,
    space::space_around,
};

/// Resizes the floater based on the provided application function.
///
/// The provided function will be run when the modifier is applied. This
/// function should modify the real floater element and return the floater's new
/// size. This allows you to apply a size on one axis, and have the platform
/// recalculate the opposite axis' size.
pub fn resize<F: FnMut(&ResizeState) -> ElemSize>(recalculator: F) -> Resize<F> {
    Resize {
        padding: Padding::splat(0.0),
        recalculator,
    }
}

pub struct Resize<F> {
    padding: Padding,
    recalculator: F,
}

#[derive(Debug)]
pub struct ResizeState {
    pub available: ElemSize,
    pub state: ModifierState,
}

impl<F> Resize<F> {
    impl_padding_builder!(padding);
}

impl<F: FnMut(&ResizeState) -> ElemSize> Modifier for Resize<F> {
    fn run(&mut self, state: &ModifierState) -> StateUpdate {
        let ModifierState {
            reference,
            floater,
            container,
            side,
        } = *state;

        let mut space = space_around(&floater, &container);
        // remove the space from the edge of the reference to the end of the container
        let invalid_space = match side {
            Side::Left => container.right() - reference.left(),
            Side::Top => container.bottom() - reference.top(),
            Side::Right => reference.right() - container.left(),
            Side::Bottom => reference.bottom() - container.top(),
        };

        *space.on_side_mut(side.opposite()) -= invalid_space;

        let (padding_width, padding_height) = match side.orientation() {
            Orientation::Horizontal => (
                self.padding.cross,
                self.padding.outward + self.padding.inward,
            ),
            Orientation::Vertical => (
                self.padding.outward + self.padding.inward,
                self.padding.cross,
            ),
        };

        let space = ElemSize::new(
            space.left + space.right + floater.width() - padding_width,
            space.top + space.bottom + floater.height() - padding_height,
        );

        let new_size = (self.recalculator)(&ResizeState {
            available: space,
            state: *state,
        });

        let new_floater_pos = compute_placement_position(reference, new_size, side);
        let new_floater = ElemRect::from_parts(new_floater_pos, new_size);

        StateUpdate::new().floater(new_floater)
    }
}
