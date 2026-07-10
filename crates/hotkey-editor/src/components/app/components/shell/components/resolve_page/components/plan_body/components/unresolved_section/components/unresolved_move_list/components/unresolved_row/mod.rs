pub mod components;
mod data;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRow;
use crate::components::app::components::shell::components::resolve_page::logic::ReasonKind;
use components::fight_row::FightRow;
use components::move_transition::MoveTransition;
use dioxus::prelude::*;
use props::UnresolvedRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// One unresolved ability card: the Stuck badge over the stuck ability, over the cell it
/// is stuck on. It owns the orc-accented card surface directly and hands each region to
/// its own child component.
#[component]
pub fn UnresolvedRow(props: UnresolvedRowProps) -> Element {
    let unresolved_view = props.unresolved_view;
    let placements = logic::placements(&unresolved_view);
    rsx! {
        div {
            class: CLASS,
            MoveReasonRow {
                kind: ReasonKind::Stuck,
                label: data::STUCK_LABEL,
            }
            FightRow { unresolved_view }
            MoveTransition { placements }
        }
    }
}

assert_component!(UnresolvedRow);
