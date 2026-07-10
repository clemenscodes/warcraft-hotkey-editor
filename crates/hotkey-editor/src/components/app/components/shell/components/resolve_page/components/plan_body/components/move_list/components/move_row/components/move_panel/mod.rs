pub mod components;
mod props;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRow;
use components::fight_row::{FightRow, FightRowProps};
use components::move_transition::{MoveTransition, MoveTransitionProps};
use dioxus::prelude::*;
pub use props::MovePanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The move card's surface (bordered, tinted, blue-accented): the reason badge over the
/// fighting-abilities row over the from → to transition block.
#[component]
pub fn MovePanel(props: MovePanelProps) -> Element {
    let fight_row = FightRowProps::from(&props);
    let move_transition = MoveTransitionProps::from(&props);
    let reason_row = props.reason_row;
    rsx! {
        div {
            class: CLASS,
            MoveReasonRow { ..reason_row }
            FightRow { ..fight_row }
            MoveTransition { ..move_transition }
        }
    }
}

assert_component!(MovePanel);
