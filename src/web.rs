use web_sys::{DomRect, Element, HtmlElement};

use crate::geometry::{ElemRect, ElemSize};

impl From<DomRect> for ElemRect {
    fn from(value: DomRect) -> Self {
        Self::new(value.x(), value.y(), value.width(), value.height())
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
    #[must_use]
    pub fn from_elem_offset(el: &HtmlElement) -> Self {
        Self::new(
            f64::from(el.offset_left()),
            f64::from(el.offset_top()),
            f64::from(el.offset_width()),
            f64::from(el.offset_height()),
        )
    }

    /// Creates an [`ElemRect`] that describes which part of the element is
    /// visible.
    ///
    /// This is intended to be used for the container element.
    ///
    /// The implementation uses the `scroll_{left,top}` for `x/y`, and
    /// `client_{width,height}` for `width/height`.
    #[must_use]
    pub fn from_elem_visibility(el: &HtmlElement) -> Self {
        Self::new(
            f64::from(el.scroll_left()),
            f64::from(el.scroll_top()),
            f64::from(el.client_width()),
            f64::from(el.client_height()),
        )
    }

    pub fn from_bounding_client_rect(el: &Element) -> Self {
        let rect = el.get_bounding_client_rect();
        Self::new(rect.x(), rect.y(), rect.width(), rect.height())
    }
}

impl ElemSize {
    pub fn from_bounding_client_rect(el: &Element) -> Self {
        let rect = el.get_bounding_client_rect();
        Self::new(rect.width(), rect.height())
    }
}
