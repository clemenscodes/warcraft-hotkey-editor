mod props;

use crate::components::app::components::shell::components::editor_page::components::shared::toggle_button::{
    ToggleButton, ToggleButtonProps,
};
use dioxus::prelude::*;
use tw_macro::assert_component;
pub use props::ModeTabProps;

/// One mode button (Melee or Campaign). It is the shared [`ToggleButton`] configured
/// for the mode toggle: a gold pill with no tooltip, carrying the keyboard handler
/// that moves focus onto the race tabs.
#[component]
pub fn ModeTab(props: ModeTabProps) -> Element {
    let button = ToggleButtonProps::from(&props);
    rsx! {
        ToggleButton { ..button }
    }
}

assert_component!(ModeTab);
