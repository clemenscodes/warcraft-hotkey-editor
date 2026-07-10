mod props;
mod style;

use dioxus::prelude::*;
pub use props::RegenQualifierProps;
use style::CLASS;
use tw_macro::assert_component;

/// The italic regen qualifier parked before the regen gain.
#[component]
pub fn RegenQualifier(props: RegenQualifierProps) -> Element {
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
