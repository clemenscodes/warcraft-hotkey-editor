pub mod components;
mod props;
mod style;

use components::fight_column::FightColumn;
use dioxus::prelude::*;
use props::FightRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// The stuck ability's row: it centers the ability's column.
#[component]
pub fn FightRow(props: FightRowProps) -> Element {
    let unresolved_view = props.unresolved_view;
    rsx! {
        div {
            class: CLASS,
            FightColumn { unresolved_view }
        }
    }
}

assert_component!(FightRow);
