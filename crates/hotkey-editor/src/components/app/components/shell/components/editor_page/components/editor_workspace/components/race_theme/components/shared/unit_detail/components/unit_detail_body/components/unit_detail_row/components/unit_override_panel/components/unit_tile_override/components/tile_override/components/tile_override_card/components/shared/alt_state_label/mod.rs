mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::AltStateLabelProps;

assert_component!(AltStateLabel);

/// The caption naming an off-state or upgraded form; renders nothing when the form
/// has no distinct name.
#[component]
pub fn AltStateLabel(props: AltStateLabelProps) -> Element {
    let Some(text) = props.text else {
        return rsx! {};
    };
    rsx! {
        p { class: CLASS, {text} }
    }
}
