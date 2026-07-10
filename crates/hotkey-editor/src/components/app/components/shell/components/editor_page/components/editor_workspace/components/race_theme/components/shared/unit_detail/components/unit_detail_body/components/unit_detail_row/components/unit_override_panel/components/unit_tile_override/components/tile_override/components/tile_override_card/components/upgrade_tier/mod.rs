pub mod components;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;

use crate::components::app::components::shell::components::shared::icons::{
    ICON_TIER_NEXT, ICON_TIER_PREV,
};
use components::tile_override_tier_button::TileOverrideTierButton;
use components::tile_override_tier_label::TileOverrideTierLabel;
use hooks::{UpgradeTierModel, use_upgrade_tier};
use style::CLASS;
use tw_macro::assert_component;

use props::UpgradeTierProps;

/// Tier-cycling footer for multi-level abilities: a prev/next arrow around a
/// "Level N of M" caption.
#[component]
pub fn UpgradeTier(props: UpgradeTierProps) -> Element {
    if props.total_tier_count <= 1 {
        return rsx! {};
    }
    let UpgradeTierModel {
        on_prev,
        on_next,
        tier_label_text,
    } = use_upgrade_tier(&props);
    rsx! {
        div {
            class: CLASS,
            TileOverrideTierButton {
                aria_label: "Previous level",
                icon: ICON_TIER_PREV,
                on_click: on_prev,
            }
            TileOverrideTierLabel { text: tier_label_text }
            TileOverrideTierButton {
                aria_label: "Next level",
                icon: ICON_TIER_NEXT,
                on_click: on_next,
            }
        }
    }
}

assert_component!(UpgradeTier);
