use super::{Modifier, ModifierReturn, ModifierState};
use crate::geometry::{side::Orientation, Side};

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
/// For the web, you can also use the [`ArrowData::generate_css_text`] method
/// (requires `web-utils` feature). Extra conditions about the arrow's styles
/// are required to use it.
pub fn arrow(inline_len: f64, data: &mut ArrowData) -> Arrow {
    Arrow {
        inline_len,
        data,
        padding: 0.0,
    }
}

#[doc(hidden)]
pub struct Arrow<'a> {
    inline_len: f64,
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
        let max_shift = (ideal_center - self.inline_len / 2.0 - self.padding).max(0.0);
        let arrow_shift = shifted_amount.clamp(-max_shift, max_shift);

        let skid = ideal_center - arrow_shift;

        // !! coordinates are for the top-left arrow element position now

        *self.data = ArrowData {
            // move from center to top-left
            offset: skid - self.inline_len / 2.0,
            center_offset: (ideal_center - skid).abs(),
        };

        ModifierReturn::new()
    }
}

#[derive(Default)]
pub struct ArrowData {
    offset: f64,
    center_offset: f64,
}

impl ArrowData {
    /// Creates new [`ArrowData`] with dummy information stored. This should
    /// not be used before being properly populated with data passed in by the
    /// [`arrow`] out parameter.
    #[must_use]
    pub fn new() -> Self { Self::default() }

    /// The offset of the arrow relative to the top-left of the floater element.
    /// This should be used with the floater's side to set the `left` or `top`
    /// CSS property.
    ///
    /// Alternatively, use the helper method [`Self::generate_css_properties`]
    /// to calculate the correct properties to set.
    #[must_use]
    pub const fn offset(&self) -> f64 { self.offset }

    /// How far the arrow is relative to the ideal position (centered on the
    /// reference element). Will always be non-negative.
    #[must_use]
    pub const fn center_offset(&self) -> f64 { self.center_offset }

    /// Generates CSS properties to set on the arrow element.
    ///
    /// The arrow element **must be square** with the arrow centered, on the
    /// **top edge, pointing down**. This can *usually* be achieved with the
    /// following CSS:
    ///
    /// ```text
    /// /* the element to put these styles on */
    /// .arrow-pos {
    ///     position: absolute;
    ///     aspect-ratio: 1;
    /// }
    ///
    /// /* nested inside .arrow-pos, containing the actual arrow */
    /// .arrow-inner {
    ///     position: relative;
    ///     /* width, height, ... */
    /// }
    /// ```
    ///
    /// The generated styles should then be set on the `.arrow-pos` element
    /// above, using [`web_sys::CssStyleDeclaration::set_css_text`].
    ///
    /// The extra information required is:
    /// - `floater_side`: which side of the reference the **floater** is on.
    ///   This is opposite to the direction the arrow will point, so that you
    ///   can directly pass in the side returned by
    ///   [`compute_position`](crate::compute_position).
    /// - `arrow_size`: side length of the arrow element. This should be the
    ///   same as the `inline_len` parameter passed in to the [`arrow`]
    ///   modifier.
    /// - `unit`: which units to use. This should be the same as the units used
    ///   to calculate the arrow's position - in most cases, "px".
    ///
    /// The properties returned are:
    /// - A `left` or `top` property to align the arrow along the side of the
    ///   floater.
    /// - Another inset property (top/left/bottom/right) to push the arrow to
    ///   the outside of the floater.
    /// - A `transform` property to rotate the arrow to point toward the
    ///   reference.
    #[allow(clippy::similar_names)]
    #[cfg(feature = "web-utils")]
    #[must_use]
    pub fn generate_css_text(&self, floater_side: Side, arrow_size: f64, unit: &str) -> String {
        let arrow_side = floater_side.opposite();
        let outset_property = arrow_side.as_css_prop();

        // this will never be the same as outset_property
        let offset_property = match arrow_side.orientation() {
            Orientation::Horizontal => "left",
            Orientation::Vertical => "top",
        };

        let rotation = match arrow_side {
            Side::Bottom => "0deg",
            Side::Top => "180deg",
            Side::Left => "90deg",
            Side::Right => "-90deg",
        };

        let offset = self.offset;

        format!(
            "\
            {outset_property}: -{arrow_size}{unit};\
            {offset_property}: {offset}{unit};\
            transform: rotate({rotation});\
            "
        )
    }
}
