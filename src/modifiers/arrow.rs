use super::{Modifier, ModifierReturn, ModifierState};
use crate::geometry::{ElemSize, Side, Vec2};

/// The arrow element should be inside the floater element, where both floater
/// and arrow has `position: absolute`.
///
/// The `data` parameter is an out parameter, which provides extra information
/// about the arrow's positioning. This should be a mut reference to a variable
/// that stores the [`ArrowData`]. No information from this parameter will be
/// read.
///
/// You should also use the `side` information provided by
/// [`compute_position`](crate::compute_position) to rotate the arrow as needed.
pub fn arrow(size: ArrowSize, data: &mut ArrowData) -> Arrow {
    Arrow {
        arrow_size: size,
        data,
        padding: 0.0,
    }
}

#[doc(hidden)]
pub struct Arrow<'a> {
    arrow_size: ArrowSize,
    padding: f64,
    data: &'a mut ArrowData,
}

impl Arrow<'_> {
    /// How far the arrow must stay from the corners of the floater.
    #[must_use]
    pub const fn padding(mut self, padding: f64) -> Self {
        self.padding = padding;
        self
    }
}

impl<'a> Modifier for Arrow<'a> {
    fn run(
        &mut self,
        ModifierState {
            reference,
            floater,
            side,
            ..
        }: &ModifierState,
    ) -> ModifierReturn {
        // !! coordinates are working as if its positioning the *center* of the arrow

        let ideal_center = floater.size().dim_along(*side) / 2.0;

        let shifted_amount =
            floater.center().coord_along(*side) - reference.center().coord_along(*side);

        // saturate at 0 in case padding > tooltip size, avoids panic in the clamp
        let max_shift = (ideal_center - self.arrow_size.inline() / 2.0 - self.padding).max(0.0);
        let arrow_shift = shifted_amount.clamp(-max_shift, max_shift);

        let skid = ideal_center - arrow_shift;

        // !! coordinates are for the top-left arrow element position now

        let arrow_side = side.opposite();
        // position of the other coordinate. e.g. for Side::Top, `center` is the
        // x-coord, need a y-coord of the arrow height.
        let outset = match arrow_side {
            Side::Left | Side::Top => -self.arrow_size.block(),
            Side::Right => floater.width(),
            Side::Bottom => floater.height(),
        };

        // move from center to top-left
        *self.data.pos.coord_along_mut(*side) = skid - self.arrow_size.inline() / 2.0;
        *self.data.pos.coord_across_mut(*side) = outset;
        self.data.center_offset = (ideal_center - skid).abs();

        ModifierReturn::new()
    }
}

#[derive(Default)]
pub struct ArrowData {
    pos: Vec2,
    center_offset: f64,
}

impl ArrowData {
    /// Creates new [`ArrowData`] with dummy information stored. This should
    /// not be used before being properly populated with data passed in by the
    /// [`arrow`] out parameter.
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// The position of the arrow relative to the floater element. This should
    /// be used to set the `left` and `top` CSS properties.
    #[must_use]
    pub const fn pos(&self) -> Vec2 { self.pos }

    /// How far the arrow is relative to the ideal position (centered on the
    /// reference element). Will always be non-negative.
    #[must_use]
    pub const fn center_offset(&self) -> f64 { self.center_offset }
}

/// The size of the arrow element independent of orientation.
///
/// `inline` = length of the arrow parallel to the side it's on; i.e. the width
/// if placed on the top/bottom.
///
/// `block` = length of the arrow perpendicular to the side it's on; i.e. the
/// height if placed on the top/bottom.
pub struct ArrowSize(ElemSize);

impl ArrowSize {
    #[must_use]
    pub const fn new(inline: f64, block: f64) -> Self { Self(ElemSize::new(inline, block)) }

    #[must_use]
    pub const fn inline(&self) -> f64 { self.0.width() }
    #[must_use]
    pub const fn block(&self) -> f64 { self.0.height() }
}
