mod logic;
mod props;
mod view;

pub use view::AnchorColumnView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIcon;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlate;
use dioxus::prelude::*;
use logic::AnchorColumnModel;
use props::AnchorColumnProps;
use style::CLASS;
use tw_macro::assert_component;

/// The rival column of a move card, shown only when the move has an anchor
/// (Fight/Swap): the rival's name plate over its icon. Renders nothing otherwise.
#[component]
pub fn AnchorColumn(props: AnchorColumnProps) -> Element {
    let move_view = props.move_view;
    let Some(model) = AnchorColumnModel::for_move(&move_view) else {
        return rsx! {};
    };
    let AnchorColumnModel {
        name,
        object_id,
        icon_url,
        carrier_count,
        is_winner,
        disabled,
        inspected,
    } = model;
    let plate_name = name.clone();
    rsx! {
        div {
            class: CLASS,
            FightNamePlate {
                name: plate_name,
                object_id,
            }
            AbilityIcon {
                name,
                icon_url,
                carrier_count,
                is_winner,
                disabled,
                inspected,
            }
        }
    }
}

assert_component!(AnchorColumn);
