use core::fmt;
use std::vec;

use crate::{vec2::Vec2, ElemRect, ElemSize, Side};

#[derive(Debug)]
pub struct MiddlewareState {
    pub side: Side,
    pub reference: ElemRect,
    pub tooltip: ElemSize,
    pub pos: Vec2,
}

pub trait Middleware {
    fn run(&mut self, state: &MiddlewareState) -> Vec2;
}

impl<F> Middleware for F
where
    F: FnMut(&MiddlewareState) -> Vec2,
{
    fn run(&mut self, state: &MiddlewareState) -> Vec2 {
        self(state)
    }
}

pub struct Middlewares<'a>(Vec<&'a mut dyn Middleware>);

impl<'a> Middlewares<'a> {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, mw: &'a mut impl Middleware) -> &mut Self {
        self.0.push(mw);
        self
    }
}

impl Default for Middlewares<'_> {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for Middlewares<'a> {
    type Item = &'a mut dyn Middleware;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Debug for Middlewares<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Middlewares").finish()
    }
}

// actual middleware //

pub fn offset(amount: f64) -> impl Middleware {
    move |MiddlewareState { side, pos, .. }: &MiddlewareState| {
        let (x, y) = match side {
            Side::Left => (pos.x - amount, pos.y),
            Side::Right => (pos.x + amount, pos.y),
            Side::Top => (pos.x, pos.y - amount),
            Side::Bottom => (pos.x, pos.y + amount),
        };

        Vec2::new(x, y)
    }
}
