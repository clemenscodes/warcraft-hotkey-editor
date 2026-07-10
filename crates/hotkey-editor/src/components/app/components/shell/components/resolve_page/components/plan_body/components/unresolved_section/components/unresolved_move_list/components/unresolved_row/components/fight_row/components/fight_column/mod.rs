mod props;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::{AbilityIcon, AbilityIconProps};
use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::fight_name_plate::{FightNamePlate, FightNamePlateProps};
use dioxus::prelude::*;
pub use props::FightColumnProps;
use style::CLASS;
use tw_macro::assert_component;

/// The stuck ability's column: its name plate stacked over its ability icon.
#[component]
pub fn FightColumn(props: FightColumnProps) -> Element {
    let name_plate = FightNamePlateProps::from(&props);
    let ability = AbilityIconProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            FightNamePlate { ..name_plate }
            AbilityIcon { ..ability }
        }
    }
}

assert_component!(FightColumn);
