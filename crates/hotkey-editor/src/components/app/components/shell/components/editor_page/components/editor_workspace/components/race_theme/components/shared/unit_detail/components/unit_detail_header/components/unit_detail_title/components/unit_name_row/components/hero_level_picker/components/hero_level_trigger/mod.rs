pub mod components;
mod props;

use components::closed_hero_level_trigger::{ClosedHeroLevelTrigger, ClosedHeroLevelTriggerProps};
use components::open_hero_level_trigger::{OpenHeroLevelTrigger, OpenHeroLevelTriggerProps};
use dioxus::prelude::*;
pub use props::HeroLevelTriggerProps;
use tw_macro::assert_component;
assert_component!(HeroLevelTrigger);

/// The hero-level dropdown trigger button. A pure dispatcher: from the menu's open
/// flag it renders the open look (`OpenHeroLevelTrigger`, an accented border and glow
/// with a flipped caret) xor the resting look (`ClosedHeroLevelTrigger`). No class of
/// its own — each look owns its button root, and both share the label, number and
/// caret leaves.
#[component]
pub fn HeroLevelTrigger(props: HeroLevelTriggerProps) -> Element {
    if props.is_open {
        let open = OpenHeroLevelTriggerProps::from(&props);
        rsx! {
            OpenHeroLevelTrigger { ..open }
        }
    } else {
        let closed = ClosedHeroLevelTriggerProps::from(&props);
        rsx! {
            ClosedHeroLevelTrigger { ..closed }
        }
    }
}
