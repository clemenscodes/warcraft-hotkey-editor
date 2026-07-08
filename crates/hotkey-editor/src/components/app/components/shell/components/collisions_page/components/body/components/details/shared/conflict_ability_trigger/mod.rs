mod props;
mod style;

use super::conflict_ability_icon::ConflictAbilityIcon;
use dioxus::prelude::*;
pub use props::ConflictAbilityTriggerProps;
use style::CLASS;
use tw_macro::assert_component;
assert_component!(ConflictAbilityTrigger);

/// The transparent icon button wrapping a conflict ability's icon: the click target
/// that deep-links to the carrying unit. Its `.conflict-ability-trigger` identity class
/// is coupled to the e2e suite.
#[component]
pub fn ConflictAbilityTrigger(props: ConflictAbilityTriggerProps) -> Element {
    let onclick = props.onclick;
    let icon = props.icon;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            ConflictAbilityIcon { ..icon }
        }
    }
}
