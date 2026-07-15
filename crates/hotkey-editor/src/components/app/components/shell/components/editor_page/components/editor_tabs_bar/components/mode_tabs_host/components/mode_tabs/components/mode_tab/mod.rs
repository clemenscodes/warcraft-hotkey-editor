mod model;
mod view;

pub use view::ModeTabView;
pub mod components;

use components::active_mode_segment::ActiveModeSegment;
use components::idle_mode_segment::IdleModeSegment;
use dioxus::prelude::*;
use model::ModeTabModel;
use tw_macro::assert_component;

#[component]
pub fn ModeTab(props: ModeTabModel) -> Element {
    let label = props.label;
    let active = props.active;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    match active {
        true => rsx! {
            ActiveModeSegment {
                label,
                onclick,
                onkeydown,
            }
        },
        false => rsx! {
            IdleModeSegment {
                label,
                onclick,
                onkeydown,
            }
        },
    }
}

assert_component!(ModeTab);
