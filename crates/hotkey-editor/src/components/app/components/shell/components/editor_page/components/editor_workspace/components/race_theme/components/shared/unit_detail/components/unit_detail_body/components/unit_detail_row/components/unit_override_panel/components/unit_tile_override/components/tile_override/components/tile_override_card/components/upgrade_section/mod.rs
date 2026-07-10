pub mod components;
mod props;
mod style;

use components::upgrade_section_header::{UpgradeSectionHeader, UpgradeSectionHeaderProps};
use dioxus::prelude::*;
pub use props::UpgradeSectionProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(UpgradeSection);

/// The upgraded-form block of a tile override: it owns its own block directly and
/// delegates the header row (label, position button, hotkey cell) to its child.
/// Renders nothing when there is no upgrade to show.
#[component]
pub fn UpgradeSection(props: UpgradeSectionProps) -> Element {
    if !props.show {
        return rsx! {};
    }
    let header = UpgradeSectionHeaderProps::from(&props);
    rsx! {
        div {
            class: CLASS,
            UpgradeSectionHeader { ..header }
        }
    }
}
