pub mod components;
mod hooks;
mod logic;
mod props;
mod style;

use components::move_panel::{MovePanel, MovePanelProps};
use dioxus::prelude::*;
use hooks::use_move_row;
pub use props::MoveRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// One move card: the reason badge, the fighting abilities (names over icons), and
/// the from -> to grids drawing where each ability lands. It owns only the grid root
/// and hands the shaped move to the panel that lays the card out. Its mover-name link
/// opens the unit through the navigation read from context.
#[component]
pub fn MoveRow(props: MoveRowProps) -> Element {
    let model = use_move_row(&props);
    let panel = MovePanelProps::from(model);
    rsx! {
        div {
            class: CLASS,
            MovePanel { ..panel }
        }
    }
}

assert_component!(MoveRow);
