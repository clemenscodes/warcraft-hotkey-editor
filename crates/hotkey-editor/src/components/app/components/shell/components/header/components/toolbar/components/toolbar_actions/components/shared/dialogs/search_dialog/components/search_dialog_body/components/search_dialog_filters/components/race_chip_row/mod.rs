mod data;
mod presentation;
mod style;

use crate::components::app::components::shell::components::header::components::toolbar::components::toolbar_actions::components::shared::dialogs::search_dialog::components::search_dialog_body::components::search_dialog_filters::components::race_chip_theme::RaceChipTheme;
use dioxus::prelude::*;
use presentation::use_race_chip_row;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn RaceChipRow() -> Element {
    let races = use_race_chip_row();
    rsx! {
        div {
            class: CLASS,
            role: "group",
            aria_label: "Filter by race",
            for choice in races {
                RaceChipTheme {
                    key: "{choice.race.slug()}",
                    race: choice.race,
                    label: data::label(choice.race),
                    active: choice.is_active,
                    on_pick: choice.on_pick,
                }
            }
        }
    }
}

assert_component!(RaceChipRow);
