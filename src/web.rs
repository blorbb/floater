use web_sys::DomRect;

use crate::geometry::ElemRect;

impl From<DomRect> for ElemRect {
    fn from(value: DomRect) -> Self {
        ElemRect::new(value.x(), value.y(), value.width(), value.height())
    }
}
