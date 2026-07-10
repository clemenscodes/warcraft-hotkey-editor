mod props;
mod style;

use dioxus::prelude::*;
pub use props::UnitIdProps;
use style::CLASS;
use tw_macro::assert_component;

/// The unit's database id.
#[component]
pub fn UnitId(props: UnitIdProps) -> Element {
    rsx! {
        code {
            class: CLASS,
            {props.unit_id.value()}
        }
    }
}

assert_component!(UnitId);
