pub mod components;
mod data;
mod props;
mod style;

use components::unresolved_move_list::{UnresolvedMoveList, UnresolvedMoveListProps};
use components::unresolved_title::UnresolvedTitle;
use dioxus::prelude::*;
pub use props::UnresolvedSectionProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnresolvedSection);

/// The unresolved-abilities section under the move list. It stacks the section title over
/// its own move-list grid of stuck-ability cards.
#[component]
pub fn UnresolvedSection(props: UnresolvedSectionProps) -> Element {
    let rows = props.rows;
    let move_list = UnresolvedMoveListProps { rows };
    rsx! {
        div {
            class: CLASS,
            UnresolvedTitle { text: data::TITLE }
            UnresolvedMoveList { ..move_list }
        }
    }
}
