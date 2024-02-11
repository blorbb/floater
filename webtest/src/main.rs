#![feature(try_blocks)]

use floater::{
    compute_position,
    geometry::{ElemRect, ElemSize, Side},
    modifiers::{arrow, arrow::ArrowData, flip, offset, resize, shift, shift::limiter},
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
        Single;
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
        div.scrolling {
            div.padding-elem;
            Dropdown;
            div.padding-elem;
        }
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
                    .add_modifier(
                        do_flip.then_some(
                            &mut flip()
                                .padding_outward(20.0)
                                .padding_cross(5.0)
                                .flip_cross(true)
                                .check_cross_axis(true),
                        ),
                    )
                    .add_modifier(
                        &mut shift()
                            .padding_outward(20.0)
                            .padding_cross(5.0)
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

            arrow_data
                .generate_css_props(side, arrow_el.offset_width() as f64, "px")
                .into_iter()
                .for_each(|(k, v)| {
                    _ = arrow_el.clone().style(k, v);
                });
        };
    });

    let on_click = move |w| {
        let _: Option<_> = try {
            let floater = tooltip()?;
            let tip_styles = (*floater).style();
            tip_styles.set_property("width", &format!("{w}px")).ok()?;
            let height = floater.get_bounding_client_rect().height();
            logging::error!("{height}");
        };
    };

    mview! {
        p {
            button on:click={move |_| on_click(20)} {"set 20px"}
            button on:click={move |_| on_click(200)} {"set 200px"}
            button ref={reference} { "reference el" }
        }
        div.tooltip ref={tooltip} {
            div.tooltip-contents {
                "lorem ipsum dolor sit amet, some other random stuff that i don't remember"
            }
            div.arrow ref={arrow_el} { div.arrow-style; }
        }
    }
}

#[component]
fn Dropdown() -> impl IntoView {
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

            let container = reference
                .offset_parent()
                .and_then(|el| el.dyn_into::<HtmlElement>().ok())
                .map(|el| ElemRect::from_elem_visibility(el.as_ref()))
                .expect("where offset parent gone :(");

            let ref_rect = ElemRect::from_elem_offset(&reference);
            let min_height = 100.0;
            let tip_size = ElemSize::new(50.0, min_height);

            let mut arrow_data = ArrowData::new();

            let data = compute_position(
                ref_rect,
                tip_size,
                container,
                PositionOpts::new()
                    .with_side(Side::Bottom)
                    .add_modifier(
                        &mut resize(|available, state| {
                            logging::warn!("{state:?}");
                            let style = (*floater).style();
                            let width = state.reference.width();
                            _ = style.set_property("width", &format!("{width}px"));
                            let height = available.height().max(min_height);
                            _ = style.set_property("height", &format!("{height}px"));
                            ElemSize::new(state.reference.width(), height)
                        })
                        .padding_outward(20.0)
                        .padding_cross(5.0)
                        .padding_inward(15.0),
                    )
                    .add_modifier(&mut flip().padding_outward(20.0).padding_cross(5.0))
                    .add_modifier(
                        &mut shift()
                            .padding_outward(20.0)
                            .padding_cross(5.0)
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

            arrow_data
                .generate_css_props(side, arrow_el.offset_width() as f64, "px")
                .into_iter()
                .for_each(|(k, v)| {
                    _ = arrow_el.clone().style(k, v);
                });
        };
    });

    mview! {
        p {
            button ref={reference} { "referenceawjroaiwkrjaowihkr" }
        }
        div.tooltip ref={tooltip} {
            div.tooltip-contents {
                ul {
                    li { "a" }
                    li { "b" }
                    li { "c" }
                    li { "d" }
                    li { "e" }
                    li { "f" }
                }
            }
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
                    .add_modifier(&mut flip().padding_outward(20.0).padding_cross(5.0))
                    .add_modifier(
                        &mut shift()
                            .padding_outward(20.0)
                            .padding_cross(5.0)
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

            arrow_data
                .generate_css_props(side, arrow_el.offset_width() as f64, "px")
                .into_iter()
                .for_each(|(k, v)| {
                    _ = arrow_el.clone().style(k, v);
                });
        };
    });

    mview! {
        p {
            button ref={reference} { "reference el" }
        }
        div.tooltip ref={tooltip} {
            div.tooltip-contents {
                "what" br; "aaaaa" br; br; "content"
            }
            div.diamond-arrow ref={arrow_el} { div.diamond ref={diamond}; }
        }
    }
}
