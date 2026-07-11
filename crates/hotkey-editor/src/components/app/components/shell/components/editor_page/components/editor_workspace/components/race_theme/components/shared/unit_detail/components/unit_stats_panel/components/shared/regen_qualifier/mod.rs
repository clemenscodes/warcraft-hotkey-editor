mod model;
mod view;

pub use view::RegenQualifierView;
mod style;

use dioxus::prelude::*;
use model::RegenQualifierModel;
use style::CLASS;
use tw_macro::assert_component;

/// The italic regen qualifier parked before the regen gain.
#[component]
pub fn RegenQualifier(props: RegenQualifierModel) -> Element {
    let Some(text) = props.text else {
        return rsx! {};
    };
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}

assert_component!(RegenQualifier);
