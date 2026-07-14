mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::UnitDetail;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_list::UnitList;
use dioxus::prelude::*;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UndeadRaceTheme() -> Element {
    rsx! {
        div {
            class: CLASS,
            UnitList {



            }
            UnitDetail {



            }
        }
    }
}

assert_component!(UndeadRaceTheme);
