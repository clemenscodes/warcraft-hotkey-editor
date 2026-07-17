mod model;
mod style;
mod view;

pub use view::NightelfRaceChipThemeView;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::search_dialog::components::search_dialog_body::components::search_dialog_filters::components::race_chip::RaceChip;
use dioxus::prelude::*;
use model::NightelfRaceChipThemeModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn NightelfRaceChipTheme(props: NightelfRaceChipThemeModel) -> Element {
    let label = props.label;
    let active = props.active;
    let on_pick = props.on_pick;
    rsx! {
        div {
            class: CLASS,
            RaceChip {
                label,
                active,
                on_pick,
            }
        }
    }
}

assert_component!(NightelfRaceChipTheme);
