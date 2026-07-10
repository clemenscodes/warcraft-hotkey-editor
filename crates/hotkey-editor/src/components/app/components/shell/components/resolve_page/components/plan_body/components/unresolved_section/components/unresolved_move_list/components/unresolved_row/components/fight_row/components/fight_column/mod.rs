mod logic;
mod props;
mod view;

pub use view::FightColumnView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIcon;
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::FightNamePlate;
use dioxus::prelude::*;
use logic::FightColumnModel;
use props::FightColumnProps;
use style::CLASS;
use tw_macro::assert_component;

/// The stuck ability's column: its name plate stacked over its ability icon.
#[component]
pub fn FightColumn(props: FightColumnProps) -> Element {
    let unresolved_view = props.unresolved_view;
    let model = FightColumnModel::from(&unresolved_view);
    let FightColumnModel {
        name,
        object_id,
        icon_url,
        carrier_count,
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
                is_winner: false,
                disabled,
                inspected,
            }
        }
    }
}

assert_component!(FightColumn);
