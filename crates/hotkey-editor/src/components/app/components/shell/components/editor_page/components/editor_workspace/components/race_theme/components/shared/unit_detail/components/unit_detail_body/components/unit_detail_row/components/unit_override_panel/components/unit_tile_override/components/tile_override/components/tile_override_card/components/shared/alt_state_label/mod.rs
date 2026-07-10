mod props;
mod view;

pub use view::AltStateLabelView;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use props::AltStateLabelProps;

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

assert_component!(AltStateLabel);
