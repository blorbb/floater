use web_sys::{DomRect, HtmlElement};

use crate::geometry::ElemRect;

impl From<DomRect> for ElemRect {
    fn from(value: DomRect) -> Self {
        ElemRect::new(value.x(), value.y(), value.width(), value.height())
    }
}

impl ElemRect {
    /// Creates an [`ElemRect`] from the provided element's `offset_*`
    /// positions.
    ///
    /// This is intended to be used for the reference element. If the reference
    /// is an inline-level element that could span multiple lines, you should
    /// probably select one of its rects with [`Element.getClientRects`](https://developer.mozilla.org/en-US/docs/Web/API/Element/getClientRects)
    ///
    /// See: [`offset_*` MDN documentation](https://developer.mozilla.org/en-US/docs/Web/API/HTMLElement/offsetLeft)
    pub fn from_elem_offset(el: &HtmlElement) -> Self {
        Self::new(
            el.offset_left() as f64,
            el.offset_top() as f64,
            el.offset_width() as f64,
            el.offset_height() as f64,
        )
    }

    /// Creates an [`ElemRect`] that describes which part of the element is
    /// visible.
    ///
    /// This is intended to be used for the container element.
    ///
    /// The implementation uses the `scroll_{left,top}` for `x/y`, and
    /// `client_{width,height}` for `width/height`.
    pub fn from_elem_visibility(el: &HtmlElement) -> Self {
        ElemRect::new(
            el.scroll_left() as f64,
            el.scroll_top() as f64,
            el.client_width() as f64,
            el.client_height() as f64,
        )
    }
}
