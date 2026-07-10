pub mod components;
mod props;
mod style;

use components::upgrade_section_header::UpgradeSectionHeader;
use dioxus::prelude::*;
use props::UpgradeSectionProps;
use style::CLASS;
use tw_macro::assert_component;

/// The upgraded-form block of a tile override: it owns its own block directly and
/// delegates the header row (label, position button, hotkey cell) to its child.
/// Renders nothing when there is no upgrade to show.
#[component]
pub fn UpgradeSection(props: UpgradeSectionProps) -> Element {
    if !props.show {
        return rsx! {};
    }
    let UpgradeSectionProps {
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
