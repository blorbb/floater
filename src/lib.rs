pub mod modifiers;
pub mod vec2;

use modifiers::{Modifier, ModifierState, Modifiers};
use vec2::Vec2;

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
    modifiers: Modifiers<'a>,
}

impl<'a> PositionOpts<'a> {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn side(mut self, side: Side) -> Self {
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

pub fn compute_position(reference: ElemRect, floater: ElemSize, opts: PositionOpts) -> vec2::Vec2 {
    let x = match opts.side {
        Side::Top | Side::Bottom => reference.center().x - floater.width / 2.0,
        Side::Left => reference.left() - floater.width,
        Side::Right => reference.right(),
    };

    let y = match opts.side {
        Side::Left | Side::Right => reference.center().y - floater.height / 2.0,
        Side::Top => reference.top() - floater.height,
        Side::Bottom => reference.bottom(),
    };

    let mut mid_state = ModifierState {
        side: opts.side,
        reference,
        floater,
        pos: Vec2::new(x, y),
    };

    for modifier in opts.modifiers {
        mid_state.pos = modifier.run(&mid_state);
    }

    mid_state.pos
}
