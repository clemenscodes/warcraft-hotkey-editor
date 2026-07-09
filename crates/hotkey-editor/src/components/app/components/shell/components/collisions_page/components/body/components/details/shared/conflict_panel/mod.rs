mod props;
mod style;

use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_card_caption::ConflictCardCaption;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_multi_stack::ConflictMultiStack;
use crate::components::app::components::shell::components::collisions_page::components::body::components::details::shared::conflict_pair_row::ConflictPairRow;
use dioxus::prelude::*;
pub use props::ConflictPanelProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictPanel);

/// The conflict card surface: the role caption over exactly one of the two clash
/// layouts (the pair row or the multi stack). Each layout renders itself away when it
/// does not apply. Shared by the hotkey and unit-position conflict cards.
#[component]
pub fn ConflictPanel(props: ConflictPanelProps) -> Element {
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
