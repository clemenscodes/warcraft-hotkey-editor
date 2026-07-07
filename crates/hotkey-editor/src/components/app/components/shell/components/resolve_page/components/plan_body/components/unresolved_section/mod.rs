pub mod components;
mod data;
mod props;
mod style;

use super::shared::move_list::MoveList;
use components::unresolved_row::UnresolvedRow;
use components::unresolved_title::UnresolvedTitle;
use dioxus::prelude::*;
pub use props::UnresolvedSectionProps;
use style::CLASS;
use tw_macro::assert_component;
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
