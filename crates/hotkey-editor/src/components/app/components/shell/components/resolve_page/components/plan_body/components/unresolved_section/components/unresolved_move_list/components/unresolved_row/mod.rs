pub mod components;
mod data;
mod model;
mod presentation;
mod view;

pub use view::UnresolvedRowView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_badge::MoveReasonBadge;
use crate::components::app::components::shell::components::resolve_page::presentation::ReasonKind;
use components::fight_row::FightRow;
use components::move_transition::MoveTransition;
use dioxus::prelude::*;
use model::UnresolvedRowModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnresolvedRow(props: UnresolvedRowModel) -> Element {
    let unresolved_view = props.unresolved_view;
    let placements = presentation::placements(&unresolved_view);
    rsx! {
        div {
            class: CLASS,
            MoveReasonBadge {
                kind: ReasonKind::Stuck,
                label: data::STUCK_LABEL,
            }
            FightRow {
                unresolved_view,
            }
            MoveTransition {
                placements,
            }
        }
    }
}

assert_component!(UnresolvedRow);
