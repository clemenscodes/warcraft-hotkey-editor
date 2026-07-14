pub mod components;
mod model;
mod presentation;
mod view;

pub use view::HeroLevelMenuView;
mod style;

use components::hero_level_option::HeroLevelOption;
use dioxus::prelude::*;
use model::HeroLevelMenuModel;
use presentation::hero_level_options;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HeroLevelMenu(props: HeroLevelMenuModel) -> Element {
    let options = hero_level_options(&props);
    rsx! {
        div {
            class: CLASS,
            for option in options {
                HeroLevelOption {
                    key: "{option.level_index}",
                    level_index: option.level_index,
                    level_picker_open: option.level_picker_open,
                }
            }
        }
    }
}

assert_component!(HeroLevelMenu);
