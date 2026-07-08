mod props;
mod style;

use dioxus::prelude::*;
pub use props::UnitIdProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitId);

/// The unit's database id.
#[component]
pub fn UnitId(props: UnitIdProps) -> Element {
    let text = props.text;
    rsx! {
        code {
            class: CLASS,
            {text}
        }
    }
}
