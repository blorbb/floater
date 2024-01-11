#![feature(try_blocks)]

use floater::{
    compute_position,
    geometry::{ElemRect, ElemSize},
    modifiers::offset,
    PositionOpts, Side,
};
use leptos::*;
use leptos_mview::mview;

fn main() { mount_to_body(App) }

#[component]
fn App() -> impl IntoView {
    let reference = NodeRef::<html::Button>::new();
    let tooltip = NodeRef::<html::Div>::new();

    create_effect(move |_| {
        let _: Option<_> = try {
            let ref_rect = reference.get()?.get_bounding_client_rect();
            let tip_rect = tooltip.get()?.get_bounding_client_rect();
            let viewport = document().document_element()?.get_bounding_client_rect();

            let ref_rect = ElemRect::from(ref_rect);
            let tip_size = ElemSize::new(tip_rect.width(), tip_rect.height());
            let viewport = ElemRect::from(viewport);
            logging::log!("{ref_rect:?}");
            logging::log!("{tip_size:?}");

            let (x, y) = compute_position(
                ref_rect,
                tip_size,
                viewport,
                PositionOpts::new()
                    .with_side(Side::Bottom)
                    .add_modifier(&mut offset(5.0)),
            )
            .xy();
            logging::log!("{x}, {y}");

            let tip_styles = tooltip.get().as_deref()?.style();
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
