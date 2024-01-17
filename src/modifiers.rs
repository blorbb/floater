pub mod offset;
pub use offset::offset;
pub mod flip;
pub use flip::flip;
pub mod shift;
pub use shift::shift;
pub mod arrow;
pub use arrow::arrow;

// nesting all the modifier stuff into a module so that `modifiers::*` don't
// have access to private fields
#[rustfmt::skip] pub use nest::*;
mod nest {
    use core::fmt;
    use std::vec;

    use crate::{
        geometry::{ElemRect, ElemSize, Vec2},
        Side,
    };

    /// Allows each modifier to read position data.
    ///
    /// This struct is marked `#[non_exhaustive]` to allow more state
    /// information to be passed in later. All fields will always be `pub`, so
    /// that you can unpack each field. A `..` pattern must always be included
    /// to ignore unused fields / due to `#[non_exhaustive]`.
    #[derive(Debug, Clone, Copy)]
    #[non_exhaustive]
    pub struct ModifierState {
        pub reference: ElemRect,
        // after initial placement, floater has a position too
        pub floater: ElemRect,
        pub container: ElemRect,
        pub side: Side,
    }

    impl ModifierState {
        #[must_use]
        pub const fn new(
            reference: ElemRect,
            floater: ElemRect,
            container: ElemRect,
            side: Side,
        ) -> Self {
            Self {
                reference,
                floater,
                container,
                side,
            }
        }

        pub fn update_with(&mut self, res: &ModifierReturn) {
            if let Some(point) = res.point {
                *self.floater.point_mut() = point;
            }
            if let Some(size) = res.size {
                *self.floater.size_mut() = size;
            }
            if let Some(side) = res.side {
                self.side = side;
            }
        }
    }

    #[derive(Default)]
    pub struct ModifierReturn {
        point: Option<Vec2>,
        size: Option<ElemSize>,
        side: Option<Side>,
    }

    impl ModifierReturn {
        #[must_use]
        pub const fn new() -> Self {
            Self {
                point: None,
                size: None,
                side: None,
            }
        }

        #[must_use]
        pub const fn point(mut self, point: Vec2) -> Self {
            self.point = Some(point);
            self
        }

        #[must_use]
        #[rustfmt::skip]
        pub const fn point_xy(self, x: f64, y: f64) -> Self {
            self.point(Vec2::new(x, y))
        }

        #[must_use]
        pub const fn size(mut self, size: ElemSize) -> Self {
            self.size = Some(size);
            self
        }

        #[must_use]
        pub const fn size_wh(self, width: f64, height: f64) -> Self {
            self.size(ElemSize::new(width, height))
        }

        #[must_use]
        pub const fn side(mut self, side: Side) -> Self {
            self.side = Some(side);
            self
        }

        #[must_use]
        pub const fn floater(mut self, rect: ElemRect) -> Self {
            self = self.point(rect.point());
            self = self.size(rect.size());
            self
        }
    }

    pub trait Modifier {
        fn run(&mut self, state: &ModifierState) -> ModifierReturn;
    }

    impl<F> Modifier for F
    where
        F: FnMut(&ModifierState) -> ModifierReturn,
    {
        fn run(&mut self, state: &ModifierState) -> ModifierReturn { self(state) }
    }

    pub struct Modifiers<'a>(Vec<&'a mut dyn Modifier>);

    impl<'a> Modifiers<'a> {
        #[must_use]
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
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            f.debug_tuple("Modifiers").finish()
        }
    }

    // TODO: need inward as well probably
    #[derive(Default)]
    pub struct Padding {
        pub outward: f64,
        pub inward: f64,
        pub cross: f64,
    }

    impl Padding {
        #[must_use]
        pub const fn splat(value: f64) -> Self {
            Self {
                outward: value,
                inward: value,
                cross: value,
            }
        }
    }

    #[macro_export]
    macro_rules! impl_padding_builder {
        ($path:ident) => {
            #[must_use]
            pub const fn padding(mut self, padding: f64) -> Self {
                self.$path = $crate::modifiers::Padding::splat(padding);
                self
            }

            #[must_use]
            pub const fn padding_inward(mut self, padding: f64) -> Self {
                self.$path.inward = padding;
                self
            }

            #[must_use]
            pub const fn padding_outward(mut self, padding: f64) -> Self {
                self.$path.outward = padding;
                self
            }

            #[must_use]
            pub const fn padding_cross(mut self, padding: f64) -> Self {
                self.$path.cross = padding;
                self
            }

            #[must_use]
            pub const fn padding_main(self, padding: f64) -> Self {
                self.padding_outward(padding).padding_inward(padding)
            }
        };
    }
}
