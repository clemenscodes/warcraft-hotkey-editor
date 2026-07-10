pub mod components;
mod props;
mod style;

use components::fight_column::{FightColumn, FightColumnProps};
use dioxus::prelude::*;
pub use props::FightRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(FightRow);

/// The stuck ability's row: it centers the ability's column.
#[component]
pub fn FightRow(props: FightRowProps) -> Element {
    let fight_column = FightColumnProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            FightColumn { ..fight_column }
        }
    }
}
