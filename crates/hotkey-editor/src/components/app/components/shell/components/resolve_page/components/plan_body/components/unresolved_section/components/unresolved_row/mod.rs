mod data;
mod logic;
mod props;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIcon;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlate;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::mini_grid::MiniGrid;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::move_reason_row::MoveReasonRow;
use dioxus::prelude::*;
use logic::UnresolvedRowModel;
pub use props::UnresolvedRowProps;
use style::{CLASS, FIGHT_COLUMN, FIGHT_ROW, MOVE_TRANSITION, PANEL};
use tw_macro::assert_component;
assert_component!(UnresolvedRow);

/// One unresolved ability card: the Stuck badge, the ability, and the cell it is stuck
/// on. It owns its own (orc-accented) card surface and layout directly.
#[component]
pub fn UnresolvedRow(props: UnresolvedRowProps) -> Element {
    let model = UnresolvedRowModel::from(&props);
    rsx! {
        div {
            class: CLASS,
            div {
                class: PANEL,
                MoveReasonRow { ..model.reason_row }
                div {
                    class: FIGHT_ROW,
                    div {
                        class: FIGHT_COLUMN,
                        FightNamePlate { ..model.name_plate }
                        AbilityIcon { ..model.ability }
                    }
                }
                div {
                    class: MOVE_TRANSITION,
                    MiniGrid { placements: model.placements }
                }
            }
        }
    }
}
