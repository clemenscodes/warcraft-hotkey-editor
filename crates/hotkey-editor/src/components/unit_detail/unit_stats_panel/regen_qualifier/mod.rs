mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::RegenQualifierProps;
use style::CLASS;
assert_component!(RegenQualifier);

/// The italic regen qualifier parked before the regen gain.
#[component]
pub fn RegenQualifier(props: RegenQualifierProps) -> Element {
    let text = props.text;
    rsx! {
        span {
            class: CLASS,
            {text}
        }
    }
}
