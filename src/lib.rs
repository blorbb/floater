pub mod middleware;
pub mod vec2;

use middleware::{Middleware, MiddlewareState, Middlewares};


/// A rectangle placed on a viewport (scrolling context).
///
/// Positive `x` goes right, positive `y` goes down. Width and height must be non-negative.
#[derive(Debug)]
pub struct ElemRect {
    point: vec2::Vec2,
    /// Must be greater than 0
    pub width: f64,
    /// Must be greater than 0
    pub height: f64,
}

impl ElemRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            point: vec2::Vec2::new(x, y),
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

    fn center(&self) -> vec2::Vec2 {
        vec2::Vec2::new(self.x() + self.width / 2.0, self.y() + self.height / 2.0)
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

    pub fn as_vec2(&self) -> vec2::Vec2 {
        vec2::Vec2::new(self.width, self.height)
    }
}

#[derive(Debug, Default)]
pub struct PositionOpts<'a> {
    side: Side,
    middleware: Middlewares<'a>,
}

impl<'a> PositionOpts<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn side(mut self, side: Side) -> Self {
        self.side = side;
        self
    }

    pub fn add_middleware(mut self, mw: &'a mut impl Middleware) -> Self {
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

pub fn compute_position(reference: ElemRect, tooltip: ElemSize, opts: PositionOpts) -> vec2::Vec2 {
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

    let mut mid_state = MiddlewareState {
        placement: opts.side,
        reference,
        tooltip,
        pos: vec2::Vec2::new(x, y),
    };

    for mw in opts.middleware {
        mid_state.pos = mw.run(&mid_state);
    }

    mid_state.pos
}
