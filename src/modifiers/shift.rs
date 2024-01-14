use self::limiter::{attached, Attached, ShiftLimiter};
use super::{Modifier, ModifierReturn, ModifierState, Padding};
use crate::{geometry::Side, space::space_around};

// TODO: option for shifting perpendicular to the side, use with the limiter
// so that it only shifts away from

#[must_use]
pub fn shift() -> Shift<Attached> {
    Shift {
        padding: 0.0.into(),
        limiter: attached(0.0),
    }
}

pub struct Shift<L> {
    padding: Padding,
    limiter: L,
}

impl<L> Shift<L> {
    #[must_use]
    pub fn padding(mut self, padding: impl Into<Padding>) -> Self {
        self.padding = padding.into();
        self
    }

    #[must_use]
    pub fn limiter<U: ShiftLimiter>(self, limiter: U) -> Shift<U> {
        Shift {
            limiter,
            padding: self.padding,
        }
    }
}

impl<L: ShiftLimiter> Modifier for Shift<L> {
    fn run(&mut self, state: &ModifierState) -> ModifierReturn {
        let ModifierState {
            floater,
            container,
            side,
            ..
        } = state;
        let space = space_around(floater, container);

        for side in side.adjacents() {
            let space_on_side = space.on_side(side);
            if space_on_side < self.padding.sideways {
                let mut new_point = floater.point();
                let shift_amount = space_on_side - self.padding.sideways;
                *new_point.coord_across_mut(side) += match side {
                    Side::Top | Side::Left => -shift_amount,
                    Side::Bottom | Side::Right => shift_amount,
                };

                // run limiter with the new state
                let mut curr_state = *state;
                *curr_state.floater.point_mut() = new_point;
                new_point = self.limiter.reshift(&curr_state);

                return ModifierReturn::new().point(new_point);
            }
        }

        ModifierReturn::new()
    }
}

pub mod limiter {
    use super::super::ModifierState;
    use crate::geometry::{side::Orientation, Vec2};

    pub trait ShiftLimiter {
        /// Should return a new position for where to place
        fn reshift(&mut self, state: &ModifierState) -> Vec2;
    }

    impl<F> ShiftLimiter for F
    where
        F: FnMut(&ModifierState) -> Vec2,
    {
        fn reshift(&mut self, state: &ModifierState) -> Vec2 { self(state) }
    }

    #[doc(hidden)]
    pub struct NoLimit;

    impl ShiftLimiter for NoLimit {
        #[rustfmt::skip]
        fn reshift(&mut self, state: &ModifierState) -> Vec2 {
            state.floater.point()
        }
    }

    #[must_use]
    pub const fn no_limit() -> NoLimit {
        // return itself
        NoLimit
    }

    #[doc(hidden)]
    pub struct Attached {
        padding: f64,
    }

    impl ShiftLimiter for Attached {
        fn reshift(
            &mut self,
            ModifierState {
                reference,
                floater,
                side,
                ..
            }: &ModifierState,
        ) -> Vec2 {
            let padding = self.padding;
            match side.orientation() {
                Orientation::Vertical => {
                    // limit y
                    let miny = reference.top() + padding - floater.height();
                    let maxy = reference.bottom() - padding;
                    let y = floater.y().clamp(miny, maxy);
                    Vec2::new(floater.x(), y)
                }
                Orientation::Horizontal => {
                    // limit x
                    let minx = reference.left() + padding - floater.width();
                    let maxx = reference.right() - padding;
                    let x = floater.x().clamp(minx, maxx);
                    Vec2::new(x, floater.y())
                }
            }
        }
    }

    #[must_use]
    pub const fn attached(padding: f64) -> Attached { Attached { padding } }
}
