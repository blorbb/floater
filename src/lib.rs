pub mod middleware;

use std::{any::Any, ops};

use middleware::{Middleware, MiddlewareData, MiddlewareState, Middlewares};

#[derive(Debug)]
pub struct Vec2 {
    pub x: f64,
    pub y: f64,
}

impl Vec2 {
    pub fn new(x: f64, y: f64) -> Self {
        Self { x, y }
    }
}

impl ops::Neg for Vec2 {
    type Output = Self;

    fn neg(self) -> Self::Output {
        Self::new(-self.x, -self.y)
    }
}

impl ops::Add for Vec2 {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Self::new(self.x + rhs.x, self.y + rhs.y)
    }
}

impl ops::Sub for Vec2 {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self::Output {
        self + -rhs
    }
}

impl ops::Mul<f64> for Vec2 {
    type Output = Self;

    fn mul(self, rhs: f64) -> Self::Output {
        Self::new(self.x * rhs, self.y * rhs)
    }
}

impl ops::Div<f64> for Vec2 {
    type Output = Vec2;

    fn div(self, rhs: f64) -> Self::Output {
        self * (1.0 / rhs)
    }
}

/// A rectangle placed on a viewport (scrolling context).
///
/// Positive `x` goes right, positive `y` goes down. Width and height must be non-negative.
#[derive(Debug)]
pub struct ElemRect {
    point: Vec2,
    /// Must be greater than 0
    pub width: f64,
    /// Must be greater than 0
    pub height: f64,
}

impl ElemRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            point: Vec2::new(x, y),
            width,
            height,
        }
    }

    fn x(&self) -> f64 {
        self.point.x
    }

    fn y(&self) -> f64 {
        self.point.y
    }

    fn center(&self) -> Vec2 {
        Vec2::new(self.x() + self.width / 2.0, self.y() + self.height / 2.0)
    }

    // fn top_left(&self) -> Vec2 {
    //     Vec2::new(self.left(), self.top())
    // }

    // fn top_right(&self) -> Vec2 {
    //     Vec2::new(self.right(), self.top())
    // }

    // fn bottom_left(&self) -> Vec2 {
    //     Vec2::new(self.left(), self.bottom())
    // }

    // fn bottom_right(&self) -> Vec2 {
    //     Vec2::new(self.right(), self.bottom())
    // }

    fn left(&self) -> f64 {
        self.x()
    }
    fn right(&self) -> f64 {
        self.x() + self.width
    }
    fn top(&self) -> f64 {
        self.y()
    }
    fn bottom(&self) -> f64 {
        self.y() + self.height
    }
}

#[derive(Debug)]
pub struct ElemSize {
    pub width: f64,
    pub height: f64,
}

impl ElemSize {
    pub fn new(width: f64, height: f64) -> Self {
        Self { width, height }
    }

    pub fn as_vec2(&self) -> Vec2 {
        Vec2::new(self.width, self.height)
    }
}

#[derive(Debug, Default)]
pub struct PositionOpts {
    side: Side,
    middleware: Middlewares,
}

impl PositionOpts {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn add_middleware(mut self, mw: impl Middleware<Extra = Box<dyn Any>>) -> Self {
        self.middleware.add(mw);
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

pub fn compute_position(
    reference: ElemRect,
    tooltip: ElemSize,
    opts: PositionOpts,
) -> (Vec2, MiddlewareData) {
    let x = match opts.side {
        Side::Top | Side::Bottom => reference.center().x - tooltip.width / 2.0,
        Side::Left => reference.left() - tooltip.width,
        Side::Right => reference.right(),
    };

    let y = match opts.side {
        Side::Left | Side::Right => reference.center().y - tooltip.height / 2.0,
        Side::Top => reference.top() - tooltip.height,
        Side::Bottom => reference.bottom(),
    };

    let mut mw_state = MiddlewareState {
        placement: opts.side,
        reference,
        tooltip,
        pos: Vec2::new(x, y),
    };
    let mut mw_data = MiddlewareData::new();

    for mw in opts.middleware {
        let (pos, data) = mw.run(&mw_state);
        mw_state.pos = pos;
        mw_data.push(data);
    }

    (mw_state.pos, mw_data)
}
