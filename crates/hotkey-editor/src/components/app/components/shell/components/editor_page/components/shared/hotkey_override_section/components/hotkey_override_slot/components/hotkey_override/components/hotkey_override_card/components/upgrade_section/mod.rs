pub mod components;
mod model;
mod view;

pub use view::UpgradeSectionView;
mod style;

use components::upgrade_section_header::UpgradeSectionHeader;
use dioxus::prelude::*;
use model::UpgradeSectionModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn UpgradeSection(props: UpgradeSectionModel) -> Element {
    if !props.show {
        return rsx! {};
    }
    let UpgradeSectionModel {
        show: _,
        upgrade_hotkey_label,
        upgrade_is_editing,
        upgrade_hotkey_is_special,
        on_position_click,
        on_hotkey_activate,
    } = props;
    rsx! {
        div {
            class: CLASS,
            UpgradeSectionHeader {
                hotkey_label: upgrade_hotkey_label,
                is_editing: upgrade_is_editing,
                is_special: upgrade_hotkey_is_special,
                on_position_click,
                on_hotkey_activate,
            }
        }
    }
}

assert_component!(UpgradeSection);
