pub mod components;
mod props;
mod view;

pub use view::MoveListView;
mod style;

use components::move_row::MoveRow;
use dioxus::prelude::*;
use props::MoveListProps;
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
    let moves = section.moves().to_vec();
    rsx! {
        div {
            class: CLASS,
            for move_view in moves {
                MoveRow { move_view }
            }
        }
    }
}

assert_component!(MoveList);
