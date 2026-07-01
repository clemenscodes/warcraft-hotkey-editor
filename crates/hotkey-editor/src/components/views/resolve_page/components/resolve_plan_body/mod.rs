mod props;
mod style;

use super::resolve_move_list::ResolveMoveList;
use super::resolve_move_row::ResolveMoveRow;
use super::resolve_unresolved_section::{ResolveUnresolvedSection, ResolveUnresolvedSectionProps};
use crate::assert_component;
use dioxus::prelude::*;
pub use props::{ResolvePlanBodyProps, ResolvePlanBodySection};
use style::CLASS;
assert_component!(ResolvePlanBody);

/// The scrollable plan body: the active category's move cards, then the unresolved
/// abilities (when any).
#[component]
pub fn ResolvePlanBody(props: ResolvePlanBodyProps) -> Element {
    let section = props.section;
    let unresolved_rows = props.unresolved_rows;
    let has_unresolved = !unresolved_rows.is_empty();
    let unresolved = ResolveUnresolvedSectionProps {
        rows: unresolved_rows,
    };
    rsx! {
        div {
            class: CLASS,
            if let Some(section) = section {
                ResolveMoveList {
                    data_category: section.data_category,
                    for row in section.rows {
                        ResolveMoveRow { ..row }
                    }
                }
            }
            if has_unresolved {
                ResolveUnresolvedSection { ..unresolved }
            }
        }
    }
}
