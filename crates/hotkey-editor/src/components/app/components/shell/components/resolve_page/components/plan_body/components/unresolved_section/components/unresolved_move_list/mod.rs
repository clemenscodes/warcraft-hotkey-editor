pub mod components;
mod props;
mod style;

use components::unresolved_row::UnresolvedRow;
use dioxus::prelude::*;
pub use props::UnresolvedMoveListProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnresolvedMoveList);

/// The unresolved section's auto-fill grid: one stuck-ability card per unresolved row,
/// tagged with its category for the e2e suite.
#[component]
pub fn UnresolvedMoveList(props: UnresolvedMoveListProps) -> Element {
    let category = props.category;
    let rows = props.rows;
    rsx! {
        div {
            class: CLASS,
            "data-category": category,
            for row in rows {
                UnresolvedRow { ..row }
            }
        }
    }
}
