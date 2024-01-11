pub mod geometry;
pub mod modifiers;

use std::mem;

use geometry::{ElemRect, ElemSize};
use modifiers::{Modifier, ModifierState, Modifiers};

#[derive(Debug, Default)]
pub struct PositionOpts<'a> {
    side: Side,
    modifiers: Modifiers<'a>,
}

impl<'a> PositionOpts<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn with_side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn add_modifier(mut self, modifier: &'a mut impl Modifier) -> Self {
        self.modifiers.push(modifier);
        self
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default)]
pub enum Side {
    Left,
    Top,
    Right,
    #[default]
    Bottom,
}

pub fn compute_position(reference: ElemRect, floater: ElemSize, opts: PositionOpts) -> ElemRect {
    let x = match opts.side {
        Side::Top | Side::Bottom => reference.center().x - floater.width() / 2.0,
        Side::Left => reference.left() - floater.width(),
        Side::Right => reference.right(),
    };

    let y = match opts.side {
        Side::Left | Side::Right => reference.center().y - floater.height() / 2.0,
        Side::Top => reference.top() - floater.height(),
        Side::Bottom => reference.bottom(),
    };

    let mut state = ModifierState::new(
        reference,
        ElemRect::new(x, y, floater.width(), floater.height()),
        opts.side,
    );

    for modifier in opts.modifiers {
        modifier.run(&mut state);
    }

    mem::take(state.floater_mut())
}
