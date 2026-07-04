mod props;
mod style;

use crate::assert_component;
use dioxus::prelude::*;
pub use props::UnitNameProps;
use style::CLASS;
assert_component!(UnitName);

/// The unit's name, in gold Friz Quadrata.
#[component]
pub fn UnitName(props: UnitNameProps) -> Element {
    let text = props.text;
    rsx! {
        h2 {
            class: CLASS,
            {text}
        }
    }
}
