pub mod components;
mod hooks;
mod props;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIcon;
use components::fight_name_button::FightNameButton;
use dioxus::prelude::*;
use hooks::{FightColumnModel, use_fight_column};
use props::FightColumnProps;
use style::CLASS;
use tw_macro::assert_component;

/// The mover's fighter column: its name button over its ability icon.
#[component]
pub fn FightColumn(props: FightColumnProps) -> Element {
    let FightColumnModel {
        name,
        object_id,
        has_unit,
        onclick,
        icon_url,
        carrier_count,
        disabled,
        inspected,
    } = use_fight_column(&props);
    let button_name = name.clone();
    rsx! {
        div {
            class: CLASS,
            FightNameButton {
                name: button_name,
                object_id,
                has_unit,
                onclick,
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
