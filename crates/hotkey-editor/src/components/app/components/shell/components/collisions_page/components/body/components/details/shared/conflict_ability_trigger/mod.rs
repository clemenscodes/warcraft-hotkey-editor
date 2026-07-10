pub mod components;
mod props;
mod style;

use components::conflict_ability_icon::ConflictAbilityIcon;
use dioxus::prelude::*;
use props::ConflictAbilityTriggerProps;
use style::CLASS;
use tw_macro::assert_component;

/// The transparent icon button wrapping a conflict ability's icon: the click target
/// that deep-links to the carrying unit. Its `.conflict-ability-trigger` identity class
/// is coupled to the e2e suite.
#[component]
pub fn ConflictAbilityTrigger(props: ConflictAbilityTriggerProps) -> Element {
    let onclick = props.onclick;
    let icon_src = props.icon_src;
    let icon_alt = props.icon_alt;
    rsx! {
        button {
            class: CLASS,
            r#type: "button",
            onclick,
            ConflictAbilityIcon {
                src: icon_src,
                alt: icon_alt,
            }
        }
    }
}

assert_component!(ConflictAbilityTrigger);
