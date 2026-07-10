pub mod components;
mod props;
mod view;

pub use view::FightRowView;
mod style;

use components::anchor_column::AnchorColumn;
use components::fight_column::FightColumn;
use dioxus::prelude::*;
use props::FightRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// The fighting-abilities row: the mover column beside the optional rival column (the
/// rival renders itself away when the move has no anchor).
#[component]
pub fn FightRow(props: FightRowProps) -> Element {
    let move_view = props.move_view;
    let mover_view = move_view.clone();
    rsx! {
        div {
            class: CLASS,
            FightColumn { move_view: mover_view }
            AnchorColumn { move_view }
        }
    }
}

assert_component!(FightRow);
