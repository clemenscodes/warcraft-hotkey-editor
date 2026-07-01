pub mod components;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
use crate::components::shared::icons::{ICON_TIER_NEXT, ICON_TIER_PREV};
use components::tile_override_tier_button::TileOverrideTierButton;
use components::tile_override_tier_label::TileOverrideTierLabel;
use hooks::use_upgrade_tier;
use style::CLASS;

pub use props::UpgradeTierProps;

assert_component!(UpgradeTier);

/// Tier-cycling footer for multi-level abilities: a prev/next arrow around a
/// "Level N of M" caption.
#[component]
pub fn UpgradeTier(props: UpgradeTierProps) -> Element {
    if props.total_tier_count <= 1 {
        return rsx! {};
    }
    let tier_label_text = props.tier_label_text.clone();
    let model = use_upgrade_tier(&props);
    rsx! {
        div { class: CLASS,
            TileOverrideTierButton {
                aria_label: "Previous level",
                icon: ICON_TIER_PREV,
                on_click: model.on_prev,
            }
            TileOverrideTierLabel { text: tier_label_text }
            TileOverrideTierButton {
                aria_label: "Next level",
                icon: ICON_TIER_NEXT,
                on_click: model.on_next,
            }
        }
    }
}
