mod model;
mod view;

pub use view::UnitDetailDialogBodyView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_description::UnitDescription;
use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_stats_panel::UnitStatsPanel;
use dioxus::prelude::*;
use model::UnitDetailDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

use super::super::super::shared::pager_card_portrait::PagerCardPortrait;

#[component]
pub fn UnitDetailDialogBody(props: UnitDetailDialogBodyModel) -> Element {
    let portrait_url = props.portrait_url;
    let description_text = props.description_text;
    let combat = props.combat;
    let hero_attributes = props.hero_attributes;
    let evasion = props.evasion;
    rsx! {
        div {
            class: CLASS,
            PagerCardPortrait {
                src: portrait_url,
            }
            UnitDescription {
                text: description_text,
            }
            UnitStatsPanel {
                combat,
                hero_attributes,
                evasion,
            }
        }
    }
}

assert_component!(UnitDetailDialogBody);
