use core::fmt;
use std::{vec, any::Any};

use crate::{ElemRect, ElemSize, Side, Vec2};

#[derive(Debug)]
pub struct MiddlewareState {
    pub placement: Side,
    pub reference: ElemRect,
    pub tooltip: ElemSize,
    pub pos: Vec2,
}

pub trait Middleware: 'static {
    type Extra;
    fn run(&self, state: &MiddlewareState) -> (Vec2, Self::Extra);
}

pub struct Middlewares(Vec<Box<dyn Middleware<Extra = Box<dyn Any>>>>);

impl Middlewares {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn add(&mut self, mw: impl Middleware<Extra = Box<dyn Any>>) -> &mut Self {
        self.0.push(Box::new(mw));
        self
    }
}

impl Default for Middlewares {
    fn default() -> Self {
        Self::new()
    }
}

impl<'a> IntoIterator for Middlewares {
    type Item = Box<dyn Middleware<Extra = Box<dyn Any>>>;

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

pub struct MiddlewareData(Vec<Box<dyn Any>>);

impl MiddlewareData {
    pub fn new() -> Self {
        Self(Vec::new())
    }

    pub fn push(&mut self, data: Box<dyn Any>) {
        self.0.push(data)
    }

    pub fn fetch<T: Middleware>(&mut self) -> Option<T::Extra> {
        for i in 0..self.0.len() {
            if self.0[i].downcast_ref::<T::Extra>().is_some() {
                return Some(*self.0.remove(i).downcast().unwrap())
            }
        }

        None
    }
}

pub struct Offset(f64);

impl Middleware for Offset {
    type Extra = ();

    fn run(&self, state: &MiddlewareState) -> (Vec2, Self::Extra) {
        (Vec2::new(self.0, self.0), ())
    }
}

pub fn offset(amount: f64) -> impl Middleware<Extra = ()> {
    Offset(amount)
}

pub struct Arrow;

impl Middleware for Arrow {
    type Extra = Vec2;

    fn run(&self, state: &MiddlewareState) -> (Vec2, Self::Extra) {
        (Vec2::new(1.0, 2.0), Vec2::new(10000.0, 10000000.0))
    }
}

// pub fn arrow() -> impl Middleware<Extra = Vec2> {
//     |state: &MiddlewareState| {
//         *data += 1;
//         Vec2::new(1.0, 1.0)
//     }
// }
