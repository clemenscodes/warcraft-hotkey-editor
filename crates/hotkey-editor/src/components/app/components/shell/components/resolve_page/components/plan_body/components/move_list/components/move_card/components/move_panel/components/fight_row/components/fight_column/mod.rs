pub mod components;
mod model;
mod presentation;
mod view;

pub use view::FightColumnView;
mod style;

use crate::components::app::components::shell::components::resolve_page::components::plan_body::components::ability_icon::AbilityIcon;
use components::fight_name_button::FightNameButton;
use dioxus::prelude::*;
use presentation::{FightColumnPresentation, use_fight_column};
use model::FightColumnModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FightColumn(props: FightColumnModel) -> Element {
    let FightColumnPresentation {
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
