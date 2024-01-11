#[rustfmt::skip] pub mod offset;
#[rustfmt::skip] pub use offset::offset;
#[rustfmt::skip] pub mod flip;
#[rustfmt::skip] pub use flip::flip;
#[rustfmt::skip] pub mod shift;
#[rustfmt::skip] pub use shift::shift;

use core::fmt;
use std::vec;

use crate::{geometry::ElemRect, Side};

/// Allows each modifier to read position data.
///
/// Modifiers should mutate `floater` and `side` where required. All other
/// fields are read-only.
#[derive(Debug)]
pub struct ModifierState {
    reference: ElemRect,
    // after initial placement, floater has a position too
    floater: ElemRect,
    container: ElemRect,
    side: Side,
}

impl ModifierState {
    pub fn new(reference: ElemRect, floater: ElemRect, container: ElemRect, side: Side) -> Self {
        Self {
            reference,
            floater,
            container,
            side,
        }
    }

    pub fn reference(&self) -> &ElemRect { &self.reference }
    pub fn floater(&self) -> &ElemRect { &self.floater }
    pub fn container(&self) -> &ElemRect { &self.container }
    pub fn side(&self) -> Side { self.side }

    pub fn floater_mut(&mut self) -> &mut ElemRect { &mut self.floater }
    pub fn side_mut(&mut self) -> &mut Side { &mut self.side }
}

pub trait Modifier {
    fn run(&mut self, state: &mut ModifierState);
}

impl<F> Modifier for F
where
    F: FnMut(&mut ModifierState),
{
    fn run(&mut self, state: &mut ModifierState) { self(state) }
}

pub struct Modifiers<'a>(Vec<&'a mut dyn Modifier>);

impl<'a> Modifiers<'a> {
    pub fn new() -> Self { Self(Vec::new()) }

    pub fn push(&mut self, modifier: &'a mut impl Modifier) { self.0.push(modifier) }
}

impl Default for Modifiers<'_> {
    fn default() -> Self { Self::new() }
}

impl<'a> IntoIterator for Modifiers<'a> {
    type Item = &'a mut dyn Modifier;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter { self.0.into_iter() }
}

impl fmt::Debug for Modifiers<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result { f.debug_tuple("Modifiers").finish() }
}
