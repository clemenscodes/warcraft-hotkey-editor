pub mod components;
mod model;
mod style;
mod view;

pub use view::TierStepperView;

use crate::components::app::components::shell::components::editor_page::components::shared::hotkey_override_section::components::hotkey_override_slot::components::hotkey_override::components::hotkey_override_card::components::shared::card_icon_button_host::CardIconButtonHost;
use crate::components::app::components::shell::components::shared::icons::{
    ICON_CHEVRON_LEFT, ICON_CHEVRON_RIGHT,
};
use components::tier_label::TierLabel;
use dioxus::prelude::*;
use model::TierStepperModel;
use style::CLASS;
use tw_macro::assert_component;

#[component]
pub fn TierStepper(props: TierStepperModel) -> Element {
    let label = props.label;
    let on_previous = props.on_previous;
    let on_next = props.on_next;
    rsx! {
        div {
            class: CLASS,
            CardIconButtonHost {
                icon: ICON_CHEVRON_LEFT,
                title: "Previous level",
                aria_label: "Previous level",
                on_click: on_previous,
            }
            TierLabel {
                text: label,
            }
            CardIconButtonHost {
                icon: ICON_CHEVRON_RIGHT,
                title: "Next level",
                aria_label: "Next level",
                on_click: on_next,
            }
        }
    }
}

assert_component!(TierStepper);
