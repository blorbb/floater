use super::{Modifier, ModifierState, StateUpdate};
use crate::{
    compute_placement_position,
    geometry::{side::Axis, ElemRect, ElemSize, Side},
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
///
/// The parameters passed in to the function are the available space (as an
/// [`ElemSize`]; values may be negative) and the current modifier state.
pub fn resize<F: FnMut(&ElemSize, &ModifierState) -> ElemSize>(recalculator: F) -> Resize<F> {
    Resize {
        padding: Padding::splat(0.0),
        recalculator,
    }
}

pub struct Resize<F> {
    padding: Padding,
    recalculator: F,
}

impl<F> Resize<F> {
    impl_padding_builder!(padding);
}

impl<F: FnMut(&ElemSize, &ModifierState) -> ElemSize> Modifier for Resize<F> {
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

        let (padding_width, padding_height) = match side.axis() {
            Axis::Vertical => (
                self.padding.cross,
                self.padding.outward + self.padding.inward,
            ),
            Axis::Horizontal => (
                self.padding.outward + self.padding.inward,
                self.padding.cross,
            ),
        };

        let space = ElemSize::new(
            space.left + space.right + floater.width() - padding_width,
            space.top + space.bottom + floater.height() - padding_height,
        );

        let new_size = (self.recalculator)(&space, state);

        let new_floater_pos = compute_placement_position(reference, new_size, side);
        let new_floater = ElemRect::from_parts(new_floater_pos, new_size);

        StateUpdate::new().floater(new_floater)
    }
}
