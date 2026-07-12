mod model;
mod view;

pub use view::AltStateLabelView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use model::AltStateLabelModel;

/// The caption naming an off-state or upgraded form; renders nothing when the form
/// has no distinct name.
#[component]
pub fn AltStateLabel(props: AltStateLabelModel) -> Element {
    let Some(text) = props.text else {
        return rsx! {};
    };
    rsx! {
        p { class: CLASS, {text} }
    }
}

assert_component!(AltStateLabel);
