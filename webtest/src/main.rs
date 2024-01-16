#![feature(try_blocks)]

use floater::{
    compute_position,
    geometry::{ElemRect, ElemSize, Side},
    modifiers::{arrow, arrow::ArrowData, flip, offset, shift, shift::limiter, Padding},
    PositionOpts,
};
use leptos::*;
use leptos_mview::mview;
use leptos_use::use_event_listener;
use web_sys::{wasm_bindgen::JsCast, HtmlElement};

fn main() { mount_to_body(App) }

#[component]
fn App() -> impl IntoView {
    console_error_panic_hook::set_once();
    mview! {
        // Single;
        div.scrolling {
            div.padding-elem;
            Single;
            div.padding-elem {"aaa"}
        }
        div.padding-elem { "random stuff" }
        Diamond s={Side::Top};
        Diamond s={Side::Left};
        Diamond s={Side::Right};
        Diamond s={Side::Bottom};
        div.padding-elem; div.padding-elem;
    }
}

#[component]
fn Single() -> impl IntoView {
    let reference = NodeRef::<html::Button>::new();
    let tooltip = NodeRef::<html::Div>::new();
    let arrow_el = NodeRef::<html::Div>::new();

    let refresh = RwSignal::new(());

    create_effect(move |_| {
        logging::log!("loaded");
        let _: Option<_> = try {
            _ = use_event_listener(reference.get()?.offset_parent()?, ev::scroll, move |_| {
                refresh.set(())
            });
            logging::log!("scroll")
        };
    });

    window_event_listener(ev::scroll, move |_| refresh.set(()));

    create_effect(move |_| {
        refresh.track();

        let _: Option<_> = try {
            let reference = reference.get()?;
            let floater = tooltip.get()?;
            let arrow_el = arrow_el.get()?;

            // let viewport = document().document_element()?;
            // let scroll = document().scrolling_element()?;
            // let client_rect = ElemRect::new(
            //     scroll.scroll_left() as f64,
            //     scroll.scroll_top() as f64,
            //     viewport.client_width() as f64,
            //     viewport.client_height() as f64,
            // );
            // logging::log!("CLIENT == {client_rect:?}");
            let container = reference
                .offset_parent()
                .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                .map(|el| ElemRect::from_elem_visibility(el.as_ref()))
                .expect("where offset parent gone :(");
            // let container = container.intersect(&client_rect);

            let ref_rect = ElemRect::from_elem_offset(&reference);
            let tip_size = ElemSize::new(
                floater.offset_width() as f64,
                floater.offset_height() as f64,
            );
            logging::log!("ref == {ref_rect:?}");
            logging::log!("flt == {tip_size:?}");
            logging::log!("con == {container:?}");

            let do_flip = true;
            let mut arrow_data = ArrowData::new();

            let data = compute_position(
                ref_rect,
                tip_size,
                container,
                PositionOpts::new()
                    .with_side(Side::Top)
                    .add_modifier(do_flip.then_some(&mut flip().padding(Padding {
                        outward: 20.0,
                        sideways: 5.0,
                    })))
                    .add_modifier(
                        &mut shift()
                            .padding(Padding {
                                outward: 20.0,
                                sideways: 5.0,
                            })
                            // should be arrow size + sideways padding (+ arrow padding)
                            .limiter(limiter::attached(20.0)),
                    )
                    .add_modifier(&mut offset(15.0))
                    .add_modifier(
                        &mut arrow(arrow_el.offset_width() as f64, &mut arrow_data).padding(5.0),
                    ),
            );
            let (x, y) = data.rect.xy();
            let side = data.side;
            logging::log!("{x}, {y}");

            let tip_styles = (*floater).style();
            tip_styles.set_property("top", &format!("{y}px")).ok()?;
            tip_styles.set_property("left", &format!("{x}px")).ok()?;

            let arrow_styles = (*arrow_el).style();
            arrow_styles.set_css_text(&arrow_data.generate_css_text(
                side,
                arrow_el.offset_width() as f64,
                "px",
            ))
        };
    });

    mview! {
        p {
            button ref={reference} { "reference el" }
        }
        div.tooltip ref={tooltip} {
            "what" br; "aaaaa" br; br; "content"
            div.arrow ref={arrow_el} { div.arrow-style; }
        }
    }
}

#[component]
fn Diamond(s: Side) -> impl IntoView {
    let reference = NodeRef::<html::Button>::new();
    let tooltip = NodeRef::<html::Div>::new();
    let arrow_el = NodeRef::<html::Div>::new();
    let diamond = NodeRef::<html::Div>::new();

    let refresh = RwSignal::new(());

    create_effect(move |_| {
        logging::log!("loaded");
        let _: Option<_> = try {
            _ = use_event_listener(reference.get()?.offset_parent()?, ev::scroll, move |_| {
                refresh.set(())
            });
            logging::log!("scroll")
        };
    });

    window_event_listener(ev::scroll, move |_| refresh.set(()));

    create_effect(move |_| {
        refresh.track();

        let _: Option<_> = try {
            let reference = reference.get()?;
            let floater = tooltip.get()?;
            let arrow_el = arrow_el.get()?;

            let container = reference
                .offset_parent()
                .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                .map(|el| ElemRect::from_elem_visibility(el.as_ref()))
                .expect("where offset parent gone :(");
            let arrow_size = arrow_el.get_bounding_client_rect();

            let ref_rect = ElemRect::from_elem_offset(&reference);
            let tip_size = ElemSize::new(
                floater.offset_width() as f64,
                floater.offset_height() as f64,
            );

            let mut arrow_data = ArrowData::new();

            let data = compute_position(
                ref_rect,
                tip_size,
                container,
                PositionOpts::new()
                    .with_side(s)
                    .add_modifier(&mut flip().padding(Padding {
                        outward: 20.0,
                        sideways: 5.0,
                    }))
                    .add_modifier(
                        &mut shift()
                            .padding(Padding {
                                outward: 20.0,
                                sideways: 5.0,
                            })
                            .limiter(limiter::attached(20.0)),
                    )
                    .add_modifier(&mut offset(15.0))
                    .add_modifier(&mut arrow(arrow_size.width(), &mut arrow_data).padding(5.0)),
            );
            let (x, y) = data.rect.xy();
            let side = data.side;
            logging::log!("{x}, {y}");

            let tip_styles = (*floater).style();
            tip_styles.set_property("top", &format!("{y}px")).ok()?;
            tip_styles.set_property("left", &format!("{x}px")).ok()?;

            let arrow_styles = (*arrow_el).style();
            arrow_styles.set_css_text(&arrow_data.generate_css_text(side, arrow_size.width(), "px"))
        };
    });

    mview! {
        p {
            button ref={reference} { "reference el" }
        }
        div.tooltip ref={tooltip} {
            "what" br; "aaaaa" br; br; "content"
            div.diamond-arrow ref={arrow_el} { div.diamond ref={diamond}; }
        }
    }
}
