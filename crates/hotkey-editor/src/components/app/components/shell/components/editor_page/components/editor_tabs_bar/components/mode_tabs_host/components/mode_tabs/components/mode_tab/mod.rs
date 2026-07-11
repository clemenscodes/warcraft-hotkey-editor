mod model;
mod view;

pub use view::ModeTabView;

use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::ToggleButton;
use dioxus::prelude::*;
use tw_macro::assert_component;
use model::ModeTabModel;

/// One mode button (Melee or Campaign). It is the shared [`ToggleButton`] configured
/// for the mode toggle: a gold pill with no tooltip, carrying the keyboard handler
/// that moves focus onto the race tabs.
#[component]
pub fn ModeTab(props: ModeTabModel) -> Element {
    let label = props.label;
    let active = props.active;
    let onclick = props.onclick;
    let onkeydown = props.onkeydown;
    rsx! {
        ToggleButton { label, active, onclick, onkeydown }
    }
}

assert_component!(ModeTab);
