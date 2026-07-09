pub mod components;
mod logic;
mod props;

use components::active_hero_level_option::{ActiveHeroLevelOption, ActiveHeroLevelOptionProps};
use components::idle_hero_level_option::{IdleHeroLevelOption, IdleHeroLevelOptionProps};
use dioxus::prelude::*;
use logic::HeroLevelOptionPresentation;
pub use props::HeroLevelOptionProps;
use tw_macro::assert_component;
assert_component!(HeroLevelOption);

/// One selectable hero level in the dropdown. A pure dispatcher: from whether it is the
/// current level it renders `ActiveHeroLevelOption` xor `IdleHeroLevelOption`. Each owns
/// its `<button>` and its own look; there is no `data-active`.
#[component]
pub fn HeroLevelOption(props: HeroLevelOptionProps) -> Element {
    let presentation = HeroLevelOptionPresentation::from(&props);
    match presentation.is_active() {
        true => {
            let option = ActiveHeroLevelOptionProps::from(&presentation);
            rsx! {
                ActiveHeroLevelOption { ..option }
            }
        }
        false => {
            let option = IdleHeroLevelOptionProps::from(&presentation);
            rsx! {
                IdleHeroLevelOption { ..option }
            }
        }
    }
}
