mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaption;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_multi_stack::ConflictMultiStack;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_pair_row::ConflictPairRow;
use dioxus::prelude::*;
pub use props::UnitPositionConflictPanelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UnitPositionConflictPanel);

/// The position-collision card surface: bordered, tinted, centered — the role-label
/// caption over the pair-row and multi-stack clash layouts. Each layout renders itself
/// away when it does not apply.
#[component]
pub fn UnitPositionConflictPanel(props: UnitPositionConflictPanelProps) -> Element {
    let caption = props.caption;
    let pair_row = props.pair_row;
    let multi_stack = props.multi_stack;
    rsx! {
        div {
            class: CLASS,
            ConflictCardCaption { ..caption }
            ConflictPairRow { ..pair_row }
            ConflictMultiStack { ..multi_stack }
        }
    }
}
