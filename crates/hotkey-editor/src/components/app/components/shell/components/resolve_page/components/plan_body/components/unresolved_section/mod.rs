pub mod components;
mod data;
mod props;
mod style;

use components::unresolved_row::UnresolvedRow;
use components::unresolved_title::UnresolvedTitle;
use dioxus::prelude::*;
pub use props::UnresolvedSectionProps;
use style::{CLASS, MOVE_LIST};
use tw_macro::assert_component;
assert_component!(UnresolvedSection);

/// The unresolved-abilities section under the move list. It owns its own move-list grid
/// directly and tags the unresolved category for e2e.
#[component]
pub fn UnresolvedSection(props: UnresolvedSectionProps) -> Element {
    let rows = props.rows;
    rsx! {
        div {
            class: CLASS,
            UnresolvedTitle { text: data::TITLE }
            div {
                class: MOVE_LIST,
                "data-category": data::CATEGORY_SLUG,
                for row in rows {
                    UnresolvedRow { ..row }
                }
            }
        }
    }
}
