pub mod components;
mod props;
mod style;

use components::unresolved_row::UnresolvedRow;
use dioxus::prelude::*;
pub use props::UnresolvedMoveListProps;
use style::CLASS;
use tw_macro::assert_component;

/// The unresolved section's auto-fill grid: one stuck-ability card per unresolved row.
#[component]
pub fn UnresolvedMoveList(props: UnresolvedMoveListProps) -> Element {
    let rows = props.rows;
    rsx! {
        div {
            class: CLASS,
            for row in rows {
                UnresolvedRow { ..row }
            }
        }
    }
}

assert_component!(UnresolvedMoveList);
