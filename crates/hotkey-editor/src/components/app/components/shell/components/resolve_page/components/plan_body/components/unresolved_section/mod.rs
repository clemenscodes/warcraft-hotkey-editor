pub mod components;
mod data;
mod props;
mod style;

use super::move_list::MoveList;
use crate::assert_component;
use components::unresolved_row::UnresolvedRow;
use components::unresolved_title::UnresolvedTitle;
use dioxus::prelude::*;
pub use props::UnresolvedSectionProps;
use style::CLASS;
assert_component!(UnresolvedSection);

/// The unresolved-abilities section under the move list.
#[component]
pub fn UnresolvedSection(props: UnresolvedSectionProps) -> Element {
    let rows = props.rows;
    rsx! {
        div {
            class: CLASS,
            UnresolvedTitle { text: data::TITLE }
            MoveList {
                data_category: data::CATEGORY_SLUG,
                for row in rows {
                    UnresolvedRow { ..row }
                }
            }
        }
    }
}
