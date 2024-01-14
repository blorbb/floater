pub mod geometry;
pub mod modifiers;
pub mod space;
#[cfg(feature = "web-utils")]
pub mod web;

use geometry::{ElemRect, ElemSize, Side, Vec2};
use modifiers::{Modifier, ModifierState, Modifiers};

#[derive(Debug, Default)]
pub struct PositionOpts<'a> {
    side: Side,
    modifiers: Modifiers<'a>,
}

impl<'a> PositionOpts<'a> {
    #[must_use]
    pub fn new() -> Self { Self::default() }

    #[must_use]
    pub const fn with_side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    /// Can also pass in an Option to only add the modifier if it is [`Some`].
    ///
    /// This is intended to be used with [`bool::then`] to conditionally use a
    /// modifier.
    #[must_use]
    pub fn add_modifier<M: Modifier + 'a>(
        mut self,
        modifier: impl Into<Option<&'a mut M>>,
    ) -> Self {
        if let Some(m) = modifier.into() {
            self.modifiers.push(m);
        }
        self
    }
}

/// Computes the required position of the floater given only its side and no
/// modifiers.
///
/// This is intended to only be used by modifiers - use [`compute_position`]
/// otherwise.
#[must_use]
pub fn compute_position_from_placement(reference: ElemRect, floater: ElemSize, side: Side) -> Vec2 {
    let x = match side {
        Side::Top | Side::Bottom => reference.center().x - floater.width() / 2.0,
        Side::Left => reference.left() - floater.width(),
        Side::Right => reference.right(),
    };

    let y = match side {
        Side::Left | Side::Right => reference.center().y - floater.height() / 2.0,
        Side::Top => reference.top() - floater.height(),
        Side::Bottom => reference.bottom(),
    };

    Vec2::new(x, y)
}

/// `reference` should be relative to the nearest scrolling context.
///
/// The returned position will also be relative to the same context.
///
/// `container` is the section of the scrolling context that is visible.
#[must_use]
pub fn compute_position(
    reference: ElemRect,
    floater: ElemSize,
    container: ElemRect,
    opts: PositionOpts,
) -> ElemRect {
    let point = compute_position_from_placement(reference, floater, opts.side);

    let mut state = ModifierState::new(
        reference,
        ElemRect::from_parts(point, floater),
        container,
        opts.side,
    );

    for modifier in opts.modifiers {
        let res = modifier.run(&state);
        state.update_with(&res);
    }

    state.floater
}
