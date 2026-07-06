pub mod components;
mod props;
mod style;

use components::clear_icon::ClearIcon;
use components::clear_label::ClearLabel;
use dioxus::prelude::*;
pub use props::ClearStateProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ClearState);

/// The centered "all clear" state for a collision kind with no conflicts.
#[component]
pub fn ClearState(props: ClearStateProps) -> Element {
    let collision_kind = props.collision_kind;
    rsx! {
        section {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "0",
            ClearIcon {}
            ClearLabel {}
        }
    }
}
