pub mod components;
mod model;
mod view;

pub use view::HeroLevelTriggerView;

use components::closed_hero_level_trigger::ClosedHeroLevelTrigger;
use components::open_hero_level_trigger::OpenHeroLevelTrigger;
use dioxus::prelude::*;
use model::HeroLevelTriggerModel;
use tw_macro::assert_component;

#[component]
pub fn HeroLevelTrigger(props: HeroLevelTriggerModel) -> Element {
    let number = props.number;
    let onclick = props.onclick;
    if props.is_open {
        rsx! {
            OpenHeroLevelTrigger {
                number,
                onclick,
            }
        }
    } else {
        rsx! {
            ClosedHeroLevelTrigger {
                number,
                onclick,
            }
        }
    }
}

assert_component!(HeroLevelTrigger);
