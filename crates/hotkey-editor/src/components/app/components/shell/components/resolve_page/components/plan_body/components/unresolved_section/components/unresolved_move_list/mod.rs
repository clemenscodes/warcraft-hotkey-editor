pub mod components;
mod props;
mod style;

use components::unresolved_row::UnresolvedRow;
use dioxus::prelude::*;
use props::UnresolvedMoveListProps;
use style::CLASS;
use tw_macro::assert_component;

/// The unresolved section's auto-fill grid: one stuck-ability card per unresolved row.
#[component]
pub fn UnresolvedMoveList(props: UnresolvedMoveListProps) -> Element {
    let unresolved = props.unresolved;
    rsx! {
        div {
            class: CLASS,
            for unresolved_view in unresolved {
                UnresolvedRow { unresolved_view }
            }
        }
    }
}

assert_component!(UnresolvedMoveList);
