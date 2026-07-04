pub mod components;
mod hooks;
mod props;
mod style;

use dioxus::prelude::*;

use crate::assert_component;
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
    let model = use_upgrade_tier(&props);
    rsx! {
        div {
            class: CLASS,
            TileOverrideTierButton { ..model.prev_button }
            TileOverrideTierLabel { ..model.label }
            TileOverrideTierButton { ..model.next_button }
        }
    }
}
