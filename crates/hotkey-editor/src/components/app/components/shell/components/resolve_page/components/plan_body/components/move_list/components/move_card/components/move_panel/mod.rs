pub mod components;
mod model;
mod presentation;
mod view;

pub use view::MovePanelView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_badge::MoveReasonBadge;
use components::fight_row::FightRow;
use components::move_transition::MoveTransition;
use dioxus::prelude::*;
use presentation::MovePanelPresentation;
use model::MovePanelModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MovePanel(props: MovePanelModel) -> Element {
    let move_view = props.move_view;
    let model = MovePanelPresentation::from(&move_view);
    let MovePanelPresentation {
        reason_kind,
        reason_label,
        from_placements,
        to_placements,
    } = model;
    rsx! {
        div {
            class: CLASS,
            MoveReasonBadge {
                kind: reason_kind,
                label: reason_label,
            }
            FightRow {
                move_view,
            }
            MoveTransition {
                from_placements,
                to_placements,
            }
        }
    }
}

assert_component!(MovePanel);
