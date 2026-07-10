pub mod components;
mod props;
mod view;

pub use view::MoveRowView;
mod style;

use components::move_panel::MovePanel;
use dioxus::prelude::*;
use props::MoveRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// One move card: the reason badge, the fighting abilities (names over icons), and
/// the from -> to grids drawing where each ability lands. It owns only the grid root
/// and hands the move down to the panel that lays the card out.
#[component]
pub fn MoveRow(props: MoveRowProps) -> Element {
    let move_view = props.move_view;
    rsx! {
        div {
            class: CLASS,
            MovePanel { move_view }
        }
    }
}

assert_component!(MoveRow);
