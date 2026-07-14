pub mod components;
mod model;
mod view;

pub use view::HeroLevelTriggerView;

use components::closed_hero_level_trigger::ClosedHeroLevelTrigger;
use components::open_hero_level_trigger::OpenHeroLevelTrigger;
use dioxus::prelude::*;
use model::HeroLevelTriggerModel;
use tw_macro::assert_component;

/// The hero-level dropdown trigger button. A pure dispatcher: from the menu's open
/// flag it renders the open look (`OpenHeroLevelTrigger`, an accented border and glow
/// with a flipped caret) xor the resting look (`ClosedHeroLevelTrigger`). No class of
/// its own — each look owns its button root, and both share the label, number and
/// caret leaves.
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
