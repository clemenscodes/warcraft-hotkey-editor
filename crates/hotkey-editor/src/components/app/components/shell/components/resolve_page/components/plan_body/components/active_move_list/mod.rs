pub mod components;
mod props;

use super::shared::move_list::MoveList;
use components::move_row::MoveRow;
use dioxus::prelude::*;
pub use props::ActiveMoveListProps;

/// The active category's move list: the grid of move cards for the selected
/// section. Renders nothing when the plan has no moves (only unresolved
/// abilities), so the plan body never has to branch on it.
use tw_macro::assert_component;
assert_component!(ActiveMoveList);
#[component]
pub fn ActiveMoveList(props: ActiveMoveListProps) -> Element {
    let Some(section) = props.section else {
        return rsx! {};
    };
    let data_category = section.data_category();
    let rows = section.rows().to_vec();
    rsx! {
        MoveList {
            data_category,
            for row in rows {
                MoveRow { ..row }
            }
        }
    }
}
