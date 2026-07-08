mod props;
mod style;

use crate::components::app::components::shell::components::shared::clear_icon::ClearIcon;
use crate::components::app::components::shell::components::shared::clear_label::ClearLabel;
use crate::components::app::components::shell::components::shared::page_state::PageState;
use dioxus::prelude::*;
pub use props::ClearStateProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ClearState);

/// The centered "all clear" state for a collision kind with no conflicts. A thin
/// identity wrapper that tags the collision kind for e2e and hands the shared
/// `PageState` shell its glyph and label.
#[component]
pub fn ClearState(props: ClearStateProps) -> Element {
    let collision_kind = props.collision_kind.kind_param();
    rsx! {
        section {
            class: CLASS,
            "data-collision-kind": collision_kind,
            "data-unit-count": "0",
            PageState {
                ClearIcon {}
                ClearLabel { text: "All clear." }
            }
        }
    }
}
