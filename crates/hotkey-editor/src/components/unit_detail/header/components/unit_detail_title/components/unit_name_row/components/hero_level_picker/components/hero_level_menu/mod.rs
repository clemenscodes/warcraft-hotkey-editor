pub mod components;
mod logic;
mod props;
mod style;

use crate::assert_component;
use components::hero_level_option::HeroLevelOption;
use dioxus::prelude::*;
use logic::hero_level_options;
pub use props::HeroLevelMenuProps;
use style::CLASS;
assert_component!(HeroLevelMenu);

/// The dropdown list of selectable hero levels.
#[component]
pub fn HeroLevelMenu(props: HeroLevelMenuProps) -> Element {
    let options = hero_level_options(&props);
    rsx! {
        div {
            class: CLASS,
            for option in options {
                HeroLevelOption { key: "{option.level_index}", ..option }
            }
        }
    }
}
