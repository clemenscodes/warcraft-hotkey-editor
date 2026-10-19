pub mod components;
mod model;
mod view;

pub use view::FightRowView;
mod style;

use components::anchor_column::AnchorColumn;
use components::fight_column::FightColumn;
use dioxus::prelude::*;
use model::FightRowModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FightRow(props: FightRowModel) -> Element {
    let move_view = props.move_view;
    let mover_view = move_view.clone();
    rsx! {
        div {
            class: CLASS,
            FightColumn {
                move_view: mover_view,
            }
            AnchorColumn {
                move_view,
            }
        }
    }
}

assert_component!(FightRow);
