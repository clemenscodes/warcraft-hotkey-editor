pub mod components;
mod props;
mod style;

use components::anchor_column::AnchorColumn;
use components::fight_column::{FightColumn, FightColumnProps};
use dioxus::prelude::*;
pub use props::FightRowProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(FightRow);

/// The fighting-abilities row: the mover column beside the optional rival column (the
/// rival renders itself away when the move has no anchor).
#[component]
pub fn FightRow(props: FightRowProps) -> Element {
    let fight_column = FightColumnProps::from(&props);
    let anchor = props.anchor;
    rsx! {
        div {
            class: CLASS,
            FightColumn { ..fight_column }
            AnchorColumn { ..anchor }
        }
    }
}
