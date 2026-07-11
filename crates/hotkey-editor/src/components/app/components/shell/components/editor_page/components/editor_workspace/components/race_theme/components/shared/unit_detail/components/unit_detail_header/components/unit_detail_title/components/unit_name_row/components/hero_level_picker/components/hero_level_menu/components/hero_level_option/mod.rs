pub mod components;
mod model;
mod presentation;
mod view;

pub use view::HeroLevelOptionView;

use components::active_hero_level_option::ActiveHeroLevelOption;
use components::idle_hero_level_option::IdleHeroLevelOption;
use dioxus::prelude::*;
use model::HeroLevelOptionModel;
use presentation::use_hero_level_option;
use tw_macro::assert_component;

/// One selectable hero level in the dropdown. A pure dispatcher: from whether it is the
/// current level (read from editor context) it renders `ActiveHeroLevelOption` xor
/// `IdleHeroLevelOption`. Each owns its `<button>` and its own look; there is no
/// `data-active`.
#[component]
pub fn HeroLevelOption(props: HeroLevelOptionModel) -> Element {
    let presentation = use_hero_level_option(&props);
    let label = presentation.label().to_owned();
    let onclick = presentation.onclick();
    match presentation.is_active() {
        true => rsx! {
            ActiveHeroLevelOption { label, onclick }
        },
        false => rsx! {
            IdleHeroLevelOption { label, onclick }
        },
    }
}

assert_component!(HeroLevelOption);
