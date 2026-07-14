pub mod components;
mod model;
mod presentation;
mod view;

pub use view::FilledHotkeyUnitDetailView;
mod style;

use components::hotkey_conflict_grid::HotkeyConflictGrid;
use components::hotkey_detail_header::HotkeyDetailHeader;
use dioxus::prelude::*;
use model::FilledHotkeyUnitDetailModel;
use presentation::FilledHotkeyUnitDetailPresentation;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn FilledHotkeyUnitDetail(props: FilledHotkeyUnitDetailModel) -> Element {
    let FilledHotkeyUnitDetailPresentation {
        unit,
        count,
        unit_id,
        conflicts,
    } = FilledHotkeyUnitDetailPresentation::from(&props);
    rsx! {
        div {
            class: CLASS,
            HotkeyDetailHeader {
                unit,
                count,
            }
            HotkeyConflictGrid {
                conflicts,
                unit_id,
            }
        }
    }
}

assert_component!(FilledHotkeyUnitDetail);
