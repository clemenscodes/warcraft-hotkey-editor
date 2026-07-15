mod presentation;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_command_grids::UnitCommandGrids;
use dioxus::prelude::*;
use presentation::{MobileCommandCard, MobileEditorView, use_mobile_editor};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn MobileEditor() -> Element {
    let card = match use_mobile_editor() {
        MobileEditorView::Empty => {
            return rsx! {
                section {
                    class: CLASS,
                    aria_label: "Mobile editor",
                }
            };
        }
        MobileEditorView::Loaded(card) => card,
    };
    let MobileCommandCard {
        unit_id,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
    } = card;
    rsx! {
        section {
            class: CLASS,
            aria_label: "Mobile editor",
            UnitCommandGrids {
                unit_id,
                command_card_slots,
                build_menu_slots,
                uprooted_menu_slots,
                research_menu_slots,
            }
        }
    }
}

assert_component!(MobileEditor);
