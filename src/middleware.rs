use core::fmt;
use std::vec;

use crate::{ElemRect, ElemSize, Side, Vec2};

#[derive(Debug)]
pub struct MiddlewareState {
    pub placement: Side,
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

pub struct Middlewares(Vec<Box<dyn Middleware>>);

impl Middlewares {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, mw: impl Middleware + 'static) -> &mut Self {
        self.0.push(Box::new(mw));
        self
    }
}

impl Default for Middlewares {
    fn default() -> Self {
        Self::new()
    }
}

impl IntoIterator for Middlewares {
    type Item = Box<dyn Middleware>;

    type IntoIter = vec::IntoIter<Self::Item>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.into_iter()
    }
}

impl fmt::Debug for Middlewares {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        f.debug_tuple("Middlewares").finish()
    }
}

pub fn offset(amount: f64) -> impl Middleware {
    move |state: &MiddlewareState| Vec2::new(amount, amount)
}

pub fn arrow(data: &mut i32) -> impl Middleware + '_ {
    |state: &MiddlewareState| {
        *data += 1;
        Vec2::new(1.0, 1.0)
    }
}
