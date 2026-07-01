pub mod components;
mod props;
mod style;

use super::resolve_move_list::ResolveMoveList;
use super::resolve_unresolved_row::ResolveUnresolvedRow;
use crate::assert_component;
use components::resolve_unresolved_title::ResolveUnresolvedTitle;
use dioxus::prelude::*;
pub use props::ResolveUnresolvedSectionProps;
use style::CLASS;
assert_component!(ResolveUnresolvedSection);

/// The unresolved-abilities section under the move list.
#[component]
pub fn ResolveUnresolvedSection(props: ResolveUnresolvedSectionProps) -> Element {
    let rows = props.rows;
    rsx! {
        div {
            class: CLASS,
            ResolveUnresolvedTitle {}
            ResolveMoveList {
                data_category: "unresolved",
                for row in rows {
                    ResolveUnresolvedRow { ..row }
                }
            }
        }
    }
}
