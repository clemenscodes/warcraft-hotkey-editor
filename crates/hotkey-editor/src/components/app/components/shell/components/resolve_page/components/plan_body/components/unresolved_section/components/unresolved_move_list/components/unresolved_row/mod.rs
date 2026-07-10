pub mod components;
mod data;
mod logic;
mod props;
mod style;

use components::fight_row::{FightRow, FightRowProps};
use components::move_transition::{MoveTransition, MoveTransitionProps};
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRow;
use dioxus::prelude::*;
use logic::UnresolvedRowModel;
pub use props::UnresolvedRowProps;
use style::CLASS;
use tw_macro::assert_component;

/// One unresolved ability card: the Stuck badge over the stuck ability, over the cell it
/// is stuck on. It owns the orc-accented card surface directly and hands each region to
/// its own child component.
#[component]
pub fn UnresolvedRow(props: UnresolvedRowProps) -> Element {
    let model = UnresolvedRowModel::from(&props);
    let reason_row = model.reason_row;
    let name_plate = model.name_plate;
    let ability = model.ability;
    let placements = model.placements;
    let fight_row = FightRowProps {
        name_plate,
        ability,
    };
    let move_transition = MoveTransitionProps { placements };
    rsx! {
        div {
            class: CLASS,
            MoveReasonRow { ..reason_row }
            FightRow { ..fight_row }
            MoveTransition { ..move_transition }
        }
    }
}

assert_component!(UnresolvedRow);
