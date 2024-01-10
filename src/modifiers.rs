use core::fmt;
use std::vec;

use crate::{vec2::Vec2, ElemRect, ElemSize, Side};

#[derive(Debug)]
pub struct ModifierState {
    pub side: Side,
    pub reference: ElemRect,
    pub floater: ElemSize,
    pub pos: Vec2,
}

pub trait Modifier {
    fn run(&mut self, state: &ModifierState) -> Vec2;
}

impl<F> Modifier for F
where
    F: FnMut(&ModifierState) -> Vec2,
{
    fn run(&mut self, state: &ModifierState) -> Vec2 {
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
    move |ModifierState { side, pos, .. }: &ModifierState| {
        let (x, y) = match side {
            Side::Left => (pos.x - amount, pos.y),
            Side::Right => (pos.x + amount, pos.y),
            Side::Top => (pos.x, pos.y - amount),
            Side::Bottom => (pos.x, pos.y + amount),
        };

        Vec2::new(x, y)
    }
}
