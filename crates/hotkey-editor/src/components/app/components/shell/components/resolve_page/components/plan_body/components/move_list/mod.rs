pub mod components;
mod props;
mod style;

use components::move_row::MoveRow;
use dioxus::prelude::*;
pub use props::MoveListProps;
use style::CLASS;
use tw_macro::assert_component;

/// The active category's move list: the grid of move cards for the selected section. It
/// owns the auto-fill grid directly. Renders nothing when the plan has no moves (only
/// unresolved abilities), so the plan body never has to branch on it.
#[component]
pub fn MoveList(props: MoveListProps) -> Element {
    let Some(section) = props.section else {
        return rsx! {};
    };
    let rows = section.rows().to_vec();
    rsx! {
        div {
            class: CLASS,
            for row in rows {
                MoveRow { ..row }
            }
        }
    }
}

assert_component!(MoveList);
