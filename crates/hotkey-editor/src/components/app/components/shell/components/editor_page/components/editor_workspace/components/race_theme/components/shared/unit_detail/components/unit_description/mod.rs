mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

use props::UnitDescriptionProps;

/// The unit's flavor text under the detail header.
#[component]
pub fn UnitDescription(props: UnitDescriptionProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}

assert_component!(UnitDescription);
