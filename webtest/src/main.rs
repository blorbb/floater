#![feature(try_blocks)]

use floater::{compute_position, ElemRect, ElemSize, PositionOpts, Side, Vec2, middleware::offset};
use leptos::*;
use leptos_mview::mview;
use web_sys::{wasm_bindgen::JsCast, HtmlElement};

fn main() {
    mount_to_body(App)
}

#[component]
fn App() -> impl IntoView {
    let reference = NodeRef::<html::Button>::new();
    let tooltip = NodeRef::<html::Div>::new();

    create_effect(move |_| {
        let _: Option<_> = try {
            let ref_rect = reference.get()?.get_bounding_client_rect();
            let tip_rect = tooltip.get()?.get_bounding_client_rect();

            let ref_rect = ElemRect::new(
                ref_rect.x(),
                ref_rect.y(),
                ref_rect.width(),
                ref_rect.height(),
            );
            let tip_size = ElemSize::new(tip_rect.width(), tip_rect.height());
            logging::log!("{ref_rect:?}");
            logging::log!("{tip_size:?}");

            let mut arrow_data = 1;

            let (Vec2 { x, y }, data) = compute_position(
                ref_rect,
                tip_size,
                PositionOpts::new().side(Side::Top).add_middleware(offset(3.0)), // .add_middleware(arrow()),
            );
            logging::log!("{x}, {y}");

            let tip_styles = tooltip.get()?.dyn_ref::<HtmlElement>()?.style();
            tip_styles.set_property("top", &format!("{y}px")).ok()?;
            tip_styles.set_property("left", &format!("{x}px")).ok()?;
        };
    });

    mview! {
        p {
            button ref={reference} { "reference el" }
        }
        div.tooltip ref={tooltip} { "what" }
    }
}
