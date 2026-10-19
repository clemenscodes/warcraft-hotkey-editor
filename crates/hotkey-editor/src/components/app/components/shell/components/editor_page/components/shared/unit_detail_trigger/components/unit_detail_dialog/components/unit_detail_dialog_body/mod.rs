pub mod components;
mod model;
mod view;

pub use view::UnitDetailDialogBodyView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::shared::unit_detail_trigger::components::unit_detail_dialog::components::unit_detail_dialog_body::components::unit_stats_panel::UnitStatsPanel;
use components::unit_detail_dialog_intro::UnitDetailDialogIntro;
use dioxus::prelude::*;
use model::UnitDetailDialogBodyModel;
use style::CLASS;
use tw_macro::assert_component;

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
            UnitDetailDialogIntro {
                portrait_url,
                description_text,
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
