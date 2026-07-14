pub mod components;
mod model;
pub mod presentation;
mod state;
mod style;

use components::unit_description::UnitDescription;
use components::unit_detail_body::UnitDetailBody;
use components::unit_detail_empty::UnitDetailEmpty;
use components::unit_detail_header::UnitDetailHeader;
use components::unit_stats_panel::UnitStatsPanel;
use dioxus::prelude::*;
use presentation::use_unit_detail_panel;
use state::{UnitDetailModel, UnitDetailView};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UnitDetail() -> Element {
    let model = match use_unit_detail_panel() {
        UnitDetailView::Loaded(model) => *model,
        UnitDetailView::Empty(message) => {
            return rsx! {
                UnitDetailEmpty {
                    message,
                }
            };
        }
    };
    let UnitDetailModel {
        unit_name,
        unit_id,
        portrait_url,
        has_hero_attributes,
        description_text,
        combat,
        hero_attributes,
        evasion,
        grid_slots,
        override_target,
    } = model;
    rsx! {
        section {
            class: CLASS,
            UnitDetailHeader {
                unit_name,
                unit_id,
                portrait_url,
                has_hero_attributes,
            }
            UnitDescription {
                text: description_text,
            }
            UnitStatsPanel {
                combat,
                hero_attributes,
                evasion,
            }
            UnitDetailBody {
                grid_slots,
                override_target,
            }
        }
    }
}

assert_component!(UnitDetail);
