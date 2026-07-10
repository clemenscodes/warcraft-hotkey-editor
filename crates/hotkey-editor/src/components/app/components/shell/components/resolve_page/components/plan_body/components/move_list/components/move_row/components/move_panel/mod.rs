pub mod components;
mod logic;
mod props;
mod view;

pub use view::MovePanelView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRow;
use components::fight_row::FightRow;
use components::move_transition::MoveTransition;
use dioxus::prelude::*;
use logic::MovePanelModel;
use props::MovePanelProps;
use style::CLASS;
use tw_macro::assert_component;

/// The move card's surface (bordered, tinted, blue-accented): the reason badge over the
/// fighting-abilities row over the from → to transition block.
#[component]
pub fn MovePanel(props: MovePanelProps) -> Element {
    let move_view = props.move_view;
    let model = MovePanelModel::from(&move_view);
    let MovePanelModel {
        reason_kind,
        reason_label,
        from_placements,
        to_placements,
    } = model;
    rsx! {
        div {
            class: CLASS,
            MoveReasonRow { kind: reason_kind, label: reason_label }
            FightRow { move_view }
            MoveTransition { from_placements, to_placements }
        }
    }
}

assert_component!(MovePanel);
