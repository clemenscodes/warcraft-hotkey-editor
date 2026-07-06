pub mod components;
mod props;
mod style;

use dioxus::prelude::*;

use components::ability_description::AbilityDescription;
use components::alt_state_section::AltStateSection;
use components::tile_override_header::TileOverrideHeader;
use components::upgrade_section::UpgradeSection;
use components::upgrade_tier::UpgradeTier;
use style::CLASS;
use tw_macro::assert_component;

pub use props::TileOverrideCardProps;

assert_component!(TileOverrideCard);

/// The gold-edged card holding the override panel's header and ability sections.
#[component]
pub fn TileOverrideCard(props: TileOverrideCardProps) -> Element {
    rsx! {
        div { class: CLASS,
            TileOverrideHeader { ..props.header }
            AbilityDescription { ..props.description }
            AltStateSection { ..props.alt_state }
            UpgradeSection { ..props.upgrade }
            UpgradeTier { ..props.tier }
        }
    }
}
