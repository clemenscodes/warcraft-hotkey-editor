mod model;
mod style;
mod view;

pub use view::HumanPagerCardThemeView;

use crate::components::app::components::shell::components::editor_page::components::mobile_editor::components::pager_card_host::components::pager_card_race_theme::components::shared::pager_card::PagerCard;
use dioxus::prelude::*;
use model::HumanPagerCardThemeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HumanPagerCardTheme(props: HumanPagerCardThemeModel) -> Element {
    let unit_id = props.unit_id;
    rsx! {
        div {
            class: CLASS,
            PagerCard {
                unit_id,
            }
        }
    }
}

assert_component!(HumanPagerCardTheme);
