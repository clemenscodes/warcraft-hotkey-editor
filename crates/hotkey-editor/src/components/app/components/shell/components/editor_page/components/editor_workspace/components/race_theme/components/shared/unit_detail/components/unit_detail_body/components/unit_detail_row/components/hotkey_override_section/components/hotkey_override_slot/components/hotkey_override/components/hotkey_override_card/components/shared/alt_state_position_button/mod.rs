mod model;
mod view;

pub use view::AltStatePositionButtonView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::AltStatePositionButtonModel;

#[component]
pub fn AltStatePositionButton(props: AltStatePositionButtonModel) -> Element {
    let title = props.title;
    let aria_label = props.aria_label;
    let on_click = props.on_click;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            title,
            aria_label,
            onclick: move |_| on_click.call(()),
            svg {
                view_box: "0 0 24 24",
                xmlns: "http://www.w3.org/2000/svg",
                circle {
                    cx: "12",
                    cy: "12",
                    r: "5",
                    fill: "none",
                    stroke: "currentColor",
                    stroke_width: "1.6",
                }
                line {
                    x1: "12",
                    y1: "2.5",
                    x2: "12",
                    y2: "6",
                    stroke: "currentColor",
                    stroke_width: "1.6",
                    stroke_linecap: "round",
                }
                line {
                    x1: "12",
                    y1: "18",
                    x2: "12",
                    y2: "21.5",
                    stroke: "currentColor",
                    stroke_width: "1.6",
                    stroke_linecap: "round",
                }
                line {
                    x1: "2.5",
                    y1: "12",
                    x2: "6",
                    y2: "12",
                    stroke: "currentColor",
                    stroke_width: "1.6",
                    stroke_linecap: "round",
                }
                line {
                    x1: "18",
                    y1: "12",
                    x2: "21.5",
                    y2: "12",
                    stroke: "currentColor",
                    stroke_width: "1.6",
                    stroke_linecap: "round",
                }
                circle {
                    cx: "12",
                    cy: "12",
                    r: "1.4",
                    fill: "currentColor",
                }
            }
        }
    }
}

assert_component!(AltStatePositionButton);
