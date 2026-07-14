pub mod components;
mod data;
mod model;
mod view;

pub use view::HotkeyOverrideSectionView;
mod style;

use crate::components::app::components::shell::components::shared::grid_heading::GridHeading;
use components::hotkey_override_slot::HotkeyOverrideSlot;
use data::HEADING;
use dioxus::prelude::*;
use model::HotkeyOverrideSectionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn HotkeyOverrideSection(props: HotkeyOverrideSectionModel) -> Element {
    let override_target = props.override_target;
    let detail = override_target.detail;
    let active_container_slots = override_target.active_container_slots;
    rsx! {
        aside {
            class: CLASS,
            GridHeading {
                heading: HEADING,
            }
            HotkeyOverrideSlot {
                detail,
                active_container_slots,
            }
        }
    }
}

assert_component!(HotkeyOverrideSection);
