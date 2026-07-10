mod props;
mod style;

use dioxus::prelude::*;

use style::CLASS;
use tw_macro::assert_component;

pub use props::UnitDescriptionProps;

assert_component!(UnitDescription);

/// The unit's flavor text under the detail header.
#[component]
pub fn UnitDescription(props: UnitDescriptionProps) -> Element {
    let text = props.text;
    rsx! {
        p { class: CLASS, {text} }
    }
}
