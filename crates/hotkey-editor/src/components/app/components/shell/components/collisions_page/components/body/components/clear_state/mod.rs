mod props;
mod style;

use crate::components::app::components::shell::components::shared::clear_icon::ClearIcon;
use crate::components::app::components::shell::components::shared::clear_label::ClearLabel;
use dioxus::prelude::*;
pub use props::ClearStateProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ClearState);

/// The centered "all clear" state for a collision kind with no conflicts. It centers
/// its glyph and label in the available space and tags the collision kind for e2e.
#[component]
pub fn ClearState(props: ClearStateProps) -> Element {
    let collision_kind = props.collision_kind.kind_param();
    rsx! {
        section {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "0",
            ClearIcon {}
            ClearLabel { text: "All clear." }
        }
    }
}
