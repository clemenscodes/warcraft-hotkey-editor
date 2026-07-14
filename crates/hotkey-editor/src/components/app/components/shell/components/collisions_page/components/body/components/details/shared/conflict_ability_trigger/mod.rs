pub mod components;
mod model;
mod view;

pub use view::ConflictAbilityTriggerView;
mod style;

use components::conflict_ability_icon::ConflictAbilityIcon;
use dioxus::prelude::*;
use model::ConflictAbilityTriggerModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn ConflictAbilityTrigger(props: ConflictAbilityTriggerModel) -> Element {
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
