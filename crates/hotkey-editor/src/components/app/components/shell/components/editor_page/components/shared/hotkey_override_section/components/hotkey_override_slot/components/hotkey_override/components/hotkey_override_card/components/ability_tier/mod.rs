pub mod components;
mod model;
mod presentation;
mod view;

pub use view::AbilityTierView;
mod style;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::shared::icons::{
    ICON_TIER_NEXT, ICON_TIER_PREV,
};
use components::ability_tier_button::AbilityTierButton;
use components::ability_tier_label::AbilityTierLabel;
use presentation::{AbilityTierPresentation, use_ability_tier};
use style::CLASS;
use tw_macro::assert_component;

use model::AbilityTierModel;

#[component]
pub fn AbilityTier(props: AbilityTierModel) -> Element {
    if props.total_tier_count <= 1 {
        return rsx! {};
    }
    let AbilityTierPresentation {
        on_prev,
        on_next,
        tier_label_text,
    } = use_ability_tier(&props);
    rsx! {
        div {
            class: CLASS,
            AbilityTierButton {
                aria_label: "Previous level",
                icon: ICON_TIER_PREV,
                on_click: on_prev,
            }
            AbilityTierLabel {
                text: tier_label_text,
            }
            AbilityTierButton {
                aria_label: "Next level",
                icon: ICON_TIER_NEXT,
                on_click: on_next,
            }
        }
    }
}

assert_component!(AbilityTier);
