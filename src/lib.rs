pub mod modifiers;
pub mod vec2;

use std::mem;

use modifiers::{Modifier, ModifierState, Modifiers};
use vec2::Vec2;

/// A rectangle placed on a viewport (scrolling context).
///
/// Positive `x` goes right, positive `y` goes down. Width and height must be non-negative.
#[derive(Debug, Default)]
pub struct ElemRect {
    point: Vec2,
    size: ElemSize,
}

impl ElemRect {
    pub fn new(x: f64, y: f64, width: f64, height: f64) -> Self {
        Self {
            point: Vec2::new(x, y),
            size: ElemSize::new(width, height),
        }
    }

    pub fn x(&self) -> f64 {
        self.point.x
    }

    pub fn y(&self) -> f64 {
        self.point.y
    }

    pub fn width(&self) -> f64 {
        self.size.width()
    }

    pub fn height(&self) -> f64 {
        self.size.height()
    }

    pub fn xy(&self) -> (f64, f64) {
        (self.x(), self.y())
    }

    pub fn point(&self) -> Vec2 {
        self.point
    }

    pub fn center(&self) -> Vec2 {
        Vec2::new(
            self.x() + self.width() / 2.0,
            self.y() + self.height() / 2.0,
        )
    }

    pub fn set_point(&mut self, point: Vec2) {
        self.point = point;
    }

    pub fn set_size(&mut self, size: ElemSize) {
        self.size = size;
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
        self.x() + self.width()
    }
    fn top(&self) -> f64 {
        self.y()
    }
    fn bottom(&self) -> f64 {
        self.y() + self.height()
    }
}

#[derive(Debug, Default)]
pub struct ElemSize(Vec2);
impl ElemSize {
    pub fn new(width: f64, height: f64) -> Self {
        Self(Vec2::new(width, height))
    }

    pub fn width(&self) -> f64 {
        self.0.x
    }

    pub fn height(&self) -> f64 {
        self.0.y
    }

    pub fn as_vec2(&self) -> &Vec2 {
        &self.0
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
