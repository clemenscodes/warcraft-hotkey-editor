mod model;
mod presentation;
mod view;

pub use view::KeyCapLabelView;
mod style;

use dioxus::prelude::*;
use model::KeyCapLabelModel;
use presentation::KeyCapGlyph;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn KeyCapLabel(props: KeyCapLabelModel) -> Element {
    let label = props.label;
    let glyph = KeyCapGlyph::from(label.as_str());
    match glyph {
        KeyCapGlyph::Icon { icon, name } => rsx! {
            span {
                class: CLASS,
                role: "img",
                aria_label: name,
                dangerous_inner_html: icon,
            }
        },
        KeyCapGlyph::Text => rsx! {
            span {
                class: CLASS,
                {label}
            }
        },
    }
}

assert_component!(KeyCapLabel);
