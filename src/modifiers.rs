use core::fmt;
use std::vec;

use crate::{geometry::{Vec2, ElemRect}, Side};

/// Allows each modifier to read position data.
///
/// Modifiers should mutate `floater` and `side` where required. `reference` is
/// read-only.
#[derive(Debug)]
pub struct ModifierState {
    reference: ElemRect,
    // after initial placement, floater has a position too
    floater: ElemRect,
    side: Side,
}

impl ModifierState {
    pub fn new(reference: ElemRect, floater: ElemRect, side: Side) -> Self {
        Self {
            reference,
            floater,
            side,
        }
    }

    pub fn reference(&self) -> &ElemRect {
        &self.reference
    }

    pub fn floater(&self) -> &ElemRect {
        &self.floater
    }

    pub fn floater_mut(&mut self) -> &mut ElemRect {
        &mut self.floater
    }

    pub fn side(&self) -> Side {
        self.side
    }

    pub fn side_mut(&mut self) -> &mut Side {
        &mut self.side
    }
}

pub trait Modifier {
    fn run(&mut self, state: &mut ModifierState);
}

impl<F> Modifier for F
where
    F: FnMut(&mut ModifierState),
{
    fn run(&mut self, state: &mut ModifierState) {
        self(state)
    }
}

pub struct Modifiers<'a>(Vec<&'a mut dyn Modifier>);

impl<'a> Modifiers<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, modifier: &'a mut impl Modifier) {
        self.0.push(modifier)
    }
}

impl Default for Modifiers<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for Modifiers<'a> {
    type Item = &'a mut dyn Modifier;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Debug for Modifiers<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Modifiers").finish()
    }
}

// actual modifiers //

pub fn offset(amount: f64) -> impl Modifier {
    move |state: &mut ModifierState| {
        let pos = state.floater();
        let (x, y) = match state.side() {
            Side::Left => (pos.x() - amount, pos.y()),
            Side::Right => (pos.x() + amount, pos.y()),
            Side::Top => (pos.x(), pos.y() - amount),
            Side::Bottom => (pos.x(), pos.y() + amount),
        };

        state.floater_mut().set_point(Vec2::new(x, y))
    }
}
