pub mod components;
mod props;
mod style;

use crate::assert_component;
use components::active_move_list::{ActiveMoveList, ActiveMoveListProps};
use components::unresolved_section::{UnresolvedSection, UnresolvedSectionProps};
use dioxus::prelude::*;
pub use props::{PlanBodyProps, PlanBodySection};
use style::CLASS;
assert_component!(PlanBody);

/// The scrollable plan body: the active category's move cards, then the unresolved
/// abilities (when any). The active move list renders itself away when the plan has
/// no moves, so the body only guards the unresolved section on presence.
#[component]
pub fn PlanBody(props: PlanBodyProps) -> Element {
    let section = props.section;
    let unresolved_rows = props.unresolved_rows;
    let has_unresolved = !unresolved_rows.is_empty();
    let active = ActiveMoveListProps { section };
    let unresolved = UnresolvedSectionProps {
        rows: unresolved_rows,
    };
    rsx! {
        div {
            class: CLASS,
            ActiveMoveList { ..active }
            if has_unresolved {
                UnresolvedSection { ..unresolved }
            }
        }
    }
}
