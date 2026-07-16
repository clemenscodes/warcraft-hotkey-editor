pub mod components;
mod model;
mod presentation;
mod view;

pub use view::PagerCardView;
mod style;

use crate::components::app::components::shell::components::editor_page::components::editor_workspace::components::race_theme::components::shared::unit_detail::components::unit_detail_body::components::unit_detail_row::components::unit_command_grids::UnitCommandGrids;
use components::pager_card_header::PagerCardHeader;
use dioxus::prelude::*;
use model::PagerCardModel;
use presentation::{PagerCardPresentation, use_pager_card};
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn PagerCard(props: PagerCardModel) -> Element {
    let presentation = use_pager_card(&props);
    let PagerCardPresentation {
        icon_url,
        name,
        unit_id,
        command_card_slots,
        build_menu_slots,
        uprooted_menu_slots,
        research_menu_slots,
    } = presentation;
    rsx! {
        div {
            class: CLASS,
            PagerCardHeader {
                icon_url,
                name,
                unit_id,
            }
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

assert_component!(PagerCard);
