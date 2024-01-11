use web_sys::{DomRect, HtmlElement};

use crate::geometry::ElemRect;

impl From<DomRect> for ElemRect {
    fn from(value: DomRect) -> Self {
        ElemRect::new(value.x(), value.y(), value.width(), value.height())
    }
}

impl ElemRect {
    pub fn from_elem_offset(el: &HtmlElement) -> Self {
        Self::new(
            el.offset_left() as f64,
            el.offset_top() as f64,
            el.offset_width() as f64,
            el.offset_height() as f64,
        )
    }
}
